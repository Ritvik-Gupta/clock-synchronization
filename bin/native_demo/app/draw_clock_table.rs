use super::TemplateApp;
use eframe::{
    egui::{Layout, RichText, Ui},
    emath::Align,
};
use egui_extras::{Size, TableBuilder};
use std::sync::atomic::Ordering;

impl TemplateApp {
    pub fn draw_clock_table(&self, ui: &mut Ui) {
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
                            let system_clock =
                                unsafe { &*self.system_clock.load(Ordering::Acquire) }.clone();
                            ui.code(
                                RichText::new(format!(
                                    " {:02} : {:02} : {:02} . {:03} ",
                                    system_clock.hour(),
                                    system_clock.minute(),
                                    system_clock.second(),
                                    system_clock.millisecond()
                                ))
                                .color(self.resources.form.system_clock_color()),
                            );
                        });
                    });

                    row.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.code(
                                RichText::new(format!(
                                    " {:02} : {:02} : {:02} . {:03} ",
                                    self.actual_clock.hour(),
                                    self.actual_clock.minute(),
                                    self.actual_clock.second(),
                                    self.actual_clock.millisecond()
                                ))
                                .color(self.resources.form.inverse_clock_color()),
                            );
                        });
                    });
                })
            });
    }
}
