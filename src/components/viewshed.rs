use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<bracket_lib::geometry::Point>,
    pub range: i32,
    pub dirty: bool
}
