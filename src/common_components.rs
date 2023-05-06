use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct RayBeam {
    time_to_live: u32,
    damage: u32,
}

#[derive(Component)]
struct Movable;
