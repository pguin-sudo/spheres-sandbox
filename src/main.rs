use std::{collections::HashSet, time::Instant};

use nalgebra::Point2;
use piston_window::*;

mod game;
mod physics;
mod settings;
mod utils;

use game::Game;
use settings::{SCREEN_HEIGHT, SCREEN_WIDTH};
use utils::Random;

use crate::settings::{add_gravity, get_gravity};

fn main() {
    let mut game = Game::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("RubikMonoOne.ttf");

    let mut window: PistonWindow =
        WindowSettings::new("Sphere Sandbox", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .resizable(false)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(
        font,
        TextureContext {
            factory,
            encoder: window.factory.create_command_buffer().into(),
        },
        TextureSettings::new(),
    )
    .unwrap();

    let random = Random::new();

    // Buttons
    let mut modifiers = HashSet::<Button>::new();
    let mut mousepos_args = [0_f64, 0_f64];

    let mut last_physics_time = Instant::now();
    let mut last_draw_time = Instant::now();

    while let Some(event) = window.next() {
        match event {
            // Input
            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Mouse(MouseButton::Right),
                    ..
                }),
                _,
            ) => {
                game.physics_engine.add_circle(random.get_random_circle(
                    Some(Point2::new(
                        mousepos_args[0] as f32,
                        mousepos_args[1] as f32,
                    )),
                    None,
                    None,
                    modifiers.contains(&Button::Keyboard(Key::LCtrl)),
                ));
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(Key::R),
                    ..
                }),
                _,
            ) => {
                game = Game::new();
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(Key::Up),
                    ..
                }),
                _,
            ) => {
                add_gravity(1.0);
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Release,
                    button: Button::Keyboard(Key::Up),
                    ..
                }),
                _,
            ) => {
                add_gravity(0.0);
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(Key::Down),
                    ..
                }),
                _,
            ) => {
                add_gravity(-1.0);
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Release,
                    button: Button::Keyboard(Key::Down),
                    ..
                }),
                _,
            ) => {
                add_gravity(0.0);
            }

            Event::Input(Input::Move(Motion::MouseCursor(new_coords)), _) => {
                mousepos_args = new_coords;
            }

            Event::Input(Input::Button(ButtonArgs { state, button, .. }), _) => {
                if state == ButtonState::Press {
                    modifiers.insert(button);
                } else {
                    modifiers.remove(&button);
                }
            }

            // Loop
            Event::Loop(_) => {
                let current_physics_time = Instant::now();
                let delta_physics_time = current_physics_time
                    .duration_since(last_physics_time)
                    .as_secs_f32();
                last_physics_time = current_physics_time;

                game.physics_engine.update(delta_physics_time);

                window.draw_2d(&event, |ctx, g, device| {
                    let current_draw_time = Instant::now();
                    let delta_draw_time = current_draw_time
                        .duration_since(last_draw_time)
                        .as_secs_f32();
                    last_draw_time = current_draw_time;

                    clear([0.0, 0.0, 0.0, 1.0], g);
                    game.draw(&ctx, g);

                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15)
                        .draw(
                            &format!(
                                "TPS: {}; FPS: {}, Gravity: {}, Objects: {}",
                                (1.0 / delta_physics_time).round() as u32,
                                (1.0 / delta_draw_time).round() as u32,
                                get_gravity(),
                                game.physics_engine.get_objects_amount()
                            ),
                            &mut glyphs,
                            &ctx.draw_state,
                            ctx.transform.trans(15.0, 15.0),
                            g,
                        )
                        .unwrap();

                    glyphs.factory.encoder.flush(device);
                });
            }

            _ => {}
        }
    }
}
