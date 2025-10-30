use std::collections::HashMap;

use crate::{
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
    child_positions: Option<HashMap<usize, Rect>>,
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

        ConstraintLayout {
            children,
            child_positions: None,
        }
    }

    pub fn recalculate_positions(&mut self, x: f32, y: f32, self_w: f32, self_h: f32) {
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

            let y_align = |constraint: &Option<Constraint>| match constraint {
                Some(Constraint::TopParent(margin)) => Some(self_top + margin),
                Some(Constraint::BottomParent(margin)) => Some(self_bottom - margin),
                Some(Constraint::Top(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.y - margin)
                }

                Some(Constraint::Bottom(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.y + other.height + margin)
                }
                None => None,
                _ => unreachable!(),
            };

            let y = match (y_align(c_top), y_align(c_bottom)) {
                (Some(top), Some(bottom)) => (bottom + top - c_height) / 2.,
                (Some(top), None) => top,
                (None, Some(bottom)) => bottom - c_height,
                (None, None) => unreachable!("Vertically unconstrained"),
            };

            let x_align = |constraint: &Option<Constraint>| match constraint {
                Some(Constraint::LeftParent(margin)) => Some(self_left + margin),
                Some(Constraint::RightParent(margin)) => Some(self_right - margin),
                Some(Constraint::Left(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.x - margin)
                }

                Some(Constraint::Right(margin, id)) => {
                    let other = calculated_positions.get(&id).unwrap();
                    Some(other.x + other.width - margin)
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

            calculated_positions.insert(
                idx,
                Rect {
                    x: x,
                    y: y,
                    width: c_width,
                    height: c_height,
                },
            );
        }

        self.child_positions = Some(calculated_positions);
    }
}

impl View for ConstraintLayout {
    fn draw(&mut self, renderer: &mut Renderer, _x: f32, _y: f32, _self_w: f32, _self_h: f32) {
        if let Some(child_positions) = &mut self.child_positions {
            for i in 0..self.children.len() {
                let rect = child_positions.get(&i).unwrap();
                self.children[i].draw(renderer, rect.x, rect.y, rect.width, rect.height);
            }
        }
    }

    fn resize(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.recalculate_positions(x, y, w, h);
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
