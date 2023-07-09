
use specs::prelude::*;

use crate::{game::GameLog, components::{PickupItemIntent, Position, Name, ItemOwned, ActiveEntity}};

pub struct InventorySystem{}

impl<'a> System<'a> for InventorySystem {
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
