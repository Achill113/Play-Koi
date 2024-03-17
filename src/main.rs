pub mod plugins;

use bevy::{prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}};
use bevy_rapier3d::prelude::*;
use plugins::{camera_controller::{CameraController, CameraControllerPlugin}, light::LightPlugin};

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Interactable;

#[derive(Component)]
struct GroundTile {
    color: Color
}

#[derive(Component)]
struct Hovered;

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


// commands.entity(entity).remove::<Hovered>();

fn interaction(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    rapier_context: Res<RapierContext>,
    material_query: Query<(Entity, &Handle<StandardMaterial>), With<Interactable>>,
    hovered_query: Query<(Entity, Option<&GroundTile>, &Handle<StandardMaterial>), With<Hovered>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, ground_tile, material_handle) in &hovered_query {
        if let Some(ground_tile) = ground_tile {
            let material = materials.get_mut(material_handle).unwrap();

            material.base_color = ground_tile.color;

            commands.entity(entity).remove::<Hovered>();
        }
    }

    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    if let Some((entity, _toi)) = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::default(),
    ) {
        commands.entity(entity).insert(Hovered);

        let (_entity, material_handle) = material_query.get(entity).unwrap();

        let material = materials.get_mut(material_handle).unwrap();
        material.base_color = Color::RED;
    }
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
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
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

    // test cube
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
        GroundTile {
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
        .add_plugins((CameraControllerPlugin, LightPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (interaction, draw_cursor))
        .run();
}
