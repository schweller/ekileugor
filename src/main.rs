use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
use components::*;
mod map;
use map::*;
mod game;
use game::*;
use systems::{remove_particles, ParticleBuilder};
mod gui;
mod systems;

pub struct State {
    pub ecs: World,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput, PreRun, PlayerTurn, MonsterTurn, CurseTurn, NextLevel
}

impl State {
    fn run_systems(&mut self) {
        let mut fov = systems::VisibilitySystem{};
        fov.run_now(&self.ecs);
        let mut map_index = systems::MapIndexingSystem{};
        map_index.run_now(&self.ecs);
        let mut melee_combat = systems::MeleeCombatSystem{};
        melee_combat.run_now(&self.ecs);
        let mut damage = systems::DamageSystem{};
        damage.run_now(&self.ecs);
        let mut inventory = systems::ItemPickupSystem{};
        inventory.run_now(&self.ecs);
        let mut inventory_use = systems::ItemUseSystem{};
        inventory_use.run_now(&self.ecs);
        let mut particles = systems::ParticleSpawnSystem{};
        particles.run_now(&self.ecs);
        let mut trigger = systems::TriggerSystem{};
        trigger.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl State {
    fn remove_entities_next_level(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let stored_items = self.ecs.read_storage::<ItemOwned>();
        let player_entity = self.ecs.fetch::<Entity>();

        let mut to_remove : Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_remove = true;

            // Don't remove the hero
            let p = player.get(entity);
            if let Some(_p) = p {
                should_remove = false;
            }

            // Don't remove hero's items
            let stored = stored_items.get(entity);
            if let Some(stored) = stored {
                if stored.owner == *player_entity {
                    should_remove = false;
                }
            }            

            if should_remove {
                to_remove.push(entity);
            }
        }
        to_remove
    }

    fn goto_next_level(&mut self) {
        let to_remove = self.remove_entities_next_level();
        for entity in to_remove {
            self.ecs.delete_entity(entity).expect("Unable to delete entity on Level Change");
        }

        let map;
        let current_depth;
        {
            let mut map_resource = self.ecs.write_resource::<Map>();
            current_depth = map_resource.depth;
            *map_resource = Map::map_with_rooms_and_corridors(current_depth + 1);
            map = map_resource.clone();
        }

        for room in map.rooms.iter().skip(1) {
            spawn_room(&mut self.ecs, room, current_depth + 1);
        }

        let (player_x, player_y) = map.rooms[0].center();
        let player_entity = self.ecs.fetch_mut::<Entity>();
        let mut positions = self.ecs.write_storage::<Position>();
        let player_entity_position = positions.get_mut(*player_entity);
        if let Some(player_entity_position) = player_entity_position {
            player_entity_position.x = player_x;
            player_entity_position.y = player_y;
        }
        let mut active_entity = self.ecs.write_resource::<ActiveEntity>();
        active_entity.target = *player_entity;

        let mut viewsheds = self.ecs.write_storage::<Viewshed>();
        let vs = viewsheds.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
        let mut log = self.ecs.fetch_mut::<GameLog>();
        log.entries.push("You reached the portal and moved on!".to_string()); 
        // reset stats?

    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();
        remove_particles(&mut self.ecs, ctx);

        match newrunstate {
            _ => {
                render_camera(&self.ecs, ctx);
                gui::draw_ui(&self.ecs, ctx);        
            }
        }

        // draw_map(&self.ecs, ctx);
        
        // {
        //     let entities = self.ecs.entities();
        //     let map = self.ecs.fetch::<Map>();
        //     let positions = self.ecs.read_storage::<Position>();
        //     let renderables = self.ecs.read_storage::<Renderable>();

        //     let mut data = (&entities, &positions, &renderables).join().collect::<Vec<_>>();
        //     data.sort_by(|&a, &b| b.2.render_order.cmp(&a.2.render_order));
        //     for (_entity, pos, render) in data.iter() {
        //         let idx = map.xy_idx(pos.x, pos.y);
        //         if map.visible_tiles[idx] {
        //             ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        //         }
        //     }
            
        //     gui::draw_ui(&self.ecs, ctx);
        // }



        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::CurseTurn;
            }
            RunState::CurseTurn => {
                self.run_systems();
                self.ecs.maintain();
                // try_curse(&mut self.ecs);
                newrunstate = RunState::AwaitingInput;
            }
            RunState::NextLevel => {
                self.goto_next_level();
                newrunstate = RunState::PreRun;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        ctx.print(1, 49, &format!("FPS: {}", ctx.fps));

        systems::delete_the_dead(&mut self.ecs);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("GMTK2023 - Ekileugor")
        .with_tile_dimensions(16, 16)
        .build()?;
    context.with_post_scanlines(true);

    let mut gamestate = State{
        ecs: World::new()
    };

    let rng = bracket_lib::random::RandomNumberGenerator::new();

    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<Player>();
    gamestate.ecs.register::<Mob>();
    gamestate.ecs.register::<Controllable>();
    gamestate.ecs.register::<Name>();
    gamestate.ecs.register::<Viewshed>();

    // Stats components
    gamestate.ecs.register::<SinglePoolStat>();
    gamestate.ecs.register::<SingleStat>();
    gamestate.ecs.register::<CombatStats>();
    gamestate.ecs.register::<PoolStats>();
    
    // Map meta components
    gamestate.ecs.register::<BlocksTile>();

    // Combat components
    gamestate.ecs.register::<MeleeIntent>();
    gamestate.ecs.register::<Damage>();
    gamestate.ecs.register::<InflictsDamage>();
    gamestate.ecs.register::<Hidden>();
    gamestate.ecs.register::<EntryTrigger>();
    gamestate.ecs.register::<EntityMoved>();

    gamestate.ecs.register::<Item>();
    gamestate.ecs.register::<ItemOwned>();
    gamestate.ecs.register::<UseItemIntent>();
    gamestate.ecs.register::<PickupItemIntent>();
    gamestate.ecs.register::<Consumable>();
    gamestate.ecs.register::<Heals>();

    gamestate.ecs.register::<ParticleLifetime>();


    let map : Map = Map::map_with_rooms_and_corridors(1);
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = game::player(&mut gamestate.ecs, player_x, player_y);

    let active_entity = ActiveEntity{
        target: player_entity
    };
    
    gamestate.ecs.insert(rng);

    for room in map.rooms.iter().skip(1) {
        game::spawn_room(&mut gamestate.ecs, room, 1);
    }

    gamestate.ecs.insert(game::GameLog{entries: vec!["You enter Ekileugor".to_string()]});
    gamestate.ecs.insert(active_entity);
    gamestate.ecs.insert(player_entity);
    gamestate.ecs.insert(map);
    gamestate.ecs.insert(RunState::PreRun);
    gamestate.ecs.insert(ParticleBuilder::new());

    main_loop(context, gamestate)
}

