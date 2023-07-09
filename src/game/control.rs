use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::*;

use super::GameLog;

pub fn try_change_control(ecs: &mut World) {
    let controllables = ecs.write_storage::<Controllable>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();


    let mut active_target = ecs.fetch_mut::<ActiveEntity>();
    let entities = ecs.entities();
    let mut rng = ecs.fetch_mut::<RandomNumberGenerator>();

    let roll = rng.roll_dice(1, 4);

    match roll {
        3 => {
            let mut log = ecs.fetch_mut::<GameLog>();
            let data = (&entities, &controllables, &names, &positions).join().collect::<Vec<_>>();
            let random = bracket_lib::random::RandomNumberGenerator::random_slice_entry(&mut rng, data.as_slice()).unwrap();
            active_target.target = random.0;
        
            let viewshed = viewsheds.get_mut(active_target.target).unwrap();
            viewshed.dirty = true;
        
            log.entries.push(format!("You've been cursed! You are now the {}", random.2.name));
        }
        _ => {}
    }


    // for (ent, _controllable, _pos, name) in (&entities, &controllables, &positions, &names).join() {
    //     active_target.target = ent
    // }
}
