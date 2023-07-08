use bracket_lib::terminal::*;
use specs::prelude::*;
use super::super::{CombatStats, PoolStats, MeleeIntent, Name, Damage};

pub struct MeleeCombatSystem {}

impl <'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MeleeIntent>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, PoolStats>,
        WriteStorage<'a, Damage>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut melee_intent, names, combat_stats, pool_stats, mut inflicted_damage) = data;
        
        for (_entity, intent, name, stats, pool) in (&entities, &melee_intent, &names, &combat_stats, &pool_stats).join() {
            if pool.hp.current > 0 {
                let target_combat_stats = combat_stats.get(intent.target).unwrap();
                let target_pool_stats = pool_stats.get(intent.target).unwrap();
                if target_pool_stats.hp.current > 0 {
                    let target_name = names.get(intent.target).unwrap();
                    let damage = i32::max(0, stats.attack - target_combat_stats.defense);

                    if damage == 0 {
                        console::log(format!("{} is unable to hurt {}", &name.name, &target_name.name))
                    } else {
                        console::log(format!("{} hits {}, for {} hp.", &name.name, &target_name.name, damage));
                        Damage::new(&mut inflicted_damage, intent.target, damage);
                    }
                }
            }
        }

        melee_intent.clear();
    }
}