use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;
use nannou_egui::egui;

use crate::{DrawableLSystem, LSystemRules};

pub fn fractal_plant_rules_object() -> LSystemRules {
    let rules = vec![
        ('X', "F-[[X]+X]+F[+FX]-X".to_string()),
        ('F', "FF".to_string()),
    ];

    LSystemRules::new(vec!['X'], rules)
}

#[derive(Debug, Clone)]
pub struct FractalPlantLSystem {
    pub line_length: f32,
    pub start_pos: Vec2,
    pub start_angle: f32,
}

impl FractalPlantLSystem {
    pub fn new(line_length: f32, start_pos: Vec2, start_angle: f32) -> Self {
        FractalPlantLSystem {
            line_length,
            start_pos,
            start_angle,
        }
    }
    pub fn default() -> Self {
        FractalPlantLSystem {
            line_length: 5.0,
            start_pos: vec2(0.0, 0.0),
            start_angle: deg_to_rad(-30.0),
        }
    }
}

impl DrawableLSystem for FractalPlantLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = fractal_plant_rules_object()
            .eval(levels)
            .expect("lsystem evaluation failed");

        let system_iter = evaluated_lsystem.chars();
        let mut pos = self.start_pos;
        let mut pos_stack: Vec<Vec2> = Vec::new();
        pos_stack.push(pos);
        let mut angle = self.start_angle;
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);

        for c in system_iter {
            match c {
                'F' => {
                    let new_pos = pos + vec2(0.0, self.line_length).rotate(angle);
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

    fn get_rules(&self) -> crate::LSystemRules {
        fractal_plant_rules_object()
    }
}
