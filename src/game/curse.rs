use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::*;

use super::GameLog;

pub fn _try_curse(ecs: &mut World) {
    let mut rng = ecs.fetch_mut::<RandomNumberGenerator>();
    let roll = rng.roll_dice(1, 4);

    // Weighted probabily
    // Different maps with different weights?
    // Harder more shifts?
    match roll {
        3 => {
            // get resources/components
            let controllables = ecs.write_storage::<Controllable>();
            let positions = ecs.read_storage::<Position>();
            let names = ecs.read_storage::<Name>();
            let mut viewsheds = ecs.write_storage::<Viewshed>();
            let mut active_target = ecs.fetch_mut::<ActiveEntity>();
            let entities = ecs.entities();
            let mut log = ecs.fetch_mut::<GameLog>();
        
            // group into a vec and turn into a slice (bc im not that smart)
            // pick one randomly
            let data = (&entities, &controllables, &names, &positions).join().collect::<Vec<_>>();
            let random = bracket_lib::random::RandomNumberGenerator::random_slice_entry(&mut rng, data.as_slice()).unwrap();
            active_target.target = random.0;
        
            let viewshed = viewsheds.get_mut(active_target.target).unwrap();
            viewshed.dirty = true;
        
            log.entries.push(format!("You've been cursed! You are now the {}", random.2.name));
        }
        _ => {}
    }
}
