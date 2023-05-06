use bevy::prelude::*;
// https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

use crate::common_components::Player;

#[derive(Component)]
pub struct MovableCamera {
    pub follow_user: bool,
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

pub fn spawn_camera(commands: &mut Commands) {
    // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    // camera
    // commands.spawn(Camera3dBundle {
    //     projection: OrthographicProjection {
    //         scale: 3.0,
    //         scaling_mode: ScalingMode::FixedVertical(2.0),
    //         ..default()
    //     }
    //     .into(),
    //     transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();
    commands.spawn((
        Camera3dBundle {
            //transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
        MovableCamera { follow_user: true },
    ));
}

// https://www.reddit.com/r/bevy/comments/128q8ps/beginner_question_have_orbit_camera_follow_player/
pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut PanOrbitCamera, &mut Transform, &mut MovableCamera), Without<Player>>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok((mut camera, mut camera_transform, mut movable_camera)) = camera.get_single_mut() else { return };

    if !movable_camera.follow_user {
        return;
    }

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}

pub fn move_camera(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MovableCamera)>, // TODO, GET THE SINGLE QUERY NOW AS TUPLE
) {
    for (mut transform, mut camera) in &mut query {
        //TODO:
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::F) {
            // let Ok((mut camera)) = query.get_single_mut() else { return };
            camera.follow_user = true;
        }
        if input.pressed(KeyCode::F) {
            // let Ok((mut camera)) = query.get_single_mut() else { return };
            camera.follow_user = true;
        }
        if input.pressed(KeyCode::E) {
            // let Ok((mut camera)) = query.get_single_mut() else { return };
            camera.follow_user = false;
        }
        transform.rotate_axis(direction, time.delta_seconds() * 45f32.to_radians());
    }
}
