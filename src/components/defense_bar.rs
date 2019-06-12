use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;


pub struct DefenseBar {
    pub x_pos: f32,
    pub y_pos: f32,
    pub defense_stack: Vec<Entity>,
}

impl Component for DefenseBar {
    type Storage = DenseVecStorage<Self>;
}