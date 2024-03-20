use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

pub fn fractal_plant_rules_object() -> LSystemRules {
    let rules = vec![
        ('X', "F-[[X]+X]+F[+FX]-X".to_string()),
        ('F', "FF".to_string()),
    ];

    LSystemRules::new(vec!['X'], rules)
}

pub struct FractalPlantLSystem {}

impl FractalPlantLSystem {
    pub fn new() -> Self {
        FractalPlantLSystem {}
    }
}

impl DrawableLSystem for FractalPlantLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = fractal_plant_rules_object()
            .eval(levels)
            .expect("lsystem evaluation failed");
        draw_fractal_plant(&evaluated_lsystem, draw, win);
    }

    fn get_rules(&self) -> crate::LSystemRules {
        fractal_plant_rules_object()
    }
}
pub fn draw_fractal_plant(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
    let start_pos = win.bottom_left() + vec2(50.0, 0.0);
    let system_iter = evaluated_lsystem.chars();
    let mut pos = start_pos;
    let mut pos_stack: Vec<Vec2> = Vec::new();
    pos_stack.push(pos);
    let start_angle = deg_to_rad(-30.0);
    let mut angle = start_angle;
    let mut angle_stack: Vec<f32> = Vec::new();
    angle_stack.push(angle);

    for c in system_iter {
        match c {
            'F' => {
                let new_pos = pos + vec2(0.0, 7.5).rotate(angle);
                draw.line()
                    .start(pos)
                    .end(new_pos)
                    .color(GREEN)
                    .stroke_weight(2.0);
                pos = new_pos;
            }
            '-' => {
                angle += deg_to_rad(25.0);
            }
            '+' => {
                angle -= deg_to_rad(25.0);
            }
            '[' => {
                pos_stack.push(pos);
                angle_stack.push(angle);
            }
            ']' => {
                pos = pos_stack.pop().unwrap();
                angle = angle_stack.pop().unwrap();
            }
            _ => (),
        }
    }
}
