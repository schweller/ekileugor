use bracket_lib::terminal::{BTerm, VirtualKeyCode, Point};
use specs::prelude::*;
use crate::{components::*, RunState};
use crate::map::{Map, TileType};

use super::{GameLog};
use super::super::State;

use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut melee_intent = ecs.write_storage::<MeleeIntent>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let stats = ecs.read_storage::<CombatStats>();

    // let controllables = ecs.read_storage::<Controllable>();
    let map = ecs.fetch::<Map>();
    let active_entity = ecs.fetch::<ActiveEntity>();

    let mut entity_moved = ecs.write_storage::<EntityMoved>();

    // let active = controllables.get(active_camera.target).unwrap();
    let active_target_pos = positions.get_mut(active_entity.target).unwrap();
    let viewshed = viewsheds.get_mut(active_entity.target).unwrap();
    let dest_idx = map.xy_idx(active_target_pos.x + delta_x, active_target_pos.y + delta_y);
    for potential_target in map.tile_content[dest_idx].iter() {
        let target = stats.get(*potential_target);
        if let Some(_t) = target {
            melee_intent.insert(active_entity.target, MeleeIntent { target: *potential_target }).expect("Unable to insert melee intent");
            return;
        }
    }

    if !map.blocked[dest_idx] {
        active_target_pos.x = min(map.width - 1 , max(0, active_target_pos.x + delta_x));
        active_target_pos.y = min(map.height - 1, max(0, active_target_pos.y + delta_y));
        viewshed.dirty = true;

        let mut player_pos = ecs.write_resource::<Point>();
        player_pos.x = active_target_pos.x;
        player_pos.y = active_target_pos.y;

        entity_moved.insert(active_entity.target, EntityMoved {}).expect("Unable to insert EntityMoved marker");
    }
}

fn pickup_item(ecs: &mut World) {
    let active_entity = ecs.fetch::<ActiveEntity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();
    let active_entity_pos = ecs.fetch::<Point>();
    
    let mut target_item : Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == active_entity_pos.x && position.y == active_entity_pos.y {
        target_item = Some(item_entity);
        }    
    }
  
    match target_item {
        None => gamelog.entries.push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<PickupItemIntent>();
            pickup.insert(active_entity.target, PickupItemIntent{ picked_by: active_entity.target, item }).expect("Unable to insert want to pickup");
        }
    }
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let active_entity = ecs.fetch::<ActiveEntity>();
    let positions = ecs.read_storage::<Position>();
    let active_entity_pos = positions.get(active_entity.target).unwrap();
    let map = ecs.fetch::<Map>();
    let active_entity_pos_idx = map.xy_idx(active_entity_pos.x, active_entity_pos.y);

    if map.tiles[active_entity_pos_idx] == TileType::Exit {
        true
    } else {
        let mut log = ecs.fetch_mut::<GameLog>();
        log.entries.push("There is no exit here!".to_string());
        false 
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::AwaitingInput } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::G => pickup_item(&mut gs.ecs),
            VirtualKeyCode::Period => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            },
            // VirtualKeyCode::A => try_curse(&mut gs.ecs),
            _ => { return RunState::AwaitingInput }
        },
    }
    RunState::PlayerTurn
}
