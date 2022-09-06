use super::state::{ClockQuartz, Form};
use chrono::{Duration, NaiveTime, Timelike, Utc};
use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, Context, Event, Grid, Key, Layout, Painter, Pos2,
    RichText, Sense, Shape, SidePanel, Slider, Stroke, TopBottomPanel, Vec2,
};
use egui_extras::{Size, TableBuilder};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    #[cfg_attr(feature = "persistence", serde(skip))]
    system_clock: NaiveTime,
    #[cfg_attr(feature = "persistence", serde(skip))]
    actual_clock: NaiveTime,
    quartz: ClockQuartz,
    form: Form,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            system_clock: Utc::now().time(),
            actual_clock: Utc::now().time(),
            form: Form::default(),
            quartz: ClockQuartz::default(),
        }
    }
}

const MILLISECOND_HAND_SIZE: f32 = 200.0;
const SECOND_HAND_SIZE: f32 = MILLISECOND_HAND_SIZE - 10.0;
const MINUTE_HAND_SIZE: f32 = SECOND_HAND_SIZE - 10.0;

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }

    fn draw_clock_hands(&self, painter: &Painter, center: Pos2, is_system_clock: bool) {
        let (clock, color) = if is_system_clock {
            (
                self.system_clock,
                self.form.system_clock_color().linear_multiply(0.25),
            )
        } else {
            (
                self.actual_clock,
                self.form.inverse_clock_color().linear_multiply(0.25),
            )
        };

        painter.add(Shape::Path(eframe::epaint::PathShape {
            points: (0..=(clock.nanosecond() as f32 * (360.0 / 10e8)) as u32)
                .map(|angle| (angle as f32 - 90.0).to_radians())
                .map(|angle| Vec2::new(angle.cos(), angle.sin()) * MILLISECOND_HAND_SIZE)
                .map(|offset| center + offset)
                .collect(),
            closed: false,
            stroke: Stroke::new(1.0, color),
            fill: Color32::TRANSPARENT,
        }));

        painter.add(Shape::Path(eframe::epaint::PathShape {
            points: (0..=clock.second() * (360 / 60))
                .map(|angle| (angle as f32 - 90.0).to_radians())
                .map(|angle| Vec2::new(angle.cos(), angle.sin()) * SECOND_HAND_SIZE)
                .map(|offset| center + offset)
                .collect(),
            closed: false,
            stroke: Stroke::new(3.0, color),
            fill: Color32::TRANSPARENT,
        }));

        painter.add(Shape::Path(eframe::epaint::PathShape {
            points: std::iter::repeat(center)
                .take(1)
                .chain(
                    (0..=clock.minute() * (360 / 60))
                        .map(|angle| (angle as f32 - 90.0).to_radians())
                        .map(|angle| Vec2::new(angle.cos(), angle.sin()) * MINUTE_HAND_SIZE)
                        .map(|offset| center + offset),
                )
                .chain(std::iter::repeat(center).take(1))
                .collect(),
            closed: true,
            stroke: Stroke::none(),
            fill: color,
        }));
    }
}

impl eframe::App for TemplateApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::style::Visuals::dark());

        ctx.input().events.iter().for_each(|event| match event {
            Event::Key {
                key: Key::ArrowUp, ..
            } => {
                println!("Arrow Up");
            }
            _ => {}
        });

        SidePanel::left("config-panel")
            .resizable(false)
            .show(ctx, |ui| {
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
                                self.form.quartz_snapshot = ClockQuartz::default();
                                self.system_clock += Duration::minutes(self.quartz.skew_min)
                                    + Duration::seconds(self.quartz.skew_sec);

                                //TODO When only updating Quartz Skew the Drfit resets back to 1.0
                                //TODO Update to fix this issue
                            }
                        });

                        ui.end_row();

                        ui.vertical_centered_justified(|ui| {
                            ui.color_edit_button_srgba(&mut self.form.chosen_clock_color);
                        });
                    });
            });

        CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
            let rect = response.rect;

            self.draw_clock_hands(&painter, rect.center(), true);
            self.draw_clock_hands(&painter, rect.center(), false);
        });

        TopBottomPanel::bottom("clock-panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(Layout::left_to_right(Align::Center))
                        .columns(Size::relative(0.5), 2)
                        .resizable(false)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("System Clock");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("Actual Clock");
                                });
                            });
                        })
                        .body(|mut body| {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.code(
                                            RichText::new(format!(
                                                " {:02} : {:02} : {:02} . {:04} ",
                                                self.system_clock.hour(),
                                                self.system_clock.minute(),
                                                self.system_clock.second(),
                                                self.system_clock.nanosecond() / 100000
                                            ))
                                            .color(self.form.system_clock_color()),
                                        );
                                    });
                                });

                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.code(
                                            RichText::new(format!(
                                                " {:02} : {:02} : {:02} . {:04} ",
                                                self.actual_clock.hour(),
                                                self.actual_clock.minute(),
                                                self.actual_clock.second(),
                                                self.actual_clock.nanosecond() / 100000
                                            ))
                                            .color(self.form.inverse_clock_color()),
                                        );
                                    });
                                });
                            })
                        });
                });
            });

        let now = Utc::now().time();

        self.system_clock += Duration::from_std(
            (now - self.actual_clock)
                .to_std()
                .unwrap()
                .mul_f64(self.quartz.drift),
        )
        .unwrap();

        self.actual_clock = now;

        ctx.request_repaint();
    }
}
