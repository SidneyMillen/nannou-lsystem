use lsystem::{LSystem, MapRules};
use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};


pub struct DragonCurve {}

impl DrawableLSystem for DragonCurve {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = self.get_rules().eval(levels).expect("lsystem evaluation failed");
        draw_dragon_curve(&evaluated_lsystem, draw, win);
    }
    fn get_rules(&self) -> LSystemRules {
        dragon_curve_rules_object()
    }
}


pub fn dragon_curve_rules_object() -> LSystemRules {
    let axiom = vec!['F'];
    let rules = vec![
        ('F', "F+G".to_string()),
        ('G', "F-G".to_string())];
    LSystemRules::new(axiom, rules)
}


pub fn draw_dragon_curve(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
    let start_pos = win.xy();
    let system_iter = evaluated_lsystem.chars();
    let mut pos = start_pos;
    let mut angle = 0.0;

    for c in system_iter {
        match c {
            'F' | 'G' => {
                let new_pos = pos + vec2(0.0, 5.0).rotate(angle);
                draw.line()
                    .start(pos)
                    .end(new_pos)
                    .color(BLUE)
                    .stroke_weight(2.0);
                pos = new_pos;
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
