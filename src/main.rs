#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod math;
pub mod util;

use std::time::{Duration, Instant};

use eframe::egui::plot::{Corner, Legend, Line, PlotPoints};
use eframe::egui::{DragValue, RichText};
use eframe::{
    egui::{self},
    epaint::Color32,
};
use egui::plot::Plot;

use math::*;
use strum::{EnumIter, IntoEnumIterator};
use util::*;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        min_window_size: Some(egui::vec2(300.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Taylor method",
        options,
        Box::new(|_cc| Box::new(TaylorApp::default())),
    )
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
enum DropdownOptions {
    First,
    Second,
    Third,
    Fourth,
}

impl DropdownOptions {
    fn get_data(self: &Self) -> FunctionDetails {
        #[allow(unused_variables)]
        match self {
            DropdownOptions::First => FunctionDetails {
                function: vec![|x, y| y[0], |x, y| y[1]],
                solution: |x, (x0, y0)| y0 * (x - x0).exp(),
                name: "y",
            },
            DropdownOptions::Second => FunctionDetails {
                function: vec![|x, y| 3.0 * y[0], |x, y| 3.0 * y[1]],
                solution: |x, (x0, y0)| y0 * ((x - x0) * 3.0).exp(),
                name: "3y",
            },
            DropdownOptions::Third => FunctionDetails {
                function: vec![
                    |x, y| -x * x * y[0] * y[0] + 2.0 * y[0] / x, // y'
                    |x, y| {
                        // y''
                        -(2.0 * x.powi(4) * y[0] * y[1] + x.powi(3) * y[0].powi(2) - x * y[1]
                            + y[0])
                            / x.powi(2)
                    },
                ],
                solution: |x, (x0, y0)| {
                    5.0 * x * x * y0 / (x.powi(5) * y0 - x0.powi(5) * y0 + 5.0 * x0.powi(2))
                },
                name: "-x²y² + 2y/x (Bernoulli)",
            },
            DropdownOptions::Fourth => FunctionDetails {
                function: vec![|x, y| y[0].powi(2) + 4.0, |x, y| 2.0 * y[0] * y[1]],
                solution: |x, (x0, y0)| 2.0 * (2.0 * x - 2.0 * x0 + (y0 / 2.0).atan()).tan(),
                name: "y² + 4",
            },
        }
    }
    fn get_description(self: &Self) -> &'static str {
        self.get_data().name
    }
}

impl Default for DropdownOptions {
    fn default() -> Self {
        Self::First
    }
}

// #[derive(Default)]
struct TaylorApp {
    x0: f64,
    y0: f64,
    function_dropdown: DropdownOptions,
    points_approximated: Vec<[f64; 2]>,
    points_exact: Vec<[f64; 2]>,
    target_epsilon: f64,
    current_epsilon: f64,
    n_divisions: usize,
    calculations_duration: Duration,
    last_args: (f64, f64, DropdownOptions, f64),
}
impl Default for TaylorApp {
    fn default() -> Self {
        Self {
            x0: 0.0,
            y0: 1.0,
            function_dropdown: Default::default(),
            points_approximated: Default::default(),
            points_exact: Default::default(),
            target_epsilon: 0.1,
            current_epsilon: f64::INFINITY,
            n_divisions: 10,
            calculations_duration: Duration::new(0, 0),
            last_args: (0.0, 0.0, DropdownOptions::default(), 0.0),
        }
    }
}
impl TaylorApp {
    fn starting_conditions(self: &Self) -> (f64, f64) {
        (self.x0, self.y0)
    }
    fn generate_arg_tuple(self: &Self) -> (f64, f64, DropdownOptions, f64) {
        (
            self.x0,
            self.y0,
            self.function_dropdown,
            self.target_epsilon,
        )
    }
    fn recalculate_graph(self: &mut Self) {
        if self.generate_arg_tuple() == self.last_args {
            return;
        }
        self.last_args = self.generate_arg_tuple();
        let tick = Instant::now();
        let xy = self.starting_conditions();
        let (x, _) = xy;
        self.n_divisions = 3;
        loop {
            self.points_approximated = taylor_method(
                xy,
                x + DEFAULT_SETTINGS.interval_length,
                &self.function_dropdown.get_data(),
                self.n_divisions,
            );
            self.points_exact = generate_points(
                x,
                x + DEFAULT_SETTINGS.interval_length,
                |x| (self.function_dropdown.get_data().solution)(x, xy),
                self.n_divisions,
            );
            self.current_epsilon =
                calculate_error_simple(&self.points_approximated, &self.points_exact);
            if self.current_epsilon < self.target_epsilon || self.n_divisions >= 50_000
            // || self.current_epsilon.is_nan()
            {
                break;
            }
            self.n_divisions *= 2;
        }
        let tock = Instant::now();
        self.calculations_duration = tock - tick;
        let target_points = 1_000;
        if self.n_divisions > target_points {
            let n_th_lives = self.n_divisions / target_points;
            self.points_exact = self
                .points_exact
                .iter()
                .enumerate()
                .filter(|(i, _v)| i % n_th_lives == 0 || i.to_owned() == self.n_divisions - 1)
                .map(|(_i, v)| v.to_owned())
                .collect();

            self.points_approximated = self
                .points_approximated
                .iter()
                .enumerate()
                .filter(|(i, _v)| i % n_th_lives == 0 || i.to_owned() == self.n_divisions - 1)
                .map(|(_i, v)| v.to_owned())
                .collect();
        }
    }
}

impl eframe::App for TaylorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("y' = f(x,y) = ");
                        egui::ComboBox::from_label("")
                            .selected_text(self.function_dropdown.get_description())
                            .show_ui(ui, |ui| {
                                let mut add_line = |x: DropdownOptions| {
                                    ui.selectable_value(
                                        &mut self.function_dropdown,
                                        x.clone(),
                                        x.get_description(),
                                    )
                                };
                                for value in DropdownOptions::iter() {
                                    add_line(value);
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.label("f(");
                        ui.add(DragValue::new(&mut self.x0).speed(0.1).prefix("x:"));
                        ui.label(") = ");
                        ui.add(DragValue::new(&mut self.y0).speed(0.1).prefix("y:"));
                    });
                });
                ui.vertical(|ui| {
                    ui.label("Target error:");
                    ui.add(
                        DragValue::new(&mut self.target_epsilon)
                            .speed(0.01)
                            .clamp_range(0.01..=10.0)
                            .prefix("ε:"),
                    );
                });
                ui.vertical(|ui| {
                    ui.label(format!(
                        "Stats:\nError: {:.5}\nDivisions: {}\nDuration: {}us",
                        self.current_epsilon,
                        self.n_divisions,
                        self.calculations_duration.as_micros()
                    ));
                });
            });

            Plot::new("my_plot")
                .legend(Legend::default().position(Corner::RightBottom))
                .show(ui, |plot_ui| {
                    let make_line = |points_vec| {
                        let plot_points = PlotPoints::from(points_vec);
                        Line::new(plot_points)
                    };
                    plot_ui.line(
                        make_line(self.points_approximated.clone())
                            .color(DEFAULT_SETTINGS.approximate_plot_color)
                            .width(3.0)
                            .name("Approximated"),
                    );
                    plot_ui.line(
                        make_line(self.points_exact.clone())
                            .color(DEFAULT_SETTINGS.exact_plot_color)
                            .name("Exact"),
                    );
                });
        });
        self.recalculate_graph();
    }
}
