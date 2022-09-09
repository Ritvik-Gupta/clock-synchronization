use std::sync::atomic::Ordering;

#[cfg(feature = "slave")]
use chrono::NaiveTime;

use super::TemplateApp;

impl TemplateApp {
    pub fn update_states(&mut self) {
        #[cfg(feature = "slave")]
        self.states.cristian_time_hover.update();
        #[cfg(feature = "slave")]
        self.states.cristian_time_click.update();

        #[cfg(feature = "slave")]
        if let Some(result) = self.states.cristian_time_click.take_and_clear() {
            let server_time = NaiveTime::from_hms_milli(
                result.hours,
                result.minutes,
                result.seconds,
                result.milliseconds,
            );

            self.system_clock
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |clock| {
                    let mut clock = unsafe { &*clock }.clone();
                    clock.set_time(server_time);
                    Some(Box::leak(Box::new(clock)))
                })
                .ok();
        }

        self.actual_clock.tick_time();
        self.system_clock
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |clock| {
                let mut clock = unsafe { &*clock }.clone();
                clock.tick_time();
                Some(Box::leak(Box::new(clock)))
            })
            .ok();
    }
}
