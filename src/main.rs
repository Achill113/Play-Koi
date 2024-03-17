pub mod plugins;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_rapier3d::prelude::*;
use plugins::{camera_controller::{CameraController, CameraControllerPlugin}, hover::{Colored, HoverPlugin, Interactable}, light::LightPlugin};

#[derive(Component)]
struct Ground;

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);


    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        point + ground.up() * 0.01,
        Direction3d::new_unchecked(ground.up()), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
}

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
            mesh: meshes.add(Plane3d::default().mesh().size(200.0, 200.0)),
            material: materials.add(Color::rgb_u8(0, 154, 23)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ground,
    ));

    // test cubes
    let cube_size = 5.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(cube_size, cube_size, cube_size))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, cube_size / 2.0, 0.0),
            ..default()
        },
        Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
        Interactable,
        Colored {
            color: Color::BLUE,
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(cube_size, cube_size, cube_size))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(-10.0, cube_size / 2.0, 0.0),
            ..default()
        },
        Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
        Colored {
            color: Color::BLUE,
        },
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((CameraControllerPlugin, LightPlugin, HoverPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}
