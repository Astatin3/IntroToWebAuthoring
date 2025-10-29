use std::collections::{HashMap, HashSet};

use crate::{
    log,
    render::Renderer,
    views::{LayoutView, View},
};

pub enum Constraint {
    Left(f32, usize),
    Right(f32, usize),
    Top(f32, usize),
    Bottom(f32, usize),

    LeftParent(f32),
    RightParent(f32),
    TopParent(f32),
    BottomParent(f32),
}

pub trait ConstraintView: LayoutView {
    fn set_constraints(
        &mut self,
        top: Option<Constraint>,
        left: Option<Constraint>,
        right: Option<Constraint>,
        bottom: Option<Constraint>,
    );
    fn get_constraints(
        &self,
    ) -> (
        &Option<Constraint>,
        &Option<Constraint>,
        &Option<Constraint>,
        &Option<Constraint>,
    );
}

pub struct ConstraintLayout {
    pub children: Vec<Box<dyn ConstraintView>>,
    child_positions: Option<Vec<Rect>>,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl ConstraintLayout {
    pub fn new(children: Vec<Box<dyn ConstraintView>>) -> Self {
        for child in &children {
            let (top, left, right, bottom) = child.get_constraints();

            assert!(
                top.is_some() || bottom.is_some(),
                "Vertically underconstrained"
            );
            assert!(
                left.is_some() || right.is_some(),
                "Horizontally underconstrained"
            );
        }

        let this = ConstraintLayout {
            children,
            child_positions: None,
        };

        this.test();

        this
    }
    pub fn test(&self) {
        let dependencies = self
            .children
            .iter()
            .map(|c| {
                let constraints = c.get_constraints();
                [constraints.0, constraints.1, constraints.2, constraints.3]
            })
            .collect::<Vec<_>>();

        let dac = topological_sort(&dependencies);

        for i in 0..dac.len() {
            log!("{:?}", dac[i])
        }
    }
}

impl View for ConstraintLayout {
    fn draw(&mut self, renderer: &mut Renderer, x: f32, y: f32, self_w: f32, self_h: f32) {
        let self_top = y;
        let self_left = x;
        let self_right = x + self_w;
        let self_bottom = y + self_h;

        let mut calculated_positions: HashMap<usize, Rect> = HashMap::new();

        let dependencies = self
            .children
            .iter()
            .map(|c| {
                let constraints = c.get_constraints();
                [constraints.0, constraints.1, constraints.2, constraints.3]
            })
            .collect::<Vec<_>>();

        let sorted = topological_sort(&dependencies);

        for idx in sorted {
            let child = &mut self.children[idx];

            let (c_top, c_left, c_right, c_bottom) = child.get_constraints();
            let (c_width, c_height) = child.bounds(self_w, self_h);

            let c_width = match c_width {
                crate::views::Bounds::MatchParent => self_w,
                crate::views::Bounds::Pixels(x) => x,
            };

            let c_height = match c_height {
                crate::views::Bounds::MatchParent => self_h,
                crate::views::Bounds::Pixels(x) => x,
            };

            let (y, height) = match (c_top, c_bottom) {
                (None, None) => unreachable!("Vertically unconstrained"),
                (Some(Constraint::TopParent(top_margin)), None) => {
                    (self_top + top_margin, c_height)
                }
                (Some(Constraint::BottomParent(top_margin)), None) => {
                    (self_top + top_margin + c_height, c_height)
                }
                (None, Some(Constraint::TopParent(top_margin))) => {
                    (self_top - top_margin, c_height)
                }
                (None, Some(Constraint::BottomParent(bottom_margin))) => {
                    (self_h - (bottom_margin + c_height), c_height)
                }
                (Some(Constraint::TopParent(_)), Some(Constraint::TopParent(_))) => unreachable!(),
                (Some(Constraint::BottomParent(_)), Some(Constraint::BottomParent(_))) => {
                    unreachable!()
                }
                (Some(Constraint::BottomParent(_)), Some(Constraint::TopParent(_))) => {
                    unreachable!()
                }
                (Some(Constraint::TopParent(_)), Some(Constraint::BottomParent(top_margin))) => {
                    (self_top + top_margin + c_height, c_height)
                }
                _ => todo!(),
            };

            assert!(height >= 0.);

            let x_align = |constraint: &Option<Constraint>| match constraint {
                Some(Constraint::LeftParent(margin)) => Some(self_left + margin),
                Some(Constraint::RightParent(margin)) => Some(self_right - margin),
                Some(Constraint::Left(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.x - margin - c_width)
                }

                Some(Constraint::Right(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.x + other.width + margin)
                }
                None => None,
                _ => unreachable!(),
            };

            let x = match (x_align(c_left), x_align(c_right)) {
                (Some(left), Some(right)) => (right + left - c_width) / 2.,
                (Some(left), None) => left,
                (None, Some(right)) => right - c_width,
                (None, None) => unreachable!("Horizontally unconstrained"),
            };

            // let (x, width) = match (c_left, c_right) {
            //     (None, None) => unreachable!("Vertically unconstrained"),
            //     (Some(Constraint::LeftParent(left_margin)), None) => {
            //         (self_top + left_margin, width)
            //     }
            //     (Some(Constraint::RightParent(right_margin)), None) => {
            //         (self_top - right_margin + width, width)
            //     }
            //     (None, Some(Constraint::LeftParent(left_margin))) => {
            //         (self_top - left_margin, width)
            //     }
            //     (None, Some(Constraint::RightParent(right_margin))) => {
            //         (self_w - (right_margin + width), width)
            //         // (self_top + top_margin + height, height)
            //     }
            //     (Some(Constraint::Left(left_margin, n)), None) => {
            //         let other = calculated_positions.get(n).unwrap();
            //         (other.x - (width + left_margin), width)
            //     }
            //     (Some(Constraint::Right(right_margin, idx)), None) => {
            //         (self_top - right_margin + width, width)
            //     }
            //     (None, Some(Constraint::Left(left_margin, idx))) => (self_top - left_margin, width),
            //     (None, Some(Constraint::Right(right_margin, idx))) => {
            //         (self_w - (right_margin + width), width)
            //         // (self_top + top_margin + height, height)
            //     }
            //     (Some(Constraint::LeftParent(_)), Some(Constraint::LeftParent(_))) => {
            //         unreachable!()
            //     }
            //     (Some(Constraint::RightParent(_)), Some(Constraint::RightParent(_))) => {
            //         unreachable!()
            //     }
            //     (Some(Constraint::RightParent(_)), Some(Constraint::LeftParent(_))) => {
            //         unreachable!()
            //     }
            //     (Some(Constraint::TopParent(_)), Some(Constraint::BottomParent(top_margin))) => {
            //         (self_top + top_margin + width, width)
            //     }
            //     _ => todo!(),
            // };

            assert!(c_width >= 0.);

            calculated_positions.insert(
                idx,
                Rect {
                    x: x,
                    y: y,
                    width: c_width,
                    height: height,
                },
            );
            child.draw(renderer, x, y, c_width, height);

            // match c_top {
            //     None => {}
            //     Some(Constraint::TopParent(top_margin)) => {

            //         // let child_top = self_top + top_margin;
            //         // let child_left = self_left;
            //         // let child_right = self_right;
            //         // let child_bottom = child_top + child.get_height();

            //         // child.draw(renderer, child_left, child_top, child_right - child_left, child_bottom - child_top);
            //     }
            //     _ => unreachable!(),
            // }

            // match

            // let child_top = child_top.unwrap_or(self_top);
            // let child_left = child_left.unwrap_or(self_left);
            // let child_right = child_right.unwrap_or(self_right);
            // let child_bottom = child_bottom.unwrap_or(self_bottom);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    Visiting,
    Visited,
}

/// Performs a topological sort on objects with constraints.
/// Returns indices in topologically sorted order (dependencies before dependents).
/// Panics if a cycle is detected.
pub fn topological_sort(objects: &Vec<[&Option<Constraint>; 4]>) -> Vec<usize> {
    let n = objects.len();
    let mut states = vec![VisitState::Unvisited; n];
    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        if states[i] == VisitState::Unvisited {
            dfs(i, objects, &mut states, &mut result);
        }
    }

    result
}

fn dfs(
    node: usize,
    objects: &Vec<[&Option<Constraint>; 4]>,
    states: &mut Vec<VisitState>,
    result: &mut Vec<usize>,
) {
    if states[node] == VisitState::Visiting {
        panic!("Cycle detected in constraint graph at node {}", node);
    }

    if states[node] == VisitState::Visited {
        return;
    }

    states[node] = VisitState::Visiting;

    // Visit all dependencies
    for constraint in &objects[node] {
        if let Some(c) = constraint {
            if let Some(dep) = get_dependency(c) {
                if dep >= objects.len() {
                    panic!("Invalid dependency index {} in node {}", dep, node);
                }
                dfs(dep, objects, states, result);
            }
        }
    }

    states[node] = VisitState::Visited;
    result.push(node);
}

/// Extracts the dependency index from a constraint, if it has one.
fn get_dependency(constraint: &Constraint) -> Option<usize> {
    match constraint {
        Constraint::Left(_, idx) => Some(*idx),
        Constraint::Right(_, idx) => Some(*idx),
        Constraint::Top(_, idx) => Some(*idx),
        Constraint::Bottom(_, idx) => Some(*idx),
        _ => None, // Parent constraints have no dependencies
    }
}

// impl LayoutView for ConstraintLayout {
//     fn set_bounds(&mut self, width: Option<Bounds>, height: Option<Bounds>) {
//         // Implement bounds setting logic here
//     }
// }
