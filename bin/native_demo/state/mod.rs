use eframe::epaint::Color32;

#[derive(PartialEq)]
pub struct ClockQuartz {
    pub drift: f64,
    pub skew_min: i64,
    pub skew_sec: i64,
}

impl Default for ClockQuartz {
    fn default() -> Self {
        Self {
            drift: 1.0,
            skew_min: 0,
            skew_sec: 0,
        }
    }
}

pub struct Form {
    pub quartz_snapshot: ClockQuartz,
    pub chosen_clock_color: Color32,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            quartz_snapshot: ClockQuartz::default(),
            chosen_clock_color: Color32::from_rgb(146, 126, 238),
        }
    }
}

impl Form {
    pub fn system_clock_color(&self) -> Color32 {
        let color = &self.chosen_clock_color;
        Color32::from_rgb(color.r(), color.g(), color.b())
    }

    pub fn inverse_clock_color(&self) -> Color32 {
        let color = &self.chosen_clock_color;
        Color32::from_rgb(255 - color.r(), 255 - color.g(), 255 - color.b())
    }
}
