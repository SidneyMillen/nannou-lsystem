use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

#[derive(Debug, Clone)]
pub struct SierpinskiTriangleLSystem {
    pub line_length: f32,
    pub start_pos: Vec2,
    pub start_angle: f32,
    pub draw_color: Hsv,
}

impl SierpinskiTriangleLSystem {
    pub fn new(line_length: f32, start_pos: Vec2, start_angle: f32, draw_color: Hsv) -> Self {
        SierpinskiTriangleLSystem {
            line_length,
            start_pos,
            start_angle,
            draw_color,
        }
    }
}

impl DrawableLSystem for SierpinskiTriangleLSystem {
    fn draw(&self, draw: &Draw, _win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = sierpinski_triangle_rules_object()
            .eval(levels)
            .expect("sierpinski triangle lsystem evaluation failed");

        let system_iter = evaluated_lsystem.chars();
        let mut pos = self.start_pos;
        let mut angle = self.start_angle;

        for c in system_iter {
            match c {
                'F' | 'G' => {
                    let new_pos = pos + vec2(0.0, self.line_length).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(self.draw_color)
                        .stroke_weight(2.0);
                    pos = new_pos;
                }
                '+' => {
                    angle += 2.0 * PI / 3.0;
                }
                '-' => {
                    angle -= 2.0 * PI / 3.0;
                }
                _ => (),
            }
        }
    }
    fn get_rules(&self) -> LSystemRules {
        sierpinski_triangle_rules_object()
    }
}

pub fn sierpinski_triangle_rules_object() -> LSystemRules {
    let axiom = vec!['F'];
    let rules = vec![('F', "F-G+F+G-F".to_string()), ('G', "GG".to_string())];
    LSystemRules::new(axiom, rules)
}
