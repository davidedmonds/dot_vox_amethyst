extern crate amethyst;
#[cfg(test)]
extern crate avow;
#[macro_use]
extern crate derivative;
extern crate dot_vox;
#[macro_use]
extern crate glsl_layout;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate gfx_core;

use amethyst::LoggerConfig;
use amethyst::prelude::*;
use amethyst::core::TransformBundle;
use amethyst::renderer::{DisplayConfig, Pipeline, PosColor, RenderBundle, Stage};
use amethyst::utils::fps_counter::FPSCounterBundle;
use renderer::DrawVoxels;
use state::Example;
use systems::RotationSystem;

mod components;
mod dot_vox_format;
mod renderer;
mod state;
mod systems;

fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawVoxels::<PosColor>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(FPSCounterBundle::default())?
        .with(RotationSystem, "rotation_system", &[]);
    let mut game = Application::build("./", Example)?
        .build(game_data)?;
    game.run();
    Ok(())
}

fn main() {
    amethyst::start_logger(LoggerConfig::default());
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
