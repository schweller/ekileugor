use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32
}
