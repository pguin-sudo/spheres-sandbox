pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

pub const NUM_CIRCLES: u32 = 0;
pub const NUM_STATIC_CIRCLES: u32 = 0;
pub const PHYSICS_MARGIN: u32 = 0;

// Random circles
pub const MIN_MASS: f32 = 100.0;
pub const MAX_MASS: f32 = 500.0;

// Physics
pub const TIME_SPEED: f32 = 0.1;

use std::sync::{LazyLock, Mutex};

static GRAVITATIONAL_ACCELERATION: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(10.0));

pub fn get_gravity() -> f32 {
    *GRAVITATIONAL_ACCELERATION.lock().unwrap()
}

pub fn add_gravity(n: f32) {
    *GRAVITATIONAL_ACCELERATION.lock().unwrap() += n;
}
