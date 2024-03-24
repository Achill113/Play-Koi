use bevy::{pbr::NotShadowCaster, prelude::*};

use super::tile::{TileSettings, GRID_SIZE};

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    tile_settings: Res<TileSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = tile_settings.tile_size * GRID_SIZE as f32;

    let mesh: Handle<Mesh> = meshes.add(
        Plane3d::default().mesh().size(size, size)
    );

    let material = materials.add(StandardMaterial {
        base_color: Color::rgba_u8(114, 162, 208, 100),
        alpha_mode: AlphaMode::Add,
        ..default()
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
