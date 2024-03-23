use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_water::{material::{StandardWaterMaterial, WaterMaterial}, WaterPlugin as BevyWaterPlugin, *};

use super::tile::{TileSettings, GRID_SIZE};

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WaterSettings {
                spawn_tiles: None,
                ..default()
            })
            .add_plugins(BevyWaterPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    water_settings: Res<WaterSettings>,
    tile_settings: Res<TileSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardWaterMaterial>>,
) {
    let size = tile_settings.tile_size * GRID_SIZE as f32;

    let mesh: Handle<Mesh> = meshes.add(
        Cuboid::from_size(Vec3::new(size, 4.0, size))
    );

    let material = materials.add(StandardWaterMaterial {
        base: default(),
        extension: WaterMaterial {
            amplitude: water_settings.amplitude,
            coord_scale: Vec2::new(256.0, 256.0),
            ..default()
        },
    });

    commands.spawn((
        Name::new("Water foo"),
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        NotShadowCaster
    ));
}
