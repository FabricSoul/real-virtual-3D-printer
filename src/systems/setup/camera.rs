use crate::components::camera::MainCamera;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub fn camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
                ..default()
            },
            PanOrbitCamera::default(),
        ))
        .insert(MainCamera);
}
