use clock_synchronization::grpc_server::clock::CristianTimeRequest;
use eframe::egui::{Button, Grid, RichText, Ui};

use crate::state::use_state::State;

use super::TemplateApp;

impl TemplateApp {
    pub fn draw_slave_operations(&mut self, ui: &mut Ui) {
        Grid::new("clock-config")
            .num_columns(1)
            .striped(true)
            .spacing([15.0, 15.0])
            .show(ui, |ui| {
                ui.heading("Slave Operations");

                ui.end_row();

                ui.vertical_centered_justified(|ui| {
                    let click_state = &mut self.states.cristian_time_click;
                    let hover_state = &mut self.states.cristian_time_hover;

                    let cristian_time_query_button =
                        ui.add_enabled(!click_state.is_pending(), Button::new("Cristian Time"));

                    if cristian_time_query_button.hovered() {
                        hover_state.use_future({
                            let mut client_conn = self.client_conn.clone();
                            async move {
                                client_conn
                                    .cristian_time(CristianTimeRequest {})
                                    .await
                                    .unwrap()
                                    .into_inner()
                            }
                        });
                    }

                    if cristian_time_query_button.clicked() {
                        click_state.clear();
                        click_state.use_future({
                            let mut client_conn = self.client_conn.clone();
                            async move {
                                client_conn
                                    .cristian_time(CristianTimeRequest {})
                                    .await
                                    .unwrap()
                                    .into_inner()
                            }
                        });
                    }

                    if let State::Ready(result) = &hover_state {
                        cristian_time_query_button.on_hover_text_at_pointer(
                            RichText::new(format!(
                                " {:02} : {:02} : {:02} . {:03} ",
                                result.hours, result.minutes, result.seconds, result.milliseconds
                            ))
                            .color(self.resources.form.system_clock_color()),
                        );
                    }
                });

                ui.end_row();
            });
    }
}
