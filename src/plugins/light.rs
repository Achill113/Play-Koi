use std::f32::consts::PI;

use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
            // .add_systems(Update, animate_light_direction);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });
}

// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in &mut query {
//         transform.rotate_y(time.delta_seconds() * 0.5);
//     }
// }
