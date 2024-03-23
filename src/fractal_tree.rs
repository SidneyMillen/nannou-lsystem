use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

#[derive(Debug, Clone)]
pub struct FractalTreeLSystem {
    pub line_length: f32,
    pub start_pos: Vec2,
    pub start_angle: f32,
    pub branch_color: Hsv,
    pub leaf_color: Hsv,
}

impl FractalTreeLSystem {
    pub fn new(
        line_length: f32,
        start_pos: Vec2,
        start_angle: f32,
        branch_color: Hsv,
        leaf_color: Hsv,
    ) -> Self {
        FractalTreeLSystem {
            line_length,
            start_pos,
            start_angle,
            branch_color,
            leaf_color,
        }
    }
}

impl DrawableLSystem for FractalTreeLSystem {
    fn draw(&self, draw: &Draw, _win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = fractal_tree_rules_object()
            .eval(levels)
            .expect("fractal tree lsystem evaluation failed");

        let system_iter = evaluated_lsystem.chars();
        let mut pos = self.start_pos;
        let mut pos_stack: Vec<Vec2> = Vec::new();
        pos_stack.push(pos);
        let mut angle = self.start_angle;
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);

        for c in system_iter {
            match c {
                '1' => {
                    let new_pos = pos + vec2(0.0, self.line_length).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(self.branch_color)
                        .stroke_weight(2.0);
                    pos = dbg! { new_pos };
                }
                '0' => {
                    let new_pos = pos + vec2(0.0, 7.5).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(self.branch_color)
                        .stroke_weight(2.0);
                    draw.ellipse()
                        .x_y(new_pos.x, new_pos.y)
                        .radius(3.0)
                        .color(self.leaf_color);
                }
                '[' => {
                    pos_stack.push(pos);
                    angle_stack.push(angle);
                    angle += PI / 4.0;
                }
                ']' => {
                    pos = pos_stack.pop().unwrap();
                    angle = angle_stack.pop().unwrap();
                    angle -= PI / 4.0;
                }
                _ => (),
            }
        }
    }
    fn get_rules(&self) -> LSystemRules {
        fractal_tree_rules_object()
    }
}

fn fractal_tree_rules_object() -> LSystemRules {
    let rules = vec![('0', "1[0]0".to_string()), ('1', "11".to_string())];

    LSystemRules::new(vec!['0'], rules)
}
