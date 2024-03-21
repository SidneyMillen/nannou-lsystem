use lsystem::{LSystem, MapRules};
use nannou::{geom::Rect, glam::Vec2, Draw};

#[derive(Debug, Clone)]
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
    fn draw(&self, draw: &Draw, win: &Rect, levels: &usize);
    fn get_rules(&self) -> LSystemRules;
}

pub struct LSystemDrawingParamaters {
    pub start_pos: Vec2,
    pub pos: Vec2,
    pub angle: f32,
    pos_stack: Vec<Vec2>,
    angle_stack: Vec<f32>,
}

impl LSystemDrawingParamaters {
    pub fn new(start_pos: Vec2, angle: f32) -> Self {
        LSystemDrawingParamaters {
            start_pos,
            pos: start_pos,
            angle,
            pos_stack: Vec::new(),
            angle_stack: Vec::new(),
        }
    }
}
