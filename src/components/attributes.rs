use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct SingleStat {
    pub base: i32,
    pub bonus: i32,
    pub modifiers: i32
}

#[derive(Component)]
pub struct SinglePoolStat {
    pub current: i32,
    pub max: i32
}

#[derive(Component)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
    pub evade: i32,
}

#[derive(Component)]
pub struct PoolStats {
    pub hp: SinglePoolStat,
    pub xp: i32,
    pub level: i32,
    pub gold: i32
}
