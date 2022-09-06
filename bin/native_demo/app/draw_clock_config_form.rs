use super::TemplateApp;
use crate::state::ClockQuartzConfig;
use chrono::Duration;
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui::{Button, Grid, Slider, Ui};

impl TemplateApp {
    pub fn draw_clock_config_form(&mut self, ui: &mut Ui) {
        Grid::new("clock-config")
            .num_columns(1)
            .striped(true)
            .spacing([15.0, 15.0])
            .show(ui, |ui| {
                ui.heading("Clock Quartz");

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    ui.add(
                        Slider::new(&mut self.form.quartz_snapshot.drift, 0.1..=5.0)
                            .text("Drift Rate")
                            .fixed_decimals(2),
                    );
                });

                ui.end_row();

                ui.vertical(|ui| {
                    ui.label("Skewness");

                    ui.add(
                        Slider::new(&mut self.form.quartz_snapshot.skew_min, -59..=59)
                            .text("Minutes")
                            .suffix(" min"),
                    );

                    ui.add(
                        Slider::new(&mut self.form.quartz_snapshot.skew_sec, -59..=59)
                            .text("Seconds")
                            .suffix(" sec"),
                    );
                });

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    let simulation_button = ui.add_enabled(
                        self.form.quartz_snapshot != self.quartz,
                        Button::new("Create Simulation"),
                    );

                    if simulation_button.clicked() {
                        std::mem::swap(&mut self.quartz, &mut self.form.quartz_snapshot);
                        self.form.quartz_snapshot = ClockQuartzConfig::default();

                        self.system_clock = QuartzUtcClock::skewed_and_drifted(
                            Duration::minutes(self.quartz.skew_min)
                                + Duration::seconds(self.quartz.skew_sec),
                            self.quartz.drift,
                        );

                        //TODO When only updating Quartz Skew the Drfit resets back to 1.0
                        //TODO Update to fix this issue
                    }
                });

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    ui.color_edit_button_srgba(&mut self.form.chosen_clock_color);
                });
            });
    }
}
