use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug, Clone)]
pub struct MeleeIntent{
    pub target: Entity
}

#[derive(Component, Debug)]
pub struct Damage{
    pub amount: Vec<i32>
}

impl Damage {
    pub fn new(store: &mut WriteStorage<Damage>, target: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(target) {
            suffering.amount.push(amount);
        } else {
            let damage = Damage { amount: vec![amount] };
            store.insert(target, damage).expect("Unable to insert Damage intent");
        }
    }
}

#[derive(Component, Debug)]
pub struct InflictsDamage{
    pub amount: i32
}
