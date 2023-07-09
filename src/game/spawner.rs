
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::*;
use specs::prelude::*;

use crate::map::{Rect, MAPWIDTH};
use crate::components::*;

const MAX_MONSTERS : i32 = 4;
const MAX_ITEMS : i32 = 2;

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

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points : Vec<usize> = Vec::new();
    let mut item_spaw_points : Vec<usize> = Vec::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS +2) - 3;

        for _i in 0 .. num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        for _i in 0 .. num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spaw_points.contains(&idx) {
                    item_spaw_points.push(idx);
                    added = true;
                }
            }            
        }
    }

    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_monster(ecs, x as i32, y as i32);
    }

    for idx in item_spaw_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_item(ecs, x as i32, y as i32);
    }    
}

pub fn random_monster(ecs : &mut World, x : i32, y : i32) {
    let roll :i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }
    match roll {
        1 => { orc(ecs, x, y) }
        _ => { goblin(ecs, x, y) }
    }    
}

fn orc(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, to_cp437('o'), "Orc"); }
fn goblin(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, to_cp437('g'), "Goblin"); }

fn monster(ecs: &mut World, x: i32, y:i32, glyph: FontCharType, name : &str) {
    ecs
        .create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: glyph,
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
            render_order: 1,
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Mob{})
        .with(Name{ name : name.to_string() })
        .with(BlocksTile{})
        .with(CombatStats{
            attack: 5,
            defense: 5,
            evade: 0
        })
        .with(PoolStats{
            hp: SinglePoolStat { current: 8, max: 8 },
            xp: 0,
            level: 1,
            gold: 0
        })
        .build();  
}

fn random_item(ecs: &mut World, x: i32, y: i32) {
    let roll :i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 4);
    }
    match roll {
        _ => { health_potion(ecs, x, y) }
    }
}

fn health_potion(ecs: &mut World, x : i32, y : i32) {
    ecs
        .create_entity()
        .with(Position{x, y})
        .with(Renderable{
            glyph: to_cp437('¡'),
            fg: RGB::named(MAGENTA),
            bg: RGB::named(BLACK),
            render_order: 2
        })
        .with(Name{ name: "Health Potion".to_string()})
        .with(Item{})
        .with(Consumable{ charges: 1})
        .with(Heals{
            amount: 8
        })
        .build();
}