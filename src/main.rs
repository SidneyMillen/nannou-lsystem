use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

static LSystemLevels: usize = 4;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);

    let mut rules = MapRules::new();
    rules.set_str('0', "1[0]0");
    rules.set_str('1', "11");
    let axiom = "0".chars().collect();
    let mut system = LSystem::new(rules, axiom);
    let final_system_state = eval_lsystem(system, LSystemLevels);

    draw_lsystem(final_system_state, &draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub fn eval_lsystem(mut system: LSystem<char, MapRules<char>>, levels: usize) -> String {
    system.nth(levels).unwrap().into_iter().collect()
}

fn draw_lsystem(final_system_state: String, draw: &Draw, win: &Rect<f32>) {
    //println!("Final state: {:?}", final_system_state);
    let start_pos = win.mid_bottom();
    let system_iter = final_system_state.chars();
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
                let new_pos = pos + vec2(0.0, 25.0).rotate(angle);
                draw.line()
                    .start(pos)
                    .end(new_pos)
                    .color(WHITE)
                    .stroke_weight(2.0);
                pos = new_pos;
            }
            '0' => {
                let new_pos = pos + vec2(0.0, 25.0).rotate(angle);
                draw.line()
                    .start(pos)
                    .end(pos)
                    .color(WHITE)
                    .stroke_weight(2.0);
                draw.ellipse().x_y(pos.x, pos.y).radius(5.0).color(PURPLE);
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
