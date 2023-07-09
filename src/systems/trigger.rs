use specs::prelude::*;
use bracket_lib::prelude::*;
use crate::{components::{EntityMoved, Position, EntryTrigger, Hidden, Name, InflictsDamage, Damage}, map::Map, game::GameLog};

use super::ParticleBuilder;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Map>,
                        WriteStorage<'a, EntityMoved>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, EntryTrigger>,
                        WriteStorage<'a, Hidden>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, InflictsDamage>,
                        WriteStorage<'a, Damage>,
                        WriteExpect<'a, ParticleBuilder>,
                        Entities<'a>,
                        WriteExpect<'a, GameLog>);

    fn run(&mut self, data : Self::SystemData) {
        let (map, mut entity_moved, position, entry_trigger, mut hidden, names, inflicts_damage, mut inflicted_damage, mut particle_builder, entities, mut log) = data;

        // Iterate the entities that moved and their final position
        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id { // Do not bother to check yourself for being a trap!
                    let maybe_trigger = entry_trigger.get(*entity_id);
                    match maybe_trigger {
                        None => {},
                        Some(_trigger) => {
                            // We triggered it                            
                            let name = names.get(*entity_id);
                            if let Some(name) = name {
                                log.entries.push(format!("{} triggers!", &name.name));
                            }

                            let damage = inflicts_damage.get(*entity_id);
                            if let Some(damage) = damage {
                                particle_builder.request(pos.x, pos.y, RGB::named(ORANGE), RGB::named(BLACK), to_cp437('‼'), 200.0);
                                Damage::new(&mut inflicted_damage, entity, damage.amount);                                
                            }

                            hidden.remove(*entity_id); // The trap is no longer hidden
                        }
                    }
                }
            }
        }

        // Remove all entity movement markers
        entity_moved.clear();
    }
}
