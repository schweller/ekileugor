use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Controllable{
    pub current: bool
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActiveEntity {
    pub target: Entity
}
