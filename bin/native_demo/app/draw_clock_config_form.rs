use super::TemplateApp;
use chrono::Duration;
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui::{Button, Color32, Grid, RichText, Slider, Ui};
use std::sync::atomic::Ordering;

impl TemplateApp {
    pub fn draw_clock_config_form(&mut self, ui: &mut Ui) {
        Grid::new("clock-config")
            .num_columns(1)
            .striped(true)
            .spacing([15.0, 15.0])
            .show(ui, |ui| {
                ui.heading("Clock Quartz");

                ui.end_row();

                ui.vertical(|ui| {
                    ui.add(
                        Slider::new(&mut self.resources.form.quartz_snapshot.drift, 0.1..=5.0)
                            .text("Drift Rate")
                            .fixed_decimals(2),
                    );
                });

                ui.end_row();

                ui.vertical(|ui| {
                    ui.label("Skewness");

                    ui.add(
                        Slider::new(&mut self.resources.form.quartz_snapshot.skew_min, -59..=59)
                            .text("Minutes")
                            .suffix(" min")
                            .custom_formatter(|n, _| format_with_sign(n as i64)),
                    );

                    ui.add(
                        Slider::new(&mut self.resources.form.quartz_snapshot.skew_sec, -59..=59)
                            .text("Seconds")
                            .suffix(" sec")
                            .custom_formatter(|n, _| format_with_sign(n as i64)),
                    );
                });

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    let simulation_button = ui.add_enabled(
                        self.resources.form.quartz_snapshot != self.quartz,
                        Button::new("Create Simulation"),
                    );

                    if simulation_button.clicked() {
                        self.quartz = self.resources.form.quartz_snapshot.clone();

                        self.system_clock.store(
                            Box::leak(Box::new(QuartzUtcClock::skewed_and_drifted(
                                Duration::minutes(self.quartz.skew_min)
                                    + Duration::seconds(self.quartz.skew_sec),
                                self.quartz.drift,
                            ))),
                            Ordering::Release,
                        );
                    }
                });

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    ui.color_edit_button_srgba(&mut self.resources.form.chosen_clock_color);
                });

                ui.end_row();

                ui.vertical_centered(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Drift Rate: ");
                        ui.label(
                            RichText::new(format!("{:.2}", self.quartz.drift))
                                .color(Color32::LIGHT_GRAY),
                        );
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Skew Minutes: ");
                        ui.label(
                            RichText::new(format_with_sign(self.quartz.skew_min))
                                .color(Color32::LIGHT_GRAY),
                        );
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label("Skew Seconds: ");
                        ui.label(
                            RichText::new(format_with_sign(self.quartz.skew_sec))
                                .color(Color32::LIGHT_GRAY),
                        );
                    });
                });

                ui.end_row();
            });
    }
}

fn format_with_sign(num: i64) -> String {
    format!("{}{:02}", if num >= 0 { '+' } else { '-' }, num.abs())
}
