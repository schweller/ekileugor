use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::game::GameLog;
use crate::{components::*, map::Map, RunState};

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(bracket_lib::color::WHITE), RGB::named(bracket_lib::color::BLACK));

    let log = ecs.fetch::<GameLog>();
    let map = ecs.fetch::<Map>();
    let depth = format!("Depth: {}", map.depth);
    ctx.print_color(2, 43, RGB::named(YELLOW), RGB::named(BLACK), &depth);

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }

    draw_inventory(ecs, ctx);
    draw_pool_stats(ecs, ctx);
    draw_active_target(ecs, ctx);
    // draw_runstate(ecs, ctx);
}

fn draw_inventory(ecs: &World, ctx: &mut BTerm) {
    let active_entity = ecs.fetch::<ActiveEntity>();
    let items_owned = ecs.read_storage::<ItemOwned>();
    let names = ecs.read_storage::<Name>();
    let entities = ecs.entities();

    let inventory = (&items_owned, &names).join().filter(|item| active_entity.target.eq(&item.0.owner));
    let inventory_count = inventory.count();

    let mut y = (25 - (inventory_count / 2)) as i32;
    ctx.draw_box(65, y-2, 14, (inventory_count+3) as i32, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_color(66, y-2, RGB::named(YELLOW), RGB::named(BLACK), "Inventory");

    let mut j = 0;
    for (_entity, _backpack, name) in (&entities, &items_owned, &names).join().filter(|item| active_entity.target.eq(&item.1.owner)) {
        ctx.set(66, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437('('));
        ctx.set(67, y, RGB::named(YELLOW), RGB::named(BLACK), 97+j as FontCharType);
        ctx.set(68, y, RGB::named(WHITE), RGB::named(BLACK), to_cp437(')'));

        ctx.print(69, y, &name.name.to_string());
        y += 1;
        j += 1;
    }
}

fn draw_pool_stats(ecs: &World, ctx: &mut BTerm) {
    let active_target = ecs.fetch::<ActiveEntity>();
    let pool_stats = ecs.read_storage::<PoolStats>();

    let active_pool_stats = pool_stats.get(active_target.target).unwrap();
    ctx.draw_box(65, 3, 14, 2, RGB::named(bracket_lib::color::ALICEBLUE), RGB::named(bracket_lib::color::BLACK));
    ctx.print_color(
        66, 
        4, 
        RGB::named(bracket_lib::color::WHITE), 
        RGB::named(bracket_lib::color::GREY), 
        &format!("HP {}/{}", active_pool_stats.hp.current.to_string(), active_pool_stats.hp.max.to_string())
    );
}

fn draw_active_target(ecs: &World, ctx: &mut BTerm) {
    let active_target = ecs.fetch::<ActiveEntity>();
    let names = ecs.read_storage::<Name>();

    let active_name = names.get(active_target.target).unwrap();
    ctx.draw_box(65, 0, 14, 2, RGB::named(bracket_lib::color::ALICEBLUE), RGB::named(bracket_lib::color::BLACK));
    ctx.print_color(
        66, 
        1, 
        RGB::named(bracket_lib::color::WHITE), 
        RGB::named(bracket_lib::color::GREY), 
        active_name.name.to_string()
    );
}

// fn draw_runstate(ecs: &World, ctx: &mut BTerm) {
//     let runstate = ecs.read_resource::<RunState>();
//     let newrunstate;
//     let runstate_string;
//     {
//         newrunstate = *runstate;
//     }

//     match newrunstate {
//         RunState::PreRun => {
//             runstate_string = "Pre Run";
//         }
//         RunState::AwaitingInput => {
//             runstate_string = "Awaiting Input";
//         }
//         RunState::PlayerTurn => {
//             runstate_string = "Player Turn";
//         }
//         RunState::MonsterTurn => {
//             runstate_string = "Monster Turn";
//         }
//         RunState::CurseTurn => {
//             runstate_string = "Curse Turn";
//         }
//     }

//     ctx.print_color(
//         66, 
//         10, 
//         RGB::named(bracket_lib::color::WHITE), 
//         RGB::named(bracket_lib::color::GREY), 
//         runstate_string
//     );    
// }
