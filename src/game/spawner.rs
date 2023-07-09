
use std::collections::HashMap;

use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::*;
use specs::prelude::*;

use crate::map::{RoomRect, MAPWIDTH};
use crate::components::*;

const MAX_MONSTERS : i32 = 4;

pub fn player(ecs: &mut World, x: i32, y:i32) -> Entity {
    ecs
        .create_entity()
        .with(Player{})
        .with(Controllable{ current: true})
        .with(Position{ x, y})
        .with(Name{name: "Player".to_string() })
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles : Vec::new(), range : 8, dirty: true })
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

// Spawning entities functions -> random and non-random
pub fn spawn_room(ecs: &mut World, room: &RoomRect, depth: i32) {
    let spawn_table = room_random_table(depth);
    let mut spawn_points : HashMap<usize, String> = HashMap::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (depth - 1) - 3;

        for _i in 0 .. num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    for spawn in spawn_points.iter() {
        let x = (*spawn.0 % MAPWIDTH) as i32;
        let y = (*spawn.0 / MAPWIDTH) as i32;

        match spawn.1.as_ref() {
            "Goblin" => goblin(ecs, x, y),
            "Orc" => orc(ecs, x, y),
            "Health Potion" => health_potion(ecs, x, y),
            "Spike Trap" => spike_trap(ecs, x, y),
            _ => {}
        }
    }
}

fn room_random_table(depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + depth)
        .add("Health Potion", 7)
        .add("Spike Trap", 100 + depth)
}

// Spawnables
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
        .with(Controllable{ current: false})
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

fn spike_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{x, y})
        .with(Renderable{
            glyph: to_cp437('^'),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
            render_order: 2
        })
        .with(EntryTrigger{})
        .with(InflictsDamage{ amount: 6})
        .with(Name{name: "Spike Trap".to_string()})
        .build();
}

// Random Spawn Table system
pub struct RandomEntry {
    name: String,
    weight: i32
}

impl RandomEntry {
    pub fn new<S:ToString>(name: S, weight: i32) -> RandomEntry {
        RandomEntry { name: name.to_string(), weight }
    }
}

#[derive(Default)]
pub struct RandomTable {
    entries : Vec<RandomEntry>,
    total_weight: i32
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable{ entries: Vec::new(), total_weight: 0}
    }

    pub fn add<S:ToString>(mut self, name : S, weight: i32) -> RandomTable {
        self.total_weight += weight;
        self.entries.push(RandomEntry::new(name.to_string(), weight));
        self
    }

    pub fn roll(&self, rng : &mut RandomNumberGenerator) -> String {
        if self.total_weight == 0 { return "None".to_string(); }
        let mut roll = rng.roll_dice(1, self.total_weight)-1;
        let mut index : usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].name.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "None".to_string()
    }
}