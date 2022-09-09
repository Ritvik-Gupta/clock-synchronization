use super::TemplateApp;
use eframe::{
    egui::{Color32, Painter, Pos2, Shape, Stroke, Vec2},
    epaint::PathShape,
};
use std::sync::atomic::Ordering;

const MILLISECOND_HAND_SIZE: f32 = 200.0;
const SECOND_HAND_SIZE: f32 = MILLISECOND_HAND_SIZE - 10.0;
const MINUTE_HAND_SIZE: f32 = SECOND_HAND_SIZE - 10.0;

impl TemplateApp {
    pub fn draw_clock_hands(&self, painter: &Painter, center: Pos2, is_system_clock: bool) {
        let (clock, color) = if is_system_clock {
            (
                unsafe { &*self.system_clock.load(Ordering::Acquire) }.clone(),
                self.resources
                    .form
                    .system_clock_color()
                    .linear_multiply(0.25),
            )
        } else {
            (
                self.actual_clock.clone(),
                self.resources
                    .form
                    .inverse_clock_color()
                    .linear_multiply(0.25),
            )
        };

        painter.add(Shape::Path(PathShape {
            points: (0..=(clock.millisecond() as f32 * (360.0 / 1000.0)) as u32)
                .map(|angle| (angle as f32 - 90.0).to_radians())
                .map(|angle| Vec2::new(angle.cos(), angle.sin()) * MILLISECOND_HAND_SIZE)
                .map(|offset| center + offset)
                .collect(),
            closed: false,
            stroke: Stroke::new(1.0, color),
            fill: Color32::TRANSPARENT,
        }));

        painter.add(Shape::Path(PathShape {
            points: (0..=clock.second() * (360 / 60))
                .map(|angle| (angle as f32 - 90.0).to_radians())
                .map(|angle| Vec2::new(angle.cos(), angle.sin()) * SECOND_HAND_SIZE)
                .map(|offset| center + offset)
                .collect(),
            closed: false,
            stroke: Stroke::new(3.0, color),
            fill: Color32::TRANSPARENT,
        }));

        painter.add(Shape::Path(PathShape {
            points: std::iter::repeat(center)
                .take(1)
                .chain(
                    (0..=clock.minute() * (360 / 60))
                        .map(|angle| (angle as f32 - 90.0).to_radians())
                        .map(|angle| Vec2::new(angle.cos(), angle.sin()) * MINUTE_HAND_SIZE)
                        .map(|offset| center + offset),
                )
                .chain(std::iter::repeat(center).take(1))
                .collect(),
            closed: true,
            stroke: Stroke::none(),
            fill: color,
        }));
    }
}
