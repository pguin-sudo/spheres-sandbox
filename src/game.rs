use piston_window::*;

use crate::physics::Engine;
use crate::settings::{NUM_CIRCLES, NUM_STATIC_CIRCLES};
use crate::utils::Random;

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
        let random = Random::new();
        for _ in 0..NUM_CIRCLES {
            self.physics_engine
                .add_circle(random.get_random_circle(None, None, None, false));
        }

        for _ in 0..NUM_STATIC_CIRCLES {
            self.physics_engine
                .add_circle(random.get_random_circle(None, None, None, true));
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
