pub mod clock {
    tonic::include_proto!("clock");
}

use self::clock::{sync_clock_server::SyncClock, CristianTimeRequest, CristianTimeResponse};
use crate::quartz_clock::QuartzUtcClock;
use std::{
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
    time::Duration,
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
        let mut time = unsafe { &*self.system_clock.load(Ordering::Acquire) }.clone();
        time.tick_time();

        let response = Response::new(CristianTimeResponse {
            hours: time.hour(),
            minutes: time.minute(),
            seconds: time.second(),
            milliseconds: time.millisecond(),
        });

        self.system_clock
            .store(Box::leak(Box::new(time)), Ordering::Release);

        tokio::time::sleep(Duration::from_secs(3)).await;

        Ok(response)
    }
}
