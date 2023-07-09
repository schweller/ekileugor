use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::game::GameLog;
use crate::{components::*, RunState};

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(bracket_lib::color::WHITE), RGB::named(bracket_lib::color::BLACK));

    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }

    draw_pool_stats(ecs, ctx);
    draw_active_target(ecs, ctx);
    draw_runstate(ecs, ctx);
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

fn draw_runstate(ecs: &World, ctx: &mut BTerm) {
    let runstate = ecs.read_resource::<RunState>();
    let newrunstate;
    let runstate_string;
    {
        newrunstate = *runstate;
    }

    match newrunstate {
        RunState::PreRun => {
            runstate_string = "Pre Run";
        }
        RunState::AwaitingInput => {
            runstate_string = "Awaiting Input";
        }
        RunState::PlayerTurn => {
            runstate_string = "Player Turn";
        }
        RunState::MonsterTurn => {
            runstate_string = "Monster Turn";
        }
        RunState::CurseTurn => {
            runstate_string = "Curse Turn";
        }
    }

    ctx.print_color(
        66, 
        10, 
        RGB::named(bracket_lib::color::WHITE), 
        RGB::named(bracket_lib::color::GREY), 
        runstate_string
    );    
}
