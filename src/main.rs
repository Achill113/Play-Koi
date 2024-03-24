pub mod plugins;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_rapier3d::prelude::*;
use plugins::{camera_controller::CameraControllerPlugin, hover::HoverPlugin, light::LightPlugin, tile::{TilePlugin, TileSettings, GRID_SIZE}, water::WaterPlugin};

#[derive(Component)]
struct Ground;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, tile_settings: Res<TileSettings>) {
    // camera
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     },
    //     CameraController::default(),
    //     Name::new("Camera"),
    // ));

    // ground
    let size = tile_settings.tile_size * GRID_SIZE as f32;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(size, size)),
            material: materials.add(Color::rgb_u8(114, 162, 208)),
            transform: Transform::from_xyz(-tile_settings.tile_size / 2.0, 0.0, -tile_settings.tile_size / 2.0),
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
        .add_plugins((WaterPlugin, CameraControllerPlugin, LightPlugin, HoverPlugin, TilePlugin))
        .add_systems(Startup, setup)
        .run();
}
