use amethyst::ecs::{Component, DenseVecStorage};

pub struct Name(pub &'static str);

impl Component for Name {
    type Storage = DenseVecStorage<Self>;
}