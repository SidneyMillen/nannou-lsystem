use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

pub fn fractal_plant_rules() -> MapRules<char> {
    let mut rules = MapRules::new();
    rules.set_str('X', "F-[[X]+X]+F[+FX]-X");
    rules.set_str('F', "FF");
    rules
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
