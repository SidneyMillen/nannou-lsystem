use lsystem::{LSystem, MapRules};
use nannou::prelude::*;

pub fn dragon_curve_rules() -> MapRules<char> {
    let mut rules = MapRules::new();
    rules.set_str('F', "F+G");
    rules.set_str('G', "F-G");
    rules
}

pub fn draw_dragon_curve(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
    let start_pos = win.xy();
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
                angle += PI / 2.0;
            }
            '-' => {
                angle -= PI / 2.0;
            }
            _ => (),
        }
    }
}
