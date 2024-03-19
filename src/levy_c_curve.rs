use nannou::prelude::*;
use lsystem::{MapRules, LSystem};
use crate::{eval_lsystem, LSYSTEM_LEVELS};
use crate::{Drawable, LSystemDrawingParamaters, LSystemRules};


pub fn levy_c_curve_rules() -> MapRules<char> {
    let mut rules = MapRules::new();
    rules.set_str('F', "+F--F+");
    rules
}

pub struct LevyCCurve {
    params: LSystemDrawingParamaters,
    rules: LSystemRules,
}

impl LevyCCurve {
    pub fn new(axiom: Vec<char>, rules: MapRules<char>, start_pos: Vec2) -> Self {
        let start_pos = start_pos;
        let start_angle = deg_to_rad(45.0);
        let params = LSystemDrawingParamaters::new(start_pos, start_angle);
        let rules = LSystemRules::new(axiom, rules);
        LevyCCurve { params, rules }
    }

    pub fn default() -> Self {
        let axiom = vec!['F'];
        let rules = levy_c_curve_rules();
        LevyCCurve::new(axiom, rules, vec2(0.0, 0.0))
    }
}

impl Drawable for LevyCCurve {
    fn draw(&self, draw: &Draw, win: &Rect<f32>) {
        let evaluated_lsystem = eval_lsystem(levy_c_curve_rules(), LSYSTEM_LEVELS);
        let system_iter = evaluated_lsystem.chars();
        let mut pos = self.params.start_pos;
        let mut angle = self.params.angle;

        for c in system_iter {
            match c {
                'F' => {
                    let new_pos = pos + vec2(0.0, 3.0).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(BLUE)
                        .stroke_weight(2.0);
                    pos = new_pos;
                }
                '+' => {
                    angle += deg_to_rad(45.0);
                }
                '-' => {
                    angle -= deg_to_rad(45.0);
                }
                _ => (),
            }
        }
    }
}
