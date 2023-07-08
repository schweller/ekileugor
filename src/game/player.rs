use bracket_lib::terminal::{BTerm, VirtualKeyCode, console};
use specs::prelude::*;
use crate::components::*;
use crate::map::Map;

use super::super::State;

use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut melee_intent = ecs.write_storage::<MeleeIntent>();
    let stats = ecs.read_storage::<CombatStats>();

    // let controllables = ecs.read_storage::<Controllable>();
    let map = ecs.fetch::<Map>();
    let active_camera = ecs.fetch::<ActiveEntity>();

    // let active = controllables.get(active_camera.target).unwrap();
    let active_target_pos = positions.get_mut(active_camera.target).unwrap();
    let dest_idx = map.xy_idx(active_target_pos.x + delta_x, active_target_pos.y + delta_y);
    for potential_target in map.tile_content[dest_idx].iter() {
        let target = stats.get(*potential_target);
        if let Some(_t) = target {
            melee_intent.insert(active_camera.target, MeleeIntent { target: *potential_target }).expect("Unable to insert melee intent");
            console::log(&format!("From Hell's Heart, I stab thee!"));
            return;
        }
    }

    if !map.blocked[dest_idx] {
        active_target_pos.x = min(79 , max(0, active_target_pos.x + delta_x));
        active_target_pos.y = min(49, max(0, active_target_pos.y + delta_y));
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
