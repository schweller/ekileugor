
use bracket_lib::terminal::RGB;
use specs::prelude::*;

use crate::components::*;

pub fn player(ecs: &mut World, x: i32, y:i32) -> Entity {
    ecs
        .create_entity()
        .with(Player{})
        .with(Controllable{ current: true})
        .with(Position{ x, y})
        .with(Name{name: "Player".to_string() })
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 30, dirty: true })
        .with(Renderable{
            glyph: bracket_lib::terminal::to_cp437('@'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
            render_order: 0
        })
        .with(CombatStats{
            attack: 10,
            defense: 10,
            evade: 0
        })
        .with(PoolStats{
            hp: SinglePoolStat { current: 25, max: 25 },
            xp: 0,
            level: 1,
            gold: 0
        })
        .build()
}