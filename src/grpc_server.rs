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
        let mut clock = unsafe { &*self.system_clock.load(Ordering::Acquire) }.clone();
        clock.tick_time();

        let response = Response::new(CristianTimeResponse {
            hours: clock.hour(),
            minutes: clock.minute(),
            seconds: clock.second(),
            milliseconds: clock.millisecond(),
        });

        self.system_clock
            .store(Box::leak(Box::new(clock)), Ordering::Release);

        Ok(response)
    }
}
