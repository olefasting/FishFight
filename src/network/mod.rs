use ff_core::ecs::World;

pub mod api;

use ff_core::result::Result;

pub fn update_network_client(world: &mut World, delta_time: f32) -> Result<()> {
    update_network_common(world, delta_time)?;

    Ok(())
}

pub fn fixed_update_network_client(
    world: &mut World,
    delta_time: f32,
    integration_factor: f32,
) -> Result<()> {
    fixed_update_network_common(world, delta_time, integration_factor)?;

    Ok(())
}

pub fn update_network_host(world: &mut World, delta_time: f32) -> Result<()> {
    update_network_common(world, delta_time)?;

    Ok(())
}

pub fn fixed_update_network_host(
    world: &mut World,
    delta_time: f32,
    integration_factor: f32,
) -> Result<()> {
    fixed_update_network_common(world, delta_time, integration_factor)?;

    Ok(())
}

fn update_network_common(_world: &mut World, _delta_time: f32) -> Result<()> {
    Ok(())
}

fn fixed_update_network_common(
    _world: &mut World,
    _delta_time: f32,
    _integration_factor: f32,
) -> Result<()> {
    Ok(())
}
