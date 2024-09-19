use nalgebra::{ Point2, Vector2 };
use piston_window::*;
use rand::Rng;

use crate::physics::{ Circle, Engine };
use crate::settings::{ NUM_CIRCLES, SCREEN_HEIGHT, SCREEN_WIDTH };

pub struct Random;

impl Random {
    pub fn get_instance() -> Self {
        Random
    }

    pub fn get_random_circle(&self) -> Circle {
        let mut rng = rand::thread_rng();
        Circle::new(
            Point2::new(
                rng.gen_range(0..SCREEN_WIDTH) as f32,
                rng.gen_range(0..SCREEN_HEIGHT) as f32
            ),
            Vector2::new(0 as f32, 0 as f32),
            0.5_f32,
            rng.gen::<f32>() * 10_f32,
            10_f32,
            [rng.gen(), rng.gen(), rng.gen(), 1.0],
            false
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
            self.physics_engine.add_circle(random.get_random_circle());
        }
    }

    pub fn draw(&mut self, context: &Context, g: &mut G2d) {
        for circle in self.physics_engine.get_circles() {
            let circle_shape = Ellipse::new(circle.color);
            circle_shape.draw(
                [
                    (circle.position[0] - circle.radius) as f64,
                    (circle.position[1] - circle.radius) as f64,
                    (circle.radius * 2_f32) as f64,
                    (circle.radius * 2_f32) as f64,
                ],
                &DrawState::default(),
                context.transform,
                g
            );
        }
    }
}
