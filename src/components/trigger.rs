use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Clone)]
pub struct Hidden{}

#[derive(Component, Clone)]
pub struct EntryTrigger{}

#[derive(Component, Clone)]
pub struct EntityMoved{}