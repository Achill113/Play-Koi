use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_water::{material::{StandardWaterMaterial, WaterMaterial}, WaterPlugin as BevyWaterPlugin, *};

use super::tile::{TileSettings, GRID_SIZE};

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WaterSettings {
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
        Plane3d::default().mesh().size(size, size)
    );

    let material = materials.add(StandardWaterMaterial {
        base: default(),
        extension: WaterMaterial {
            amplitude: water_settings.amplitude,
            coord_scale: Vec2::new(256.0, 256.0),
            ..default()
        }
        // base_color: Color::rgba_u8(114, 162, 208, 100),
        // alpha_mode: AlphaMode::Add,
        // ..default()
    });

    commands.spawn((
        Name::new("Water"),
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_xyz(-tile_settings.tile_size / 2.0, 4.0, -tile_settings.tile_size / 2.0),
            ..default()
        },
        NotShadowCaster
    ));
}
