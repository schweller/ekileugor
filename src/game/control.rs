use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::*;

pub fn try_change_control(ecs: &mut World) {
    let controllables = ecs.write_storage::<Controllable>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();


    let mut active_target = ecs.fetch_mut::<ActiveEntity>();
    let entities = ecs.entities();
    let mut rng = ecs.fetch_mut::<RandomNumberGenerator>();

    let data = (&entities, &controllables, &names, &positions).join().collect::<Vec<_>>();
    let random = bracket_lib::random::RandomNumberGenerator::random_slice_entry(&mut rng, data.as_slice()).unwrap();
    active_target.target = random.0;

    let viewshed = viewsheds.get_mut(active_target.target).unwrap();
    viewshed.dirty = true;

    console::log(&format!("{} is being controlled", random.2.name));

    // for (ent, _controllable, _pos, name) in (&entities, &controllables, &positions, &names).join() {
    //     active_target.target = ent
    // }
}
