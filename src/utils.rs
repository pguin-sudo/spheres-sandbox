use std::f32::consts::PI;

use nalgebra::{Point2, Vector2};
use rand::Rng;

use crate::{
    physics::Circle,
    settings::{MAX_MASS, MIN_MASS, SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub struct Random;

impl Random {
    pub fn new() -> Self {
        Random
    }

    pub fn get_random_circle(
        &self,
        position_arg: Option<Point2<f32>>,
        mass_arg: Option<f32>,
        amortization_arg: Option<f32>,
        is_static: bool,
    ) -> Circle {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let mass: f32 =
            mass_arg.unwrap_or_else(|| rng.gen::<f32>() * (MAX_MASS - MIN_MASS) + MIN_MASS);
        let amort_non_normalized: f32 = rng.gen::<f32>();
        let amortization: f32 =
            amortization_arg.unwrap_or_else(|| amort_non_normalized / 4.0 + 0.75);
        //let amortization: f32 = amortization_arg.unwrap_or_else(|| 0.9);
        let position: Point2<f32> = position_arg.unwrap_or_else(|| {
            Point2::new(
                rng.gen_range(0..SCREEN_WIDTH) as f32,
                rng.gen_range(0..SCREEN_HEIGHT) as f32,
            )
        });

        Circle::new(
            position,
            Vector2::new(0.0, 0.0),
            amortization,
            mass * 10.0,
            (mass / PI).sqrt(),
            [
                amort_non_normalized / 2.0 + 0.5,
                0.6,
                1.0 - (amort_non_normalized / 2.0 + 0.5),
                1.0,
            ],
            is_static,
        )
    }
}
