mod draw_clock_config_form;
mod draw_clock_hands;
mod draw_clock_table;
#[cfg(feature = "slave")]
mod draw_slave_operations;
mod update_states;

use crate::state::{ClockQuartzConfig, Form};
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui::{self, CentralPanel, Context, Event, Key, Sense, SidePanel, TopBottomPanel};
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};

#[cfg(feature = "slave")]
use crate::state::use_state::State;
#[cfg(feature = "slave")]
use clock_synchronization::grpc_server::clock::{
    sync_clock_client::SyncClockClient, CristianTimeResponse,
};
#[cfg(feature = "slave")]
use tonic::transport::Channel;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct Resourses {
    form: Form,
}

#[cfg(feature = "slave")]
#[derive(Default)]
pub struct States {
    cristian_time_hover: State<CristianTimeResponse>,
    cristian_time_click: State<CristianTimeResponse>,
}

pub struct TemplateApp {
    system_clock: Arc<AtomicPtr<QuartzUtcClock>>,
    actual_clock: QuartzUtcClock,
    quartz: ClockQuartzConfig,
    resources: Resourses,

    #[cfg(feature = "slave")]
    client_conn: SyncClockClient<Channel>,
    #[cfg(feature = "slave")]
    states: States,
}

impl TemplateApp {
    pub fn new(
        _cc: &eframe::CreationContext,
        system_clock: Arc<AtomicPtr<QuartzUtcClock>>,
        #[cfg(feature = "slave")] client_conn: SyncClockClient<Channel>,
    ) -> Self {
        #[allow(unused_mut)]
        let mut app = Self {
            system_clock,
            actual_clock: QuartzUtcClock::default(),
            quartz: ClockQuartzConfig::default(),
            resources: Resourses::default(),

            #[cfg(feature = "slave")]
            client_conn,
            #[cfg(feature = "slave")]
            states: States::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = _cc.storage {
            app.resources = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        app.system_clock.store(
            Box::leak(Box::new(QuartzUtcClock::default())),
            Ordering::Release,
        );

        app
    }
}

impl eframe::App for TemplateApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.resources);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        #[cfg(feature = "puffin-profile")]
        puffin::GlobalProfiler::lock().new_frame();

        #[cfg(feature = "puffin-profile")]
        puffin::profile_function!();

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

        #[cfg(feature = "slave")]
        SidePanel::right("client")
            .resizable(false)
            .show(ctx, |ui| self.draw_slave_operations(ui));

        TopBottomPanel::bottom("clock-panel")
            .resizable(false)
            .show(ctx, |ui| self.draw_clock_table(ui));

        CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
            let rect = response.rect;

            self.draw_clock_hands(&painter, rect.center(), true);
            self.draw_clock_hands(&painter, rect.center(), false);
        });

        self.update_states();
        ctx.request_repaint();
    }
}
