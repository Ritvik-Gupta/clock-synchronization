use chrono::{Duration, NaiveTime, Timelike, Utc};
use std::time::Instant;

pub struct QuartzUtcClock {
    current_time: NaiveTime,
    instant: Instant,
    drift_rate: f64,
}

impl Default for QuartzUtcClock {
    fn default() -> Self {
        Self::skewed_and_drifted(Duration::zero(), 1.0)
    }
}

impl QuartzUtcClock {
    pub fn skewed_and_drifted(skew: Duration, drift_rate: f64) -> Self {
        Self {
            current_time: Utc::now().time() + skew,
            instant: Instant::now(),
            drift_rate,
        }
    }

    pub fn drifted(drift_rate: f64) -> Self {
        Self::skewed_and_drifted(Duration::zero(), drift_rate)
    }

    pub fn tick_time(&mut self) {
        self.current_time +=
            Duration::from_std(self.instant.elapsed().mul_f64(self.drift_rate)).unwrap();
        self.instant = Instant::now();
    }

    pub fn hour(&self) -> u32 {
        self.current_time.hour()
    }

    pub fn minute(&self) -> u32 {
        self.current_time.minute()
    }

    pub fn second(&self) -> u32 {
        self.current_time.second()
    }

    pub fn millisecond(&self) -> u32 {
        self.current_time.nanosecond() / 1000000
    }
}

#[cfg(test)]
mod test {
    use super::QuartzUtcClock;
    use chrono::Duration;

    #[test]
    fn quartz_clocks_should_have_correct_bounds() {
        let mut quartz_clocks = [
            QuartzUtcClock::default(),
            QuartzUtcClock::skewed_and_drifted(Duration::seconds(10), 2.0),
            QuartzUtcClock::skewed_and_drifted(Duration::minutes(-5), 0.1),
            QuartzUtcClock::skewed_and_drifted(Duration::hours(-2), 1.0),
        ];

        quartz_clocks.iter_mut().for_each(|quartz_clock| {
            for _ in 0..1 << 20 {
                quartz_clock.tick_time();
                assert!(quartz_clock.hour() < 24);
                assert!(quartz_clock.minute() < 60);
                assert!(quartz_clock.second() < 60);
                assert!(quartz_clock.millisecond() < 1000);
            }
        });
    }
}
