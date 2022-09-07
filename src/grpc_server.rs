pub mod clock {
    tonic::include_proto!("clock");
}

use self::clock::{sync_clock_server::SyncClock, CristianTimeRequest, CristianTimeResponse};
use crate::quartz_clock::QuartzUtcClock;
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct SyncClockService {
    system_clock: Arc<AtomicPtr<QuartzUtcClock>>,
}

impl SyncClockService {
    pub fn with_clock(system_clock: Arc<AtomicPtr<QuartzUtcClock>>) -> Self {
        Self { system_clock }
    }
}

#[tonic::async_trait]
impl SyncClock for SyncClockService {
    async fn cristian_time(
        &self,
        _request: Request<CristianTimeRequest>,
    ) -> Result<Response<CristianTimeResponse>, Status> {
        let time = unsafe { &mut *self.system_clock.load(Ordering::Relaxed) };
        time.tick_time();

        Ok(Response::new(CristianTimeResponse {
            hours: time.hour(),
            minutes: time.minute(),
            seconds: time.second(),
            milliseconds: time.millisecond(),
        }))
    }
}
