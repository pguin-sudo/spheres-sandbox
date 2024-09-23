use std::f32::consts::PI;

use nalgebra::{ Point2, Vector2 };
use piston_window::*;
use rand::Rng;

use crate::physics::{ Circle, Engine };
use crate::settings::{ MAX_MASS, NUM_CIRCLES, NUM_STATIC_CIRCLES, SCREEN_HEIGHT, SCREEN_WIDTH };

pub struct Random;

impl Random {
    pub fn get_instance() -> Self {
        Random
    }

    pub fn get_random_circle(
        &self,
        position_arg: Option<Point2<f32>>,
        mass_arg: Option<f32>,
        amortization_arg: Option<f32>,
        is_static: bool
    ) -> Circle {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let mass: f32 = mass_arg.unwrap_or_else(|| rng.gen::<f32>() * MAX_MASS);
        let amortization: f32 = amortization_arg.unwrap_or_else(|| rng.gen::<f32>() / 4.0 + 0.75);
        let position: Point2<f32> = position_arg.unwrap_or_else(||
            Point2::new(
                rng.gen_range(0..SCREEN_WIDTH) as f32,
                rng.gen_range(0..SCREEN_HEIGHT) as f32
            )
        );

        Circle::new(
            position,
            Vector2::new(0.0, 0.0),
            amortization,
            mass * 10.0,
            (mass / PI).sqrt() * 50.0,
            [(mass / MAX_MASS).min(1.0), 0.6, amortization, 1.0],
            is_static
        )
    }
}

pub struct Game {
    pub physics_engine: Engine,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            physics_engine: Engine::new(),
        };
        game.init();
        game
    }

    pub fn init(&mut self) {
        let random = Random::get_instance();
        for _ in 0..NUM_CIRCLES {
            self.physics_engine.add_circle(random.get_random_circle(None, None, None, false));
        }

        for _ in 0..NUM_STATIC_CIRCLES {
            self.physics_engine.add_circle(random.get_random_circle(None, None, None, true));
        }
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d) {
        let circles = self.physics_engine.get_circles();
        for circle in circles {
            let diameter = circle.radius * 2.0;
            let rectangle = [
                (circle.position[0] - circle.radius) as f64,
                (circle.position[1] - circle.radius) as f64,
                diameter as f64,
                diameter as f64,
            ];
            Ellipse::new(circle.color).draw(rectangle, &DrawState::default(), context.transform, g);
        }
    }
}
