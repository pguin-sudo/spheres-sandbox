use piston_window::*;
use std::time::Instant;

mod physics;
mod settings;
mod game;

use settings::{ SCREEN_WIDTH, SCREEN_HEIGHT };
use game::Game;

fn main() {
    let mut game = Game::new();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let ref font = assets.join("RubikMonoOne.ttf");

    let mut window: PistonWindow = WindowSettings::new("Sphere Sandbox", [
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    ])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(
        font,
        TextureContext {
            factory,
            encoder: window.factory.create_command_buffer().into(),
        },
        TextureSettings::new()
    ).unwrap();

    let mut last_update = Instant::now();
    while let Some(event) = window.next() {
        let delta_time = Instant::now().duration_since(last_update).as_secs_f32();

        window.draw_2d(&event, |ctx, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.draw(&ctx, g);
            text::Text
                ::new_color([1.0, 1.0, 1.0, 1.0], 15)
                .draw(
                    &(1_f32 / delta_time).to_string(),
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(15_f64, 15_f64),
                    g
                )
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });

        game.physics_engine.update(delta_time);
        last_update = Instant::now();
    }
}
