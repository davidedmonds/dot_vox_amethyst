use amethyst::{GameData, State, StateData, Trans};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::renderer::{Camera, Event, KeyboardInput, MaterialDefaults, Mesh, Projection,
                         WindowEvent, VirtualKeyCode};
use amethyst::prelude::*;

use components::Name;

use dot_vox_format::DotVoxFormat;

pub struct Example;

const ARENA_HEIGHT: f32 = 1000.0;
const ARENA_WIDTH: f32 = 1000.0;

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
