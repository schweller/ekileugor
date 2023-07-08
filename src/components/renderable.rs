use specs::prelude::*;
use specs_derive::*;

use bracket_lib::terminal::FontCharType;
use bracket_lib::color::RGB;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32
}
