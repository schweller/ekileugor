
use specs::prelude::*;

use crate::{game::GameLog, components::*, map::Map};

pub struct ItemPickupSystem{}

impl<'a> System<'a> for ItemPickupSystem {
    type SystemData = ( ReadExpect<'a, ActiveEntity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, PickupItemIntent>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, ItemOwned>,
                      );
    
    fn run(&mut self, data: Self::SystemData) {
        let (active_entity, mut log, mut pickup_intent, mut pos, names, mut item_owned) = data;

        for pickup in pickup_intent.join() {
            pos.remove(pickup.item);
            item_owned.insert(pickup.item, ItemOwned{ owner: pickup.picked_by }).expect("Unable to insert ItemOwned");

            if pickup.picked_by == active_entity.target {
                log.entries.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }
        
        pickup_intent.clear();
    }
}

pub struct ItemUseSystem{}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = ( ReadExpect<'a, ActiveEntity>,
                        WriteExpect<'a, GameLog>,
                        ReadExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, UseItemIntent>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Heals>,
                        WriteStorage<'a, PoolStats>,
                        ReadStorage<'a, Consumable>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (player_entity, mut gamelog, map, entities, mut use_intent, names, healing, mut pool_stats, consumables) = data;

        for (entity, useitem) in (&entities, &use_intent).join() {
            let mut used_item = true;

            let mut targets : Vec<Entity> = Vec::new();
            match useitem.target {
                None => {targets.push(player_entity.target)}
                Some(_target) => {}
            }

            let item_heals = healing.get(useitem.item);
            match item_heals {
                None => {}
                Some(healer) => {
                    used_item = false;
                    for target in targets.iter() {
                        let stats = pool_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp.current = i32::min(stats.hp.max, stats.hp.current + healer.amount);
                            if entity == player_entity.target {
                                gamelog.entries.push(format!("You use the {}, healing {} hp.", names.get(useitem.item).unwrap().name, healer.amount));
                            }
                            used_item = true;
                        }                        
                    }
                }
            }

            if used_item {
                let consumable = consumables.get(useitem.item);
                match consumable {
                    None => {}
                    Some(_) => {
                        entities.delete(useitem.item).expect("Delete failed");
                    }
                }
            }
        }

        use_intent.clear();
    }
}