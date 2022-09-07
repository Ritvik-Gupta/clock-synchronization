mod draw_clock_config_form;
mod draw_clock_hands;
mod draw_clock_table;

use crate::state::{ClockQuartzConfig, Form};
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui::{self, CentralPanel, Context, Event, Key, Sense, SidePanel, TopBottomPanel};
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};

#[cfg(feature = "mode-client")]
use clock_synchronization::grpc_server::clock::sync_clock_client::SyncClockClient;
#[cfg(feature = "mode-client")]
use tonic::transport::Channel;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct TemplateApp {
    #[cfg_attr(feature = "persistence", serde(skip))]
    system_clock: Arc<AtomicPtr<QuartzUtcClock>>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    actual_clock: QuartzUtcClock,
    #[cfg_attr(feature = "persistence", serde(skip))]
    quartz: ClockQuartzConfig,

    #[cfg(feature = "mode-client")]
    #[cfg_attr(feature = "persistence", serde(skip))]
    client_conn: Option<SyncClockClient<Channel>>,

    form: Form,
}

impl TemplateApp {
    pub fn new(
        _cc: &eframe::CreationContext,
        system_clock: Arc<AtomicPtr<QuartzUtcClock>>,
        #[cfg(feature = "mode-client")] client_conn: Option<SyncClockClient<Channel>>,
    ) -> Self {
        #[allow(unused_mut)]
        let mut app = Self {
            system_clock: system_clock.clone(),
            actual_clock: QuartzUtcClock::default(),
            quartz: ClockQuartzConfig::default(),

            #[cfg(feature = "mode-client")]
            client_conn,

            form: Form::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).map(|cached_app| app = cached_app);
            app.system_clock = system_clock.clone();
        }

        app
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

        #[cfg(feature = "mode-client")]
        SidePanel::right("client")
            .resizable(false)
            .show(ctx, |ui| {});

        TopBottomPanel::bottom("clock-panel")
            .resizable(false)
            .show(ctx, |ui| self.draw_clock_table(ui));

        CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
            let rect = response.rect;

            self.draw_clock_hands(&painter, rect.center(), true);
            self.draw_clock_hands(&painter, rect.center(), false);
        });

        self.actual_clock.tick_time();
        unsafe { &mut *self.system_clock.load(Ordering::Relaxed) }.tick_time();
        ctx.request_repaint();
    }
}
