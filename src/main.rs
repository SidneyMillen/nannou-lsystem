use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;

static LSYSTEM_LEVELS: usize = 6;

struct Model {
    evaluated_lsystem: String,
    lsystem_levels: usize,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    let mut rules = MapRules::new();
    rules.set_str('0', "1[0]0");
    rules.set_str('1', "11");
    let axiom: Vec<char> = "0".chars().collect();
    let evaluated_lsystem = eval_lsystem(rules, LSYSTEM_LEVELS);

    Model {
        evaluated_lsystem,
        lsystem_levels: LSYSTEM_LEVELS,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(Key::Up) => {
                model.lsystem_levels += 1;
            }
            KeyPressed(Key::Down) => {
                model.lsystem_levels -= 1;
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to black.
    draw.background().color(BLACK);

    draw_lsystem(&model.evaluated_lsystem, &draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub fn eval_lsystem(rules: MapRules<char>, levels: usize) -> String {
    let mut system = LSystem::new(rules, vec!['0']);
    system.nth(levels).unwrap().into_iter().collect()
}

fn draw_lsystem(evaluated_lsystem: &String, draw: &Draw, win: &Rect<f32>) {
    //println!("Final state: {:?}", final_system_state);
    let start_pos = win.mid_bottom();
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
