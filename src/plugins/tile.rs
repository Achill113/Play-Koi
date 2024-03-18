use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::hover::Interactable;

const CUBE_SIZE: f32 = 0.5;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
    Path,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub color: Color,
    pub height: f32,
}

pub struct TileGenerator {
    tile_colors: HashMap<TileType, Color>,
    tile_heights: HashMap<TileType, f32>,
}

impl Default for TileGenerator {
    fn default() -> Self {
        Self {
            tile_colors: HashMap::from([
                (
                    TileType::Grass, 
                    Color::rgb_u8(179,202,130),
                ),
                (
                    TileType::Dirt,
                    Color::rgb_u8(125,96,65),
                ),
                (
                    TileType::Water,
                    Color::rgb_u8(35,137,218),
                ),
                (
                    TileType::Path,
                    Color::rgb_u8(189,175,188),
                ),
            ]),
            tile_heights: HashMap::from([
                (
                    TileType::Grass,
                    1.0,
                ),
                (
                    TileType::Dirt,
                    0.5,
                ),
                (
                    TileType::Water,
                    0.01,
                ),
                (
                    TileType::Path,
                    1.0,
                ),
            ]),
        }
    }
}

impl TileGenerator {
    pub fn generate(&self, tile_type: TileType) -> Tile {
        let color = &self.tile_colors[&tile_type];
        let height = &self.tile_heights[&tile_type];

        Tile {
            tile_type,
            color: *color,
            height: *height,
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

    for row in 0..grid_size {
        for col in 0..grid_size {
            let tile = tile_generator.generate(TileType::Grass);
            let x = 0.0 - ((grid_size as f32 * cube_size) / 2.0) + (row as f32 * cube_size);
            let z = 0.0 - ((grid_size as f32 * cube_size) / 2.0) + (col as f32 * cube_size);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(Vec3::new(cube_size, tile.height, cube_size))),
                    material: materials.add(tile.color),
                    transform: Transform::from_xyz(x, tile.height / 2.0, z),
                    ..default()
                },
                Collider::cuboid(cube_size / 2.0, tile.height / 2.0, cube_size / 2.0),
                Interactable,
                tile,
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
    mut transform_query: Query<&mut Transform>,
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
            match tile_query.get_mut(entity) {
                Ok(mut tile) => {
                    let new_tile = match tile.tile_type {
                        TileType::Grass => tile_generator.generate(TileType::Dirt),
                        TileType::Dirt => tile_generator.generate(TileType::Path),
                        TileType::Path => tile_generator.generate(TileType::Water),
                        _ => tile_generator.generate(TileType::Grass)
                    };

                    *tile = new_tile.clone();

                    let material_handle = material_query.get(entity).unwrap();

                    let material = materials.get_mut(material_handle).unwrap();

                    material.base_color = new_tile.color;

                    let mut transform = transform_query.get_mut(entity).unwrap();
                    transform.scale.y = tile.height;
                }
                Err(_) => {}
            }

        }
    }
}
