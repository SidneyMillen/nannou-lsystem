use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

#[derive(Debug, Clone)]
pub struct KochCurveLSystem {
    pub line_length: f32,
    pub start_pos: Vec2,
    pub start_angle: f32,
    pub turn_angle: f32,
    pub rules: LSystemRules,
    pub draw_color: Hsv,
}

impl KochCurveLSystem {
    pub fn new(line_length: f32, start_pos: Vec2, start_angle: f32, turn_angle: f32) -> Self {
        KochCurveLSystem {
            line_length,
            start_pos,
            start_angle,
            turn_angle,
            rules: koch_island_rules_object(),
            draw_color: Hsv::new(0.0, 0.0, 1.0),
        }
    }
    pub fn default() -> Self {
        KochCurveLSystem {
            line_length: 5.0,
            start_pos: vec2(0.0, 0.0),
            start_angle: deg_to_rad(0.0),
            turn_angle: deg_to_rad(90.0),
            rules: koch_island_rules_object(),
            draw_color: Hsv::new(0.0, 0.0, 1.0),
        }
    }
    pub fn with_rules(rules: LSystemRules) -> Self {
        KochCurveLSystem {
            line_length: 5.0,
            start_pos: vec2(0.0, 0.0),
            start_angle: deg_to_rad(0.0),
            turn_angle: deg_to_rad(90.0),
            rules,
            draw_color: Hsv::new(0.0, 0.0, 1.0),
        }
    }
}

impl DrawableLSystem for KochCurveLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = self.rules.eval(levels).expect("lsystem evaluation failed");

        let system_iter = evaluated_lsystem.chars();
        let mut pos = self.start_pos;
        let mut angle = self.start_angle;

        for c in system_iter {
            match c {
                'F' => {
                    let new_pos = pos + vec2(0.0, self.line_length).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(self.draw_color)
                        .stroke_weight(2.0);
                    pos = new_pos;
                }
                'f' => {
                    pos += vec2(0.0, self.line_length).rotate(angle);
                }
                '+' => {
                    angle += PI / 2.0;
                }
                '-' => {
                    angle -= PI / 2.0;
                }
                _ => (),
            }
        }
    }
    fn get_rules(&self) -> LSystemRules {
        self.rules.clone()
    }
}

pub fn koch_island_rules_object() -> LSystemRules {
    // koch island
    let axiom = "F-F-F-F".chars().collect();
    let rules = vec![('F', "F-F+F+FF-F-F+F".to_string())];

    LSystemRules::new(axiom, rules)
}

pub fn koch_pyramid_rules_object() -> LSystemRules {
    // koch pyramid
    let axiom = "-F".chars().collect();
    let rules = vec![('F', "F+F-F-F+F".to_string())];

    LSystemRules::new(axiom, rules)
}
