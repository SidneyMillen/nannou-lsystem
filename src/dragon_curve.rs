use nannou::prelude::*;

use crate::{DrawableLSystem, LSystemRules};

#[derive(Debug, Clone)]
pub struct DragonCurveLSystem {
    pub start_pos: Vec2,
    pub start_angle: f32,
    pub line_length: f32,
    pub draw_color: Hsv,
}
    
impl DragonCurveLSystem {
    pub fn new(start_pos: Vec2, start_angle: f32, line_length: f32, draw_color: Hsv) -> Self {
        DragonCurveLSystem {
            start_pos,
            start_angle,
            line_length,
            draw_color,
        }
    }
}

impl DrawableLSystem for DragonCurveLSystem {
    fn draw(&self, draw: &Draw, win: &Rect<f32>, levels: &usize) {
        let evaluated_lsystem = self
            .get_rules()
            .eval(levels)
            .expect("lsystem evaluation failed");

        let system_iter = evaluated_lsystem.chars();
        let mut angle = self.start_angle;
        let mut pos = self.start_pos;

        for c in system_iter {
            match c {
                'F' | 'G' => {
                    let new_pos = pos + vec2(0.0, self.line_length).rotate(angle);
                    draw.line()
                        .start(pos)
                        .end(new_pos)
                        .color(self.draw_color)
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
    fn get_rules(&self) -> LSystemRules {

    let axiom = vec!['F'];
    let rules = vec![('F', "F+G".to_string()), ('G', "F-G".to_string())];
    LSystemRules::new(axiom, rules)

    }
}


