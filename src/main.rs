use bevy::{prelude::*, render::camera::ScalingMode};

//use std::f32::consts::PI;


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct Velocity{x: f32}


fn populate_world(mut commands: Commands,
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
    commands.spawn((
        PbrBundle {
            //mesh: meshes.add(Mesh::from(shape::RegularPolygon{sides: 3, radius: 1.0})),//shape::Cube { size: 1.0 })),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::LIME_GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Movable,
        Player,
        Velocity{x: 0.0},
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.92,
    });

    // // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    // camera
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}

pub struct StarLustPlugin;

impl Plugin for StarLustPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        app.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(populate_world)
        .add_system(move_player);
            //.add_system(hello_world)
            //.add_system(greet_people);
    }
}


fn move_player(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<Player>, With<Movable>)>,
) {
    for (mut transform, mut velocity)  in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        //transform.translation += time.delta_seconds() * 2.0 * direction;
        transform.rotate_axis(direction,
                              time.delta_seconds() * 45f32.to_radians());

        velocity.x += time.delta_seconds() * ((input.pressed(KeyCode::Space) as i32) as f32 - 0.1f32);
        if velocity.x <= 0f32
        {
            velocity.x = 0f32;
        }
        //transform.translation += time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32);
        transform.translation += (time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32));
        transform.
        //transform.translate_around(time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32),Quat::IDENTITY);

    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StarLustPlugin)
        .run();
}

