use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::{parry::utils::Array1, prelude::*};
use rand::prelude::*;

use super::hover::{Colored, Interactable};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
    Path,
}

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub color: Color,
}

pub struct TileGenerator {
    tile_colors: HashMap<TileType, Vec<Color>>,
}

impl Default for TileGenerator {
    fn default() -> Self {
        Self {
            tile_colors: HashMap::from([
                (
                    TileType::Grass, 
                    vec!(
                        Color::rgb_u8(95, 195, 20),
                        Color::rgb_u8(121, 208, 33),
                        Color::rgb_u8(161, 223, 80),
                        Color::rgb_u8(193, 243, 118),
                        Color::rgb_u8(85, 194, 51),
                        Color::rgb_u8(55, 174, 15)
                    )
                ),
                (
                    TileType::Dirt,
                    vec!(
                        Color::rgb_u8(166, 139, 113),
                        Color::rgb_u8(136, 103, 78),
                        Color::rgb_u8(102, 65, 33),
                        Color::rgb_u8(84, 45, 28)
                    )
                ),
                (
                    TileType::Water,
                    vec!(
                        Color::rgb_u8(121, 161, 126),
                        Color::rgb_u8(199, 230, 215),
                        Color::rgb_u8(95, 122, 166),
                        Color::rgb_u8(123, 152, 196)
                    )
                ),
                (
                    TileType::Path,
                    vec!(
                        Color::rgb_u8(228, 228, 228),
                        Color::rgb_u8(205, 207, 204),
                        Color::rgb_u8(183, 184, 179),
                        Color::rgb_u8(154, 156, 153),
                        Color::rgb_u8(93, 94, 96)
                    )
                ),
            ])
        }
    }
}

impl TileGenerator {
    pub fn generate(&self, tile_type: TileType) -> Tile {
        let colors = &self.tile_colors[&tile_type];
        let index = rand::thread_rng().gen_range(0..colors.len() - 1);

        let Some(color) = colors.get_at(index) else {
            return Tile {
                tile_type,
                color: Color::WHITE,
            };
        };

        Tile {
            tile_type,
            color: *color
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, handle_click);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let tile_generator = TileGenerator::default();

    // test cubes
    let grid_size = 50;
    let cube_size = 5.0;
    let cube_height = 1.0;

    for row in 0..grid_size {
        for col in 0..grid_size {
            let tile = tile_generator.generate(TileType::Grass);
            let x = 0.0 - ((grid_size as f32 * cube_size) / 2.0) + (row as f32 * cube_size);
            let z = 0.0 - ((grid_size as f32 * cube_size) / 2.0) + (col as f32 * cube_size);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(Vec3::new(cube_size, cube_height, cube_size))),
                    material: materials.add(tile.color),
                    transform: Transform::from_xyz(x, cube_height / 2.0, z),
                    ..default()
                },
                Collider::cuboid(cube_size / 2.0, cube_height / 2.0, cube_size / 2.0),
                Interactable,
                Colored {
                    color: tile.color,
                },
            ));
        }
    }
}

fn handle_click(
    mut materials: ResMut<Assets<StandardMaterial>>,
    rapier_context: Res<RapierContext>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    material_query: Query<&Handle<StandardMaterial>, With<Interactable>>,
    mut tile_query: Query<&mut Tile, With<Tile>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let tile_generator = TileGenerator::default();

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
            let mut tile = tile_query.get_mut(entity).unwrap();

            let new_tile = match tile.tile_type {
                TileType::Grass => tile_generator.generate(TileType::Dirt),
                _ => tile_generator.generate(TileType::Grass)
            };

            tile.tile_type = new_tile.tile_type;
            tile.color = new_tile.color;

            let material_handle = material_query.get(entity).unwrap();

            let material = materials.get_mut(material_handle).unwrap();
            material.base_color = tile.color;
        }
    }
}
