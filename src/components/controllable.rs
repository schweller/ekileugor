use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Controllable{
    pub current: bool
}

pub struct ActiveEntity {
    pub target: Entity
}
