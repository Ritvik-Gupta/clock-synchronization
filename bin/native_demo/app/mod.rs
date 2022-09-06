mod draw_clock_config_form;
mod draw_clock_hands;
mod draw_clock_table;

use crate::state::{ClockQuartzConfig, Form};
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui::{self, CentralPanel, Context, Event, Key, Sense, SidePanel, TopBottomPanel};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct TemplateApp {
    #[cfg_attr(feature = "persistence", serde(skip))]
    system_clock: QuartzUtcClock,
    #[cfg_attr(feature = "persistence", serde(skip))]
    actual_clock: QuartzUtcClock,
    quartz: ClockQuartzConfig,
    form: Form,
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
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
            .show(ctx, |ui| self.draw_clock_config_form(ui));

        CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
            let rect = response.rect;

            self.draw_clock_hands(&painter, rect.center(), true);
            self.draw_clock_hands(&painter, rect.center(), false);
        });

        TopBottomPanel::bottom("clock-panel")
            .resizable(false)
            .show(ctx, |ui| self.draw_clock_table(ui));

        self.actual_clock.tick_time();
        self.system_clock.tick_time();

        ctx.request_repaint();
    }
}
