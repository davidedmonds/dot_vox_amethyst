extern crate amethyst;
extern crate dot_vox;
extern crate dot_vox_amethyst;
extern crate gfx_core;

use amethyst::LoggerConfig;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::{GlobalTransform, Transform, TransformBundle};
use amethyst::core::cgmath::{Deg, Matrix4, Vector3};
use amethyst::ecs::{Component, DenseVecStorage, Join, ReadStorage, System, WriteStorage};
use amethyst::prelude::*;
use amethyst::renderer::{Camera, DisplayConfig, Event, KeyboardInput, MaterialDefaults, Mesh,
                         Pipeline, PosColor, Projection, RenderBundle, Stage, WindowEvent,
                         VirtualKeyCode};
use amethyst::utils::fps_counter::FPSCounterBundle;
use dot_vox_amethyst::{DotVoxFormat, DrawVoxels};

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

pub struct Example;

const ARENA_HEIGHT: f32 = 10.0;
const ARENA_WIDTH: f32 = 10.0;

impl<'a, 'b> State<GameData<'a, 'b>> for Example {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let (mesh, material) = {
            let loader = world.read_resource::<Loader>();
            let mesh_storage = world.read_resource::<AssetStorage<Mesh>>();
            let mat_defaults = world.read_resource::<MaterialDefaults>();
            let mesh = loader.load(
                "resources/mesh/placeholder.vox",
                DotVoxFormat,
                Default::default(),
                (),
                &mesh_storage,
            );
            let material = mat_defaults.0.clone();
            (mesh, material)
        };

        let mut transform = Transform::default();
        transform.move_global(Vector3::new(5.0, 5.0, -5.0));

        world.create_entity()
            .with(Name("Model"))
            .with(mesh)
            .with(material)
            .with(transform)
            .with(GlobalTransform(Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0))))
            .build();

        initialise_camera(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

pub struct RotationSystem;

impl<'s> System<'s> for RotationSystem {
    type SystemData = (
        ReadStorage<'s, Name>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (names, mut transforms): Self::SystemData) {
        for (_name, mut transform) in (&names, &mut transforms).join() {
            transform.rotate_local(Vector3::new(1.0, 1.0, 0.0), Deg(1.0));
        }
    }
}

pub struct Name(pub &'static str);

impl Component for Name {
    type Storage = DenseVecStorage<Self>;
}
