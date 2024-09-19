use std::vec::Vec;

use nalgebra::{ distance, Point2, Vector2 };

use crate::settings::{ GRAVITATIONAL_ACCELERATION, PHYSICS_MARGIN, SCREEN_HEIGHT, SCREEN_WIDTH };

pub struct Circle {
    pub position: Point2<f32>,
    velocity: Vector2<f32>,
    amortization: f32,
    mass: f32,
    pub radius: f32,
    pub color: [f32; 4],
    is_static: bool,
}

impl Circle {
    pub fn new(
        position: Point2<f32>,
        velocity: Vector2<f32>,
        amortization: f32,
        mass: f32,
        radius: f32,
        color: [f32; 4],
        is_static: bool
    ) -> Self {
        Circle {
            position,
            velocity,
            amortization,
            mass,
            radius,
            color,
            is_static,
        }
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        self.velocity.x += force.x / self.mass;
        self.velocity.y += force.y / self.mass;
    }

    // fn apply_impulse(&mut self, impulse: Vector2<f32>) {
    //     self.velocity.x += impulse.x;
    //     self.velocity.y += impulse.y;
    // }

    fn apply_offset(&mut self, offset: Vector2<f32>) {
        if self.is_static {
            return;
        }
        self.position.x += offset.x;
        self.position.y += offset.y;
    }

    fn update(&mut self, delta_time: f32) {
        if self.is_static {
            return;
        }
        self.position.x += (self.velocity.x * delta_time) / 10.0;
        self.position.y += (self.velocity.y * delta_time) / 10.0;
    }
}

pub struct Engine {
    circles: Vec<Circle>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            circles: Vec::new(),
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }

    pub fn get_circles(&self) -> &Vec<Circle> {
        &self.circles
    }

    pub fn update(&mut self, delta_time: f32) {
        let len = self.circles.len();
        for i in 0..len {
            let (checked, others) = self.circles.split_at_mut(i + 1);
            let circle = checked.get_mut(i).expect("Empty circle");

            // Handle wall collisions
            let radius = circle.radius;

            if circle.position.x - radius < (PHYSICS_MARGIN as f32) {
                circle.position.x = radius + (PHYSICS_MARGIN as f32);
                circle.velocity.x *= -circle.amortization;
            }

            if circle.position.x + radius > ((SCREEN_WIDTH - PHYSICS_MARGIN) as f32) {
                circle.position.x = ((SCREEN_WIDTH - PHYSICS_MARGIN) as f32) - radius;
                circle.velocity.x *= -circle.amortization;
            }

            if circle.position.y - radius < (PHYSICS_MARGIN as f32) {
                circle.position.y = radius + (PHYSICS_MARGIN as f32);
                circle.velocity.y *= -circle.amortization;
            }

            if circle.position.y + radius > ((SCREEN_HEIGHT - PHYSICS_MARGIN) as f32) {
                circle.position.y = ((SCREEN_HEIGHT - PHYSICS_MARGIN) as f32) - radius;
                circle.velocity.y *= -circle.amortization;
            }

            // Handle circle collisions
            for other_circle in others {
                let distance = distance(&circle.position, &other_circle.position);
                let collision_distance = circle.radius + other_circle.radius;

                if distance < collision_distance {
                    let normal = Vector2::new(
                        other_circle.position.x - circle.position.x,
                        other_circle.position.y - circle.position.y
                    ).normalize();
                    let relative_velocity = Vector2::new(
                        other_circle.velocity.x - circle.velocity.x,
                        other_circle.velocity.y - circle.velocity.y
                    );
                    let dot_product = relative_velocity.dot(&normal);

                    if dot_product < 0.0 {
                        let impulse_mag =
                            (-(1.0 + circle.amortization) * dot_product) /
                            (1.0 / circle.mass + 1.0 / other_circle.mass);

                        // Apply impulse
                        let impulse = Vector2::new(normal.x * impulse_mag, normal.y * impulse_mag);

                        circle.velocity.x -= impulse.x / circle.mass;
                        circle.velocity.y -= impulse.y / circle.mass;
                        other_circle.velocity.x += impulse.x / other_circle.mass;
                        other_circle.velocity.y += impulse.y / other_circle.mass;

                        // Separate circles to avoid overlap
                        let overlap = collision_distance - distance;
                        let separation_vector = Vector2::new(
                            normal.x * overlap * 0.5,
                            normal.y * overlap * 0.5
                        );

                        circle.apply_offset(
                            Vector2::new(-separation_vector.x, -separation_vector.y)
                        );
                        other_circle.apply_offset(separation_vector);
                    }
                }
            }

            circle.apply_force(Vector2::new(0.0, GRAVITATIONAL_ACCELERATION * circle.mass));

            // Update circle position based on velocity
            circle.update(delta_time);
        }
    }
}
