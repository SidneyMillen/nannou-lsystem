use levy_c_curve::{levy_c_curve_rules, LevyCCurve};
use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;
use sierpinski_triangle::{draw_sierpinski_triangle, sierpinski_triangle_rules};

mod fractal_tree;
mod fractal_plant;
mod dragon_curve;
mod sierpinski_triangle;
mod levy_c_curve;

static LSYSTEM_LEVELS: usize = 4;



struct Model {
    drawable: LevyCCurve,
    lsystem_levels: usize,
}

pub struct LSystemRules {
    axiom: Vec<char>,
    rules: MapRules<char>,
}

impl LSystemRules {
    fn new(axiom: Vec<char>, rules: MapRules<char>) -> Self {
        LSystemRules { axiom, rules }
    }
}

pub trait Drawable {
    fn draw(&self, draw: &Draw, win: &Rect<f32>);
}

struct LSystemDrawingParamaters {
    start_pos: Vec2,
    pos: Vec2,
    angle: f32,
    pos_stack: Vec<Vec2>,
    angle_stack: Vec<f32>,
}

impl LSystemDrawingParamaters {
    fn new(start_pos: Vec2, angle: f32) -> Self {
        LSystemDrawingParamaters {
            start_pos,
            pos: start_pos,
            angle,
            pos_stack: Vec::new(),
            angle_stack: Vec::new(),
        }
    }
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {



    Model {
        drawable: LevyCCurve::default(),
        lsystem_levels: LSYSTEM_LEVELS,}
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(Key::Up) => {
                model.lsystem_levels += 1;
                model.drawable = LevyCCurve::new(vec!['F'], levy_c_curve_rules(), vec2(0.0, 0.0));

            }
            KeyPressed(Key::Down) => {
                model.lsystem_levels -= 1;
                model.drawable = LevyCCurve::new(vec!['F'], levy_c_curve_rules(), vec2(0.0, 0.0));
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

    model.drawable.draw(&draw, &win);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub fn eval_lsystem(rules: MapRules<char>, levels: usize) -> String {
    let mut system = LSystem::new(rules, vec!['F']);
    system.nth(levels).unwrap().into_iter().collect()
}
