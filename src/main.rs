extern crate amethyst;
#[cfg(test)]
extern crate avow;
extern crate dot_vox;
#[macro_use]
extern crate lazy_static;

use amethyst::prelude::*;
use amethyst::core::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use state::Example;
use amethyst::LoggerConfig;

mod dot_vox_format;
mod state;

fn run() -> Result<(), amethyst::Error> {
    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
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
