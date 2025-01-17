use hecs::World;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::path::Path;

use num_traits::*;

use crate::math::Vec2;

pub use crate::backend_impl::particles::*;
use crate::drawables::AnimatedSpriteMetadata;
use crate::file::read_from_file;
use crate::parsing::deserialize_bytes_by_extension;
use crate::result::Result;
use crate::transform::Transform;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticleEmitterMetadata {
    /// The id of the particle effect.
    #[serde(rename = "particle_effect")]
    pub particle_effect_id: String,
    /// The offset is added to the `position` provided when calling `draw`
    #[serde(default, with = "crate::parsing::vec2_def")]
    pub offset: Vec2,
    /// Delay before emission will begin
    #[serde(default, skip_serializing_if = "f32::is_zero")]
    pub delay: f32,
    /// The interval between each emission.
    #[serde(default, skip_serializing_if = "f32::is_zero")]
    pub interval: f32,
    /// Amount of emissions per activation. If set to `None` it will emit indefinitely
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissions: Option<u32>,
    /// This is a temporary hack that enables texture based effects until we add texture support
    /// to our macroquad-particles fork
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animations: Option<AnimatedSpriteMetadata>,
    /// If this is set to `true` the `ParticleController` will start to emit automatically
    #[serde(default, skip_serializing_if = "crate::parsing::is_false")]
    pub should_autostart: bool,
}

impl Default for ParticleEmitterMetadata {
    fn default() -> Self {
        ParticleEmitterMetadata {
            particle_effect_id: "".to_string(),
            offset: Vec2::ZERO,
            delay: 0.0,
            emissions: None,
            interval: 0.0,
            animations: None,
            should_autostart: false,
        }
    }
}

pub struct ParticleEmitter {
    pub particle_effect_id: String,
    pub offset: Vec2,
    pub delay: f32,
    pub emissions: Option<u32>,
    pub interval: f32,
    pub emission_cnt: u32,
    pub delay_timer: f32,
    pub interval_timer: f32,
    pub is_active: bool,
}

impl ParticleEmitter {
    pub fn new(meta: ParticleEmitterMetadata) -> Self {
        ParticleEmitter {
            particle_effect_id: meta.particle_effect_id,
            offset: meta.offset,
            delay: meta.delay,
            interval: meta.interval,
            emissions: meta.emissions,
            emission_cnt: 0,
            delay_timer: 0.0,
            interval_timer: meta.interval,
            is_active: meta.should_autostart,
        }
    }

    pub fn get_offset(&self, flip_x: bool, flip_y: bool) -> Vec2 {
        let mut offset = self.offset;

        if flip_x {
            offset.x = -offset.x;
        }

        if flip_y {
            offset.y = -offset.y;
        }

        offset
    }

    pub fn activate(&mut self) {
        self.delay_timer = 0.0;
        self.interval_timer = self.interval;
        self.emission_cnt = 0;
        self.is_active = true;
    }
}

impl From<ParticleEmitterMetadata> for ParticleEmitter {
    fn from(params: ParticleEmitterMetadata) -> Self {
        ParticleEmitter::new(params)
    }
}

#[derive(Default)]
pub struct ParticleEmitterCache {
    pub cache_map: HashMap<String, EmittersCache>,
}

impl ParticleEmitterCache {
    pub fn new() -> Self {
        let mut cache_map = HashMap::new();

        for (id, config) in iter_particle_effects() {
            cache_map.insert(id.clone(), EmittersCache::new(config.clone()));
        }

        ParticleEmitterCache { cache_map }
    }
}

static mut PARTICLE_EMITTER_CACHE: Option<ParticleEmitterCache> = None;

fn particle_emitter_cache() -> &'static mut ParticleEmitterCache {
    unsafe { PARTICLE_EMITTER_CACHE.get_or_insert_with(ParticleEmitterCache::new) }
}

fn update_one_particle_emitter(
    delta_time: f32,
    mut position: Vec2,
    rotation: f32,
    emitter: &mut ParticleEmitter,
) {
    if emitter.is_active {
        emitter.delay_timer += delta_time;

        if emitter.delay_timer >= emitter.delay {
            emitter.interval_timer += delta_time;
        }

        if emitter.delay_timer >= emitter.delay && emitter.interval_timer >= emitter.interval {
            emitter.interval_timer = 0.0;

            if rotation == 0.0 {
                position += emitter.offset;
            } else {
                let offset_position = position + emitter.offset;

                let sin = rotation.sin();
                let cos = rotation.cos();

                position = Vec2::new(
                    cos * (offset_position.x - position.x) - sin * (offset_position.y - position.y)
                        + position.x,
                    sin * (offset_position.x - position.x)
                        + cos * (offset_position.y - position.y)
                        + position.y,
                );
            }

            let particles = particle_emitter_cache();
            let cache = particles
                .cache_map
                .get_mut(&emitter.particle_effect_id)
                .unwrap();

            cache.spawn(position);

            if let Some(emissions) = emitter.emissions {
                emitter.emission_cnt += 1;

                if emissions > 0 && emitter.emission_cnt >= emissions {
                    emitter.is_active = false;
                }
            }
        }
    }
}

pub fn update_particle_emitters(world: &mut World, delta_time: f32) -> Result<()> {
    for (_, (transform, emitter)) in world.query_mut::<(&Transform, &mut ParticleEmitter)>() {
        update_one_particle_emitter(delta_time, transform.position, transform.rotation, emitter);
    }

    for (_, (transform, emitters)) in world.query_mut::<(&Transform, &mut Vec<ParticleEmitter>)>() {
        for emitter in emitters.iter_mut() {
            update_one_particle_emitter(
                delta_time,
                transform.position,
                transform.rotation,
                emitter,
            );
        }
    }

    Ok(())
}

pub fn draw_particles(_world: &mut World, _delta_time: f32) -> Result<()> {
    let particles = particle_emitter_cache();

    for cache in particles.cache_map.values_mut() {
        cache.draw();
    }

    Ok(())
}

const PARTICLE_EFFECT_RESOURCES_FILE: &str = "particle_effects";

static mut PARTICLE_EFFECTS: Option<HashMap<String, EmitterConfig>> = None;

pub fn try_get_particle_effect(id: &str) -> Option<&EmitterConfig> {
    unsafe { PARTICLE_EFFECTS.get_or_insert_with(HashMap::new).get(id) }
}

pub fn get_particle_effect(id: &str) -> &EmitterConfig {
    try_get_particle_effect(id).unwrap()
}

pub fn iter_particle_effects() -> Iter<'static, String, EmitterConfig> {
    unsafe { PARTICLE_EFFECTS.get_or_insert_with(HashMap::new) }.iter()
}

#[derive(Serialize, Deserialize)]
struct ParticleEffectMetadata {
    id: String,
    path: String,
}

pub async fn load_particle_effects<P: AsRef<Path>>(
    path: P,
    ext: &str,
    is_required: bool,
    should_overwrite: bool,
) -> Result<()> {
    let particle_effects = unsafe { PARTICLE_EFFECTS.get_or_insert_with(HashMap::new) };

    if should_overwrite {
        particle_effects.clear();
    }

    let particle_effects_file_path = path
        .as_ref()
        .join(PARTICLE_EFFECT_RESOURCES_FILE)
        .with_extension(ext);

    match read_from_file(&particle_effects_file_path).await {
        Err(err) => {
            if is_required {
                return Err(err.into());
            }
        }
        Ok(bytes) => {
            let metadata: Vec<ParticleEffectMetadata> =
                deserialize_bytes_by_extension(ext, &bytes)?;

            for meta in metadata {
                let file_path = path.as_ref().join(&meta.path);

                let extension = file_path.extension().unwrap().to_str().unwrap();

                let bytes = read_from_file(&file_path).await?;

                let cfg: EmitterConfig = deserialize_bytes_by_extension(extension, &bytes)?;

                particle_effects.insert(meta.id, cfg);
            }
        }
    }

    Ok(())
}
