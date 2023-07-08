use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Mob{}

#[derive(Component)]
pub struct Name{
    pub name: String    
}
