mod dragon_curve;
mod fractal_plant;
mod fractal_tree;
mod koch_curves;
mod levy_c_curve;
mod lsystem_egui;
mod lsystems;
mod sierpinski_triangle;

use std::borrow::BorrowMut;

use fractal_plant::FractalPlantLSystem;
use fractal_tree::FractalTreeLSystem;

use lsystem_egui::LSystemRulesEditor;
pub use lsystems::{DrawableLSystem, LSystemDrawingParamaters, LSystemRules};
use nannou::{color::FromColor, prelude::*};
use nannou_egui::{self, egui, Egui};
use sierpinski_triangle::SierpinskiTriangleLSystem;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSystemSelection {
    DragonCurve,
    SierpinskiTriangle,
    LevyCCurve,
    FractalTree,
    FractalPlant,
    KochCurve,
}

#[derive(Clone, Debug)]
pub struct Settings {
    lsystem_selection: LSystemSelection,
    lsystem_levels: usize,
    fractal_plant_lsystem: FractalPlantLSystem,
    fractal_tree_lsystem: FractalTreeLSystem,
    sierpinski_triangle_lsystem: SierpinskiTriangleLSystem,
    dragon_curve_lsystem: dragon_curve::DragonCurveLSystem,
    koch_curve_lsystem: koch_curves::KochCurveLSystem,
    lsystem_rules_editor: LSystemRulesEditor,
}

struct Model {
    settings: Settings,
    egui: Egui,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        egui,
        settings: Settings {
            lsystem_selection: LSystemSelection::FractalPlant,
            lsystem_levels: 4,
            sierpinski_triangle_lsystem: SierpinskiTriangleLSystem::new(
                5.0,
                vec2(0.0, 0.0),
                0.0,
                Hsv::new(0.0, 1.0, 1.0),
            ),
            fractal_plant_lsystem: FractalPlantLSystem::new(
                5.0,
                vec2(0.0, 0.0),
                deg_to_rad(-30.0),
                Hsv::from_rgb(LinSrgb::new(0.0, 0.5, 0.0)),
                fractal_plant::another_custom_fractal_plant_rules_object(),
            ),
            fractal_tree_lsystem: FractalTreeLSystem::new(
                5.0,
                vec2(0.0, 0.0),
                deg_to_rad(-90.0),
                Hsv::new(0.5, 1.0, 1.0),
                Hsv::new(1.0, 1.0, 1.0),
            ),
            dragon_curve_lsystem: dragon_curve::DragonCurveLSystem::new(
                vec2(0.0, 0.0),
                0.0,
                5.0,
                Hsv::new(0.5, 1.0, 1.0),
            ),
            koch_curve_lsystem: koch_curves::KochCurveLSystem::with_rules(
                koch_curves::koch_pyramid_rules_object(),
            ),
            lsystem_rules_editor: LSystemRulesEditor::new(LSystemRules::new(
                vec!['F'],
                vec![('F', "F+F-F-F+F".to_string())],
            )),
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Iterations:");
        ui.add(egui::Slider::new(&mut settings.lsystem_levels, 1..=10));
        ui.label("L-System:");
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::FractalTree,
            "Fractal Tree",
        );
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::FractalPlant,
            "Fractal Plant",
        );
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::DragonCurve,
            "Dragon Curve",
        );
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::SierpinskiTriangle,
            "Sierpinski Triangle",
        );
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::LevyCCurve,
            "Levy C Curve",
        );
        ui.radio_value(
            &mut settings.lsystem_selection,
            LSystemSelection::KochCurve,
            "Koch Curve",
        );

        settings.lsystem_rules_editor.setup_window(ui, &ctx);

        match settings.lsystem_selection {
            LSystemSelection::DragonCurve => {
                let dragon_curve_settings = &mut settings.dragon_curve_lsystem;

                ui.label("Dragon Curve LSystem Parameters");
                ui.add(
                    egui::Slider::new(&mut dragon_curve_settings.line_length, 0.0..=20.0)
                        .text("Line Length"),
                );
                ui.add(
                    egui::Slider::new(&mut dragon_curve_settings.start_angle, -PI..=PI)
                        .text("Start Angle"),
                );

                let screen_rect = ctx.screen_rect();
                let width = screen_rect.width();
                let height = screen_rect.height();

                ui.add(
                    egui::Slider::new(
                        &mut dragon_curve_settings.start_pos.x,
                        -width / 2.0..=width / 2.0,
                    )
                    .text("Start Pos X"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut dragon_curve_settings.start_pos.y,
                        -height / 2.0..=height / 2.0,
                    )
                    .text("Start Pos Y"),
                );

                egui_edit_hsv(ui, &mut dragon_curve_settings.draw_color);
            }
            LSystemSelection::SierpinskiTriangle => {
                let sierpinski_triangle_settings = &mut settings.sierpinski_triangle_lsystem;

                ui.label("Sierpinski Triangle LSystem Parameters");
                ui.add(
                    egui::Slider::new(&mut sierpinski_triangle_settings.line_length, 0.0..=20.0)
                        .text("Line Length"),
                );
                ui.add(
                    egui::Slider::new(&mut sierpinski_triangle_settings.start_angle, -PI..=PI)
                        .text("Start Angle"),
                );

                let screen_rect = ctx.screen_rect();
                let width = screen_rect.width();
                let height = screen_rect.height();

                ui.add(
                    egui::Slider::new(
                        &mut sierpinski_triangle_settings.start_pos.x,
                        -width / 2.0..=width / 2.0,
                    )
                    .text("Start Pos X"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut sierpinski_triangle_settings.start_pos.y,
                        -height / 2.0..=height / 2.0,
                    )
                    .text("Start Pos Y"),
                );

                egui_edit_hsv(ui, &mut sierpinski_triangle_settings.draw_color);
            }
            LSystemSelection::LevyCCurve => {}
            LSystemSelection::FractalTree => {
                let fractal_tree_settings = &mut settings.fractal_tree_lsystem;
                ui.label("Fractal Tree LSystem Parameters");
                ui.add(
                    egui::Slider::new(&mut fractal_tree_settings.line_length, 0.0..=20.0)
                        .text("Line Length"),
                );
                ui.add(
                    egui::Slider::new(&mut fractal_tree_settings.start_angle, -PI..=PI)
                        .text("Start Angle"),
                );

                let screen_rect = ctx.screen_rect();
                let width = screen_rect.width();
                let height = screen_rect.height();

                ui.add(
                    egui::Slider::new(
                        &mut fractal_tree_settings.start_pos.x,
                        -width / 2.0..=width / 2.0,
                    )
                    .text("Start Pos X"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut fractal_tree_settings.start_pos.y,
                        -height / 2.0..=height / 2.0,
                    )
                    .text("Start Pos Y"),
                );

                egui_edit_hsv(ui, &mut fractal_tree_settings.branch_color);
                egui_edit_hsv(ui, &mut fractal_tree_settings.leaf_color);
            }
            LSystemSelection::FractalPlant => {
                let fractal_plant_settings = &mut settings.fractal_plant_lsystem;

                ui.label("Fractal Plant LSystem Parameters");
                ui.add(
                    egui::Slider::new(&mut fractal_plant_settings.line_length, 0.0..=20.0)
                        .text("Line Length"),
                );
                ui.add(
                    egui::Slider::new(&mut fractal_plant_settings.start_angle, -PI..=PI)
                        .text("Start Angle"),
                );

                let screen_rect = ctx.screen_rect();
                let width = screen_rect.width();
                let height = screen_rect.height();

                ui.add(
                    egui::Slider::new(
                        &mut fractal_plant_settings.start_pos.x,
                        -width / 2.0..=width / 2.0,
                    )
                    .text("Start Pos X"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut fractal_plant_settings.start_pos.y,
                        -height / 2.0..=height / 2.0,
                    )
                    .text("Start Pos Y"),
                );

                egui_edit_hsv(ui, &mut fractal_plant_settings.draw_color);
            }
            LSystemSelection::KochCurve => {
                let koch_curve_settings = &mut settings.koch_curve_lsystem;

                ui.label("Koch Curve LSystem Parameters");
                ui.add(
                    egui::Slider::new(&mut koch_curve_settings.line_length, 0.0..=20.0)
                        .text("Line Length"),
                );
                ui.add(
                    egui::Slider::new(&mut koch_curve_settings.start_angle, -PI..=PI)
                        .text("Start Angle"),
                );

                let screen_rect = ctx.screen_rect();
                let width = screen_rect.width();
                let height = screen_rect.height();

                ui.add(
                    egui::Slider::new(
                        &mut koch_curve_settings.start_pos.x,
                        -width / 2.0..=width / 2.0,
                    )
                    .text("Start Pos X"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut koch_curve_settings.start_pos.y,
                        -height / 2.0..=height / 2.0,
                    )
                    .text("Start Pos Y"),
                );

                egui_edit_hsv(ui, &mut koch_curve_settings.draw_color);
            }
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let _t = app.time;
    let draw = app.draw();
    let settings = &model.settings;

    // Clear the background to black.
    draw.background().color(BLACK);

    match model.settings.lsystem_selection {
        LSystemSelection::DragonCurve => {
            settings
                .dragon_curve_lsystem
                .draw(&draw, &win, &settings.lsystem_levels);
        }
        LSystemSelection::SierpinskiTriangle => {
            settings
                .sierpinski_triangle_lsystem
                .draw(&draw, &win, &settings.lsystem_levels);
        }
        LSystemSelection::LevyCCurve => {
            let levy_c_drawable = levy_c_curve::setup_levy_c_curve_lsystem(app.window_rect());

            levy_c_drawable.draw(&draw, &win, &settings.lsystem_levels)
        }
        LSystemSelection::FractalTree => {
            settings
                .fractal_tree_lsystem
                .draw(&draw, &win, &settings.lsystem_levels);
        }
        LSystemSelection::FractalPlant => {
            settings
                .fractal_plant_lsystem
                .draw(&draw, &win, &settings.lsystem_levels);
        }
        LSystemSelection::KochCurve => {
            settings
                .koch_curve_lsystem
                .draw(&draw, &win, &settings.lsystem_levels);
        }
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn egui_edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv =
        egui::ecolor::Hsva::new(color.hue.to_degrees(), color.saturation, color.value, 1.0);

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = nannou::color::Hsv::new(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}
