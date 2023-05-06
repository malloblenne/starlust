use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub mod camera_handling;
pub mod common_components;
pub mod player;

use crate::camera_handling::{move_camera, spawn_camera, sync_player_camera};
use crate::player::{control_player, spawn_player};

fn populate_world(
    mut commands: Commands,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.92,
    });

    spawn_camera(&mut commands);

    /* Create the ground. */
    commands.spawn((
        Collider::cuboid(100.0, 0.1, 100.0),
        TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)),
    ));

    /* Create Player */
    spawn_player(&mut commands);

}

pub struct StarLustPlugin;

impl Plugin for StarLustPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(populate_world).add_systems((
            control_player,
            move_camera.after(control_player),
            sync_player_camera.after(control_player),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(StarLustPlugin)
        .run();
}
