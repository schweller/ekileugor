use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Item{}

#[derive(Component, Debug, Clone)]
pub struct ItemOwned{
    pub owner: Entity
}

#[derive(Component, Debug, Clone)]
pub struct PickupItemIntent{
    pub picked_by: Entity,
    pub item : Entity
}

#[derive(Component, Debug, Clone)]
pub struct UseItemIntent{
    pub item : Entity
}

#[derive(Component, Debug)]
pub struct Consumable{
    pub charges: i32
}

#[derive(Component, Debug)]
pub struct Heals {
    pub amount: i32
}
