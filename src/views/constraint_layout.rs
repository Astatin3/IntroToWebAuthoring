use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    log,
    parser::Tag,
    render::Renderer,
    views::{Constraint, View, box_view::BoxView},
};

#[derive(Debug)]
pub struct ConstraintLayout {
    pub children: Vec<BoxView>,
    child_positions: Option<HashMap<usize, Rect>>,
    child_order: Option<Vec<usize>>,
}

#[derive(Debug)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl ConstraintLayout {
    pub fn new(children: Vec<BoxView>) -> Self {
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

        let mut this = ConstraintLayout {
            children,
            child_positions: None,
            child_order: None,
        };

        this.recalculate_child_render_order();

        this
    }

    pub fn recalculate_positions(&mut self, x: f32, y: f32, self_w: f32, self_h: f32) {
        let self_top = y;
        let self_left = x;
        let self_right = x + self_w;
        let self_bottom = y + self_h;

        let mut calculated_positions: HashMap<usize, Rect> = HashMap::new();

        let child_order = self.child_order.as_ref().unwrap();
        for idx in child_order {
            let child: &mut BoxView = &mut self.children[*idx];

            log!("{:?}", child.get_constraints());

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

            let y_align = |constraint: &Option<Constraint>, margin_reverse: bool| {
                let margin_mult = if margin_reverse { -1. } else { 1. };
                match constraint {
                    Some(Constraint::TopParent(margin)) => Some(self_top + margin * margin_mult),
                    Some(Constraint::BottomParent(margin)) => {
                        Some(self_bottom + margin * margin_mult)
                    }
                    Some(Constraint::Top(margin, id)) => {
                        let other = calculated_positions.get(&id).unwrap();
                        Some(other.y + margin * margin_mult)
                    }

                    Some(Constraint::Bottom(margin, id)) => {
                        let other = calculated_positions.get(&id).unwrap();
                        Some(other.y + other.height + margin * margin_mult)
                    }
                    None => None,
                    _ => unreachable!(),
                }
            };

            let y = match (y_align(c_top, false), y_align(c_bottom, true)) {
                (Some(top), Some(bottom)) => (bottom + top - c_height) / 2.,
                (Some(top), None) => top,
                (None, Some(bottom)) => bottom - c_height,
                (None, None) => unreachable!("Vertically unconstrained"),
            };

            let x_align = |constraint: &Option<Constraint>, margin_reverse: bool| {
                let margin_mult = if margin_reverse { -1. } else { 1. };
                match constraint {
                    Some(Constraint::LeftParent(margin)) => Some(self_left + margin * margin_mult),
                    Some(Constraint::RightParent(margin)) => {
                        Some(self_right + margin * margin_mult)
                    }
                    Some(Constraint::Left(margin, id)) => {
                        let other = calculated_positions.get(&id).unwrap();
                        Some(other.x + margin * margin_mult)
                    }
                    Some(Constraint::Right(margin, id)) => {
                        let other = calculated_positions.get(&id).unwrap();
                        Some(other.x + other.width + margin * margin_mult)
                    }
                    None => None,
                    _ => unreachable!(),
                }
            };

            let x = match (x_align(c_left, false), x_align(c_right, true)) {
                (Some(left), Some(right)) => (right + left - c_width) / 2.,
                (Some(left), None) => left,
                (None, Some(right)) => right - c_width,
                (None, None) => unreachable!("Horizontally unconstrained"),
            };

            calculated_positions.insert(
                *idx,
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

    pub fn recalculate_child_render_order(&mut self) {
        let dependencies = self
            .children
            .iter()
            .map(|c| {
                let constraints = c.get_constraints();
                [constraints.0, constraints.1, constraints.2, constraints.3]
            })
            .collect::<Vec<_>>();

        self.child_order = Some(topological_sort(&dependencies));
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

    fn from_tag(
        attributes: &std::collections::HashMap<String, String>,
        children: &Vec<Tag>,
    ) -> Box<dyn View>
    where
        Self: Sized,
    {
        let mut parsed_children = Vec::new();

        for child in children {
            let child = child.parse();

            if let Ok(b) = child.as_any().downcast::<BoxView>() {
                parsed_children.push(*b);
            } else {
                panic!("Constraint Layout must contain only BoxView!")
            }
        }

        Box::new(Self::new(parsed_children))
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
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
        log!("Cycle detected in constraint graph at node {}", node);
        panic!();
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
