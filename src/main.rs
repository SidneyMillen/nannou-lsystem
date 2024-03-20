use fractal_plant::FractalPlantLSystem;
use fractal_tree::FractalTreeLSystem;
use lsystem::{LRules, LSystem, MapRules};
pub use lsystems::{DrawableLSystem, LSystemDrawingParamaters, LSystemRules};
use nannou::{geom::rect, prelude::*};
use nannou_egui::{self, egui, Egui};
use sierpinski_triangle::SierpinskiTriangleLSystem;

mod dragon_curve;
mod fractal_plant;
mod fractal_tree;
mod levy_c_curve;
mod lsystems;
mod sierpinski_triangle;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSystemSelection {
    DragonCurve,
    SierpinskiTriangle,
    LevyCCurve,
    FractalTree,
    FractalPlant,
}

#[derive(Clone, Debug)]
pub struct Settings {
    lsystem_selection: LSystemSelection,
    lsystem_levels: usize,
    fractal_plant_lsystem: FractalPlantLSystem,
    fractal_tree_lsystem: FractalTreeLSystem,
    sierpinski_triangle_lsystem: SierpinskiTriangleLSystem,
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
            sierpinski_triangle_lsystem: SierpinskiTriangleLSystem::new(5.0, vec2(0.0, 0.0), 0.0),
            fractal_plant_lsystem: FractalPlantLSystem::new(5.0, vec2(0.0, 0.0), deg_to_rad(-30.0)),
            fractal_tree_lsystem: FractalTreeLSystem::new(
                5.0,
                vec2(0.0, 0.0),
                deg_to_rad(-90.0),
                Rgb::new(1.0, 1.0, 1.0),
                Rgb::new(0.0, 1.0, 0.0),
            ),
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
        ui.add(egui::Slider::new(&mut settings.lsystem_levels, 1..=7));
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
        match settings.lsystem_selection {
            LSystemSelection::DragonCurve => {}
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
                nannou_egui::color_picker::color_edit_button_rgb(
                    ui,
                    &mut [
                        fractal_tree_settings.leaf_color.red,
                        fractal_tree_settings.leaf_color.green,
                        fractal_tree_settings.leaf_color.blue,
                    ],
                );
                nannou_egui::color_picker::color_edit_button_rgb(
                    ui,
                    &mut [
                        fractal_tree_settings.branch_color.red,
                        fractal_tree_settings.branch_color.green,
                        fractal_tree_settings.branch_color.blue,
                    ],
                );
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
    let t = app.time;
    let draw = app.draw();
    let settings = &model.settings;

    // Clear the background to black.
    draw.background().color(BLACK);

    match model.settings.lsystem_selection {
        LSystemSelection::DragonCurve => {
            let dragon_curve_drawable = dragon_curve::DragonCurve {};
            dragon_curve_drawable.draw(&draw, &win, &model.settings.lsystem_levels);
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
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
