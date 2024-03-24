use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(240, 228, 76),
        brightness: 1000.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 50.0, -2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
