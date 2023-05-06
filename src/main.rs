use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub mod camera_handling;
pub mod common_components;
pub mod player;

use crate::camera_handling::move_camera;
use crate::camera_handling::spawn_camera;
use crate::camera_handling::sync_player_camera;
use crate::player::move_player;
use crate::player::spawn_player;

#[derive(Component)]
struct VelocityCustom {
    x: f32,
}

fn populate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    //commands.spawn(BackgroundColor(Color::BLACK));

    // ground plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(10.0).into()),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::BLACK,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });

    // cube
    // commands.spawn((
    //     PbrBundle {
    //         //mesh: meshes.add(Mesh::from(shape::RegularPolygon{sides: 3, radius: 1.0})),//shape::Cube { size: 1.0 })),
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::LIME_GREEN,
    //             ..default()
    //         }),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..default()
    //     },
    //     Movable,
    //     Player,
    //     VelocityCustom{x: 0.0},
    // ));

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

    // commands
    //     .spawn((Collider::cuboid(10.0, 0.1, 10.0),
    //     TransformBundle::from(Transform::from_xyz(0.0,  -1.0, 0.0))
    //     ));
    /* Create Player */
    spawn_player(&mut commands);

    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert((
    //         Collider::ball(0.5),
    //         TransformBundle::from(Transform::from_xyz(-5.0, 3.0, 0.0)),
    //     ))
    //     .insert(Restitution::coefficient(1.0))
    //     // .insert(Velocity {
    //     //     linvel: Vec3::new(0.0, 0.0, 0.0),
    //     //     angvel: Vec3::new(0.2, 0.0, 0.0),
    //     // })
    //     .insert(GravityScale(0.5))
    //     .insert(Sleeping::disabled());
}

pub struct StarLustPlugin;

impl Plugin for StarLustPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        //app.insert_resource(ClearColor(Color::BLACK))
        app.add_startup_system(populate_world).add_systems((
            move_player,
            move_camera.after(move_player),
            sync_player_camera.after(move_player),
        ));
        //.add_system(print_ball_altitude);
        //.add_system(hello_world)
        //.add_system(greet_people);
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
