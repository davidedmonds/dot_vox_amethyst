use amethyst::core::Transform;
use amethyst::core::cgmath::{Deg, Vector3};
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use components::Name;

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