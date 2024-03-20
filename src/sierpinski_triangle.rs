use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

pub struct SierpinskiTriangleLSystem {}

impl SierpinskiTriangleLSystem {
    pub fn new() -> Self {
        SierpinskiTriangleLSystem {}
    }
}

impl DrawableLSystem for SierpinskiTriangleLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = sierpinski_triangle_rules_object()
            .eval(levels)
            .expect("sierpinski triangle lsystem evaluation failed");
        draw_sierpinski_triangle(&evaluated_lsystem, draw, win);
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


pub fn draw_sierpinski_triangle(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
    let start_pos = win.bottom_left() + vec2(50.0, 50.0);
    let system_iter = evaluated_lsystem.chars();
    let mut pos = start_pos;
    let mut angle = 0.0;

    for c in system_iter {
        match c {
            'F' | 'G' => {
                let new_pos = pos + vec2(0.0, 3.0).rotate(angle);
                draw.line()
                    .start(pos)
                    .end(new_pos)
                    .color(BLUE)
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
