use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with_children(|parent| {
            parent.spawn((TransformBundle::default(), MainChatAttach));
        });
}

pub fn despawn_camera(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn zoom_and_move_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &Camera)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut mouse_zoom = 0.0;
    for event in mouse_wheel_events.read() {
        mouse_zoom = event.y;
    }
    for (mut transform, camera) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if kb_input.pressed(KeyCode::KeyW) || kb_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if kb_input.pressed(KeyCode::KeyS) || kb_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if kb_input.pressed(KeyCode::KeyA) || kb_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if kb_input.pressed(KeyCode::KeyD) || kb_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if kb_input.pressed(KeyCode::Equal) {
            direction.z += 1.0;
        }
        if kb_input.pressed(KeyCode::Minus) {
            direction.z -= 1.0;
        }
        direction.z += mouse_zoom * 10.;
        if direction.z != 0.0 {
            transform.scale -= Vec3::splat(direction.z * time.delta_seconds());
            transform.scale = transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(1.0));
        }
        if direction.truncate() != Vec2::ZERO {
            let speed = 500.0 * transform.scale.x.sqrt();
            transform.translation +=
                (direction.truncate().normalize() * time.delta_seconds() * speed).extend(0.0);
        }
    }
}
