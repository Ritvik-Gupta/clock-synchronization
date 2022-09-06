pub mod quartz_clock;

#[cfg(test)]
mod test {
    use chrono::{Duration as CDuration, Utc};
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    const CLOCK_DRIFT_SPEED: f64 = 1.2;

    #[test]
    fn drifted_clock_should_be_ahead() {
        let instant = Instant::now();
        let then = Utc::now();

        sleep(Duration::from_millis(500));
        let elapsed = instant.elapsed();

        let drifted_time = then + CDuration::from_std(elapsed.mul_f64(CLOCK_DRIFT_SPEED)).unwrap();
        let actual_time = Utc::now();

        assert!(drifted_time > actual_time);
    }
}
