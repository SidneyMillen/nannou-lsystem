use levy_c_curve::{levy_c_curve_rules, LevyCCurve};
use lsystem::{LRules, LSystem, MapRules};
use nannou::prelude::*;
use sierpinski_triangle::{draw_sierpinski_triangle, sierpinski_triangle_rules};

mod dragon_curve;
mod fractal_plant;
mod fractal_tree;
mod levy_c_curve;
mod sierpinski_triangle;

struct Model {
    drawable: LevyCCurve,
    lsystem_levels: usize,
}
impl Model {}

pub struct LSystemRules {
    pub axiom: Vec<char>,
    pub rules: Vec<(char, String)>,
}

impl LSystemRules {
    pub fn new(axiom: Vec<char>, rules: Vec<(char, String)>) -> Self {
        LSystemRules { axiom, rules }
    }
    pub fn eval(&self, levels: &usize) -> String {
        let mut map_rules = MapRules::new();
        for (k, v) in self.rules.clone() {
            map_rules.set_str(k, &v);
        }

        let mut system = LSystem::new(map_rules, self.axiom.clone());
        system.nth(levels.clone()).unwrap().into_iter().collect()
    }
}

pub trait Drawable {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize);
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
        lsystem_levels: 4,
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

    model.drawable.draw(&draw, &win, &model.lsystem_levels);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
