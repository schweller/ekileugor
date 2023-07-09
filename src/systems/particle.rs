
use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{ParticleLifetime, Position, Renderable};

pub struct ParticleSpawnSystem {}

impl <'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut particles, mut particle_builder) = data;

        for new_particle in particle_builder.requests.iter() {
            let particle = entities.create();
            positions
                .insert(particle, Position { x: new_particle.x, y: new_particle.y })
                .expect("Unable to insert Position");
            renderables
                .insert(particle, Renderable { glyph: new_particle.glyph, fg: new_particle.fg, bg: new_particle.bg, render_order: 0 })
                .expect("Unable to insert Renderable for Particle");
            particles
                .insert(particle, ParticleLifetime { lifetime_ms: new_particle.lifetime })
                .expect("Unable to insert ParticleLifetime for Particle");
        }

        particle_builder.requests.clear();
    }
}

struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: FontCharType,
    lifetime: f32
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder{ requests: Vec::new() }
    }

    pub fn request(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: FontCharType, lifetime: f32) {
        self.requests.push(
            ParticleRequest { x, y, fg, bg, glyph, lifetime }
        )
    }
}

pub fn remove_particles(ecs: &mut World, ctx: &BTerm) {
    let mut dead_particles : Vec<Entity> = Vec::new();
    {
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= ctx.frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                dead_particles.push(entity);
            }            
        }
    }

    for particle in dead_particles.iter() {
        ecs.delete_entity(*particle).expect("Unable to delete Particle")
    }
}
