pub mod plugins;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_rapier3d::prelude::*;
use plugins::{camera_controller::{CameraController, CameraControllerPlugin}, hover::HoverPlugin, light::LightPlugin, tile::TilePlugin};

#[derive(Component)]
struct Ground;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });

    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2000.0, 2000.0)),
            material: materials.add(Color::rgba_u8(0, 154, 23, 0)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Ground,
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Play Koi".into(),
                    ..default()
                }),
                ..default()
            })
            .build()
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((CameraControllerPlugin, LightPlugin, HoverPlugin, TilePlugin))
        .add_systems(Startup, setup)
        .run();
}
