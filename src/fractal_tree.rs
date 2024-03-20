use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

pub struct FractalTreeLSystem {}

impl FractalTreeLSystem {
    pub fn new() -> Self {
        FractalTreeLSystem {}
    }
}

impl DrawableLSystem for FractalTreeLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = fractal_tree_rules_object()
            .eval(levels)
            .expect("fractal tree lsystem evaluation failed");
        draw_fractal_tree(&evaluated_lsystem, draw, win);
    }
    fn get_rules(&self) -> LSystemRules {
        fractal_tree_rules_object()
    }
}

    pub fn draw_fractal_tree(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
        let start_pos = win.xy();
        let system_iter = evaluated_lsystem.chars();
        let mut pos = start_pos;
        let mut pos_stack: Vec<Vec2> = Vec::new();
        pos_stack.push(pos);
        let start_angle = 0.0;
        let mut angle = start_angle;
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);

        for c in system_iter {
            match c {
                '1' => {
                    let new_pos = pos + vec2(0.0, 7.5).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(WHITE)
                        .stroke_weight(2.0);
                    pos = new_pos;
                }
                '0' => {
                    let new_pos = pos + vec2(0.0, 7.5).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(WHITE)
                        .stroke_weight(2.0);
                    draw.ellipse()
                        .x_y(new_pos.x, new_pos.y)
                        .radius(3.0)
                        .color(PURPLE);
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

    fn fractal_tree_rules_object() -> LSystemRules {
        let rules = vec![('0', "1[0]0".to_string()), ('1', "11".to_string())];

        LSystemRules::new(vec!['0'], rules)
}
