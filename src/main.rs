use fractal_plant::FractalPlantLSystem;
use lsystem::{LRules, LSystem, MapRules};
use nannou::{geom::rect, prelude::*};

mod dragon_curve;
mod fractal_plant;
mod fractal_tree;
mod levy_c_curve;
mod sierpinski_triangle;

pub enum LSystemSelection {
    DragonCurve,
    SierpinskiTriangle,
    LevyCCurve,
    FractalTree,
    FractalPlant,
}

struct Model {
    lsystem_selection: LSystemSelection,
    lsystem_levels: usize,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    Model {
        lsystem_selection: LSystemSelection::SierpinskiTriangle,
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

    match model.lsystem_selection {
        LSystemSelection::DragonCurve => {
            let dragon_curve_drawable = dragon_curve::DragonCurve {};
            dragon_curve_drawable.draw(&draw, &win, &model.lsystem_levels);
        }
        LSystemSelection::SierpinskiTriangle => {
            let sierpinski_triangle_drawable =
                sierpinski_triangle::SierpinskiTriangleLSystem::new();
            sierpinski_triangle_drawable.draw(&draw, &win, &model.lsystem_levels);
        }
        LSystemSelection::LevyCCurve => {
            let levy_c_drawable = levy_c_curve::setup_levy_c_curve_lsystem(app.window_rect());

            levy_c_drawable.draw(&draw, &win, &model.lsystem_levels)
        }
        LSystemSelection::FractalTree => {
            let fractal_tree_drawable = fractal_tree::FractalTreeLSystem::new();
            fractal_tree_drawable.draw(&draw, &win, &model.lsystem_levels);
        }
        LSystemSelection::FractalPlant => {
            let fractal_plant_drawable = FractalPlantLSystem::new();
            fractal_plant_drawable.draw(&draw, &win, &model.lsystem_levels);
        }
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub struct LSystemRules {
    pub axiom: Vec<char>,
    pub rules: Vec<(char, String)>,
}

impl LSystemRules {
    pub fn new(axiom: Vec<char>, rules: Vec<(char, String)>) -> Self {
        LSystemRules { axiom, rules }
    }
    pub fn eval(&self, levels: &usize) -> Option<String> {
        let mut map_rules = MapRules::new();
        for (k, v) in self.rules.clone() {
            map_rules.set_str(k, &v);
        }

        let mut system = LSystem::new(map_rules, self.axiom.clone());
        let output = system.nth(levels.clone())?.into_iter().collect();
        Some(output)
    }
}

pub trait DrawableLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize);
    fn get_rules(&self) -> LSystemRules;
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
