use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

use crate::LSystemRules;

#[derive(Debug, Clone)]
pub struct LSystemRulesEditor {
    rules: LSystemRules,
}
impl LSystemRulesEditor {
    pub fn new(rules: LSystemRules) -> Self {
        Self { rules }
    }
    pub fn setup_window(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut gui_axiom = self.rules.axiom.iter().collect::<String>();
        egui::Window::new("LSystem Rules").show(ui.ctx(), |ui| {
            ui.label("Axiom");
            ui.text_edit_singleline(&mut gui_axiom);
            ui.label("Rules");
            let mut rules_copy = self.rules.rules.clone();
            for (i, rule) in self.rules.rules.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label("->");
                    let to = ui.add(egui::TextEdit::singleline(&mut rule.1));
                    if to.changed() {
                        rules_copy[i].1 = rule.1.clone();
                    }
                    if ui.button("X").clicked() {
                        rules_copy.remove(i);
                    }
                });
            }

            self.rules.rules = rules_copy;
            self.rules.axiom = gui_axiom.chars()[0];

            if ui.button("Add Rule").clicked() {
                self.rules.rules.push(Default::default());
            }
        });
    }
}
