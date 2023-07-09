use specs::prelude::*;
use bracket_lib::prelude::*;
use crate::{game::GameLog, components::Position};

use super::super::{CombatStats, PoolStats, MeleeIntent, Name, Damage, ParticleBuilder};

pub struct MeleeCombatSystem {}

impl <'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MeleeIntent>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, PoolStats>,
        WriteStorage<'a, Damage>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut melee_intent, names, combat_stats, pool_stats, mut inflicted_damage, mut log, mut particle_builder, positions) = data;
        
        for (_entity, intent, name, stats, pool) in (&entities, &melee_intent, &names, &combat_stats, &pool_stats).join() {
            if pool.hp.current > 0 {
                let target_combat_stats = combat_stats.get(intent.target).unwrap();
                let target_pool_stats = pool_stats.get(intent.target).unwrap();
                if target_pool_stats.hp.current > 0 {
                    let target_name = names.get(intent.target).unwrap();
                    let target_pos = positions.get(intent.target);
                    if let Some(pos) = target_pos {
                        particle_builder.request(pos.x, pos.y, RGB::named(ORANGE), RGB::named(BLACK), to_cp437('‼'), 200.0)
                    }
                    let damage = i32::max(0, stats.attack - target_combat_stats.defense);

                    if damage == 0 {
                        log.entries.push(format!("{} is unable to hurt {}", &name.name, &target_name.name));
                    } else {
                        log.entries.push(format!("{} hits {}, for {} hp.", &name.name, &target_name.name, damage));
                        Damage::new(&mut inflicted_damage, intent.target, damage);
                    }
                }
            }
        }

        melee_intent.clear();
    }
}