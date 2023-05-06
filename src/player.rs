use crate::common_components::Player;
use bevy::prelude::*;
//use bevy_ecs::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_player(commands: &mut Commands) {
    let radius = 0.5;

    commands
        .spawn(RigidBody::Dynamic)
        .insert((
            Collider::ball(radius),
            TransformBundle::from(Transform::from_xyz(1.0, 2.0, 0.0)),
            Player,
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.2, 0.0, 0.0),
        })
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Sleeping::disabled());
}

pub fn control_player(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in &mut query {
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
        // transform.rotate_axis(direction,
        //                       time.delta_seconds() * 45f32.to_radians());

        // velocity.x += time.delta_seconds() * ((input.pressed(KeyCode::Space) as i32) as f32 - 0.1f32);
        // if velocity.x <= 0f32
        // {
        //     velocity.x = 0f32;
        // }
        //transform.translation += time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32);
        //transform.translation += (time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32));
        //transform.translate_around(time.delta_seconds() * Vec3::new(velocity.x, 0f32, 0f32),Quat::IDENTITY);
        velocity.linvel += time.delta_seconds() * direction;
    }
}
