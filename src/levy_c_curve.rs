use crate::{DrawableLSystem, LSystemDrawingParamaters, LSystemRules};
use lsystem::{LSystem, MapRules};
use nannou::prelude::*;

pub fn levy_c_curve_rules() -> Vec<(char, String)> {
    let rules = vec![('F', "+F--F+".to_string())];

    rules
}

fn levy_rules_object() -> LSystemRules {
    LSystemRules::new(vec!['F'], levy_c_curve_rules())
}

pub fn setup_levy_c_curve_lsystem(rect: Rect) -> LevyCCurve {
    let axiom = vec!['F'];
    let rules = levy_c_curve_rules();
    let start_pos = rect.xy();
    let start_angle = deg_to_rad(45.0);
    let drawable = LevyCCurve::new(axiom, rules, start_pos);
    drawable
}
pub struct LevyCCurve {
    params: LSystemDrawingParamaters,
}

impl LevyCCurve {
    pub fn new(axiom: Vec<char>, rules: Vec<(char, String)>, start_pos: Vec2) -> Self {
        let start_pos = start_pos;
        let start_angle = deg_to_rad(90.0);
        let params = LSystemDrawingParamaters::new(start_pos, start_angle);

        LevyCCurve { params }
    }

    pub fn with_params(
        axiom: Vec<char>,
        rules: Vec<(char, String)>,
        params: LSystemDrawingParamaters,
    ) -> Self {
        LevyCCurve { params }
    }

    pub fn default() -> Self {
        let axiom = vec!['F'];
        let rules = levy_c_curve_rules();
        LevyCCurve::new(axiom, rules, vec2(0.0, 0.0))
    }
}

impl DrawableLSystem for LevyCCurve {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = levy_rules_object()
            .eval(levels)
            .expect("lsystem evaluation failed");
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
                        .stroke_weight(1.0);
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

    fn get_rules(&self) -> LSystemRules {
        levy_rules_object()
    }
}
