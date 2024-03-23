use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::hover::Interactable;

const TILE_SIZE: f32 = 5.0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Grass,
    Dirt,
    Path,
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub color: Color,
    pub height: f32,
    pub position: Vec3,
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
                        Color::rgba_u8(179, 202, 130, 255),
                ),
                (
                    TileType::Dirt,
                    Color::rgba_u8(125, 96, 65, 255),
                ),
                (
                    TileType::Path,
                    Color::rgba_u8(189, 175, 188, 255),
                ),
            ]),
            tile_heights: HashMap::from([
                (TileType::Grass, 5.0),
                (TileType::Dirt, 4.5),
                (TileType::Path, 5.0),
            ]),
        }
    }
}

impl TileGenerator {
    pub fn generate(&self, tile_type: TileType, position: &Vec2) -> Tile {
        let color = &self.tile_colors[&tile_type];
        let height = &self.tile_heights[&tile_type];

        Tile {
            tile_type,
            color: *color,
            height: *height,
            position: Vec3::new(position.x, height / 2.0, position.y),
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, handle_click);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_generator = TileGenerator::default();

    // test cubes
    let grid_size = 50;

    for row in 0..grid_size {
        for col in 0..grid_size {
            let x = 0.0 - ((grid_size as f32 * TILE_SIZE) / 2.0) + (row as f32 * TILE_SIZE);
            let z = 0.0 - ((grid_size as f32 * TILE_SIZE) / 2.0) + (col as f32 * TILE_SIZE);
            let position = Vec2::new(x, z);
            let tile = tile_generator.generate(TileType::Grass, &position);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(Vec3::new(
                        TILE_SIZE,
                        tile.height,
                        TILE_SIZE,
                    ))),
                    material: materials.add(StandardMaterial{
                        base_color: tile.color,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        tile.position.x,
                        tile.position.y,
                        tile.position.z,
                    ),
                    ..default()
                },
                Collider::cuboid(TILE_SIZE / 2.0, tile.height / 2.0, TILE_SIZE / 2.0),
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
                        TileType::Grass => {
                            tile_generator.generate(TileType::Dirt, &to_top_down(tile.position))
                        }
                        TileType::Dirt => {
                            tile_generator.generate(TileType::Path, &to_top_down(tile.position))
                        }
                        TileType::Path => {
                            tile_generator.generate(TileType::Grass, &to_top_down(tile.position))
                        }
                    };

                    *tile = new_tile.clone();

                    let material_handle = material_query.get(entity).unwrap();

                    let material = materials.get_mut(material_handle).unwrap();

                    material.base_color = new_tile.color;

                    let mut transform = transform_query.get_mut(entity).unwrap();
                    // using TILE_SIZE here because tiles are cubes
                    transform.scale.y = tile.height / TILE_SIZE;
                    transform.translation = tile.position;
                }
                Err(_) => {}
            }
        }
    }
}

pub fn to_top_down(vec_3d: Vec3) -> Vec2 {
    Vec2::new(vec_3d.x, vec_3d.z)
}
