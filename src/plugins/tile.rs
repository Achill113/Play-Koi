use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{hover::Interactable, tools::ToolModeState};

pub const GRID_SIZE: u32 = 50;

#[derive(Resource)]
pub struct TileSettings {
    pub tile_size: f32,
}

impl Default for TileSettings {
    fn default() -> Self {
        Self {
            tile_size: 5.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Grass,
    Dirt,
    Path,
    Water,
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
                (
                    TileType::Water,
                    Color::rgba_u8(114, 162, 208, 0),
                )
            ]),
            tile_heights: HashMap::from([
                (TileType::Grass, 5.0),
                (TileType::Dirt, 4.5),
                (TileType::Path, 5.0),
                (TileType::Water, 4.0),
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
        app
            .insert_resource(TileSettings::default())
            .add_systems(Startup, setup)
            .add_systems(Update, handle_click);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tile_settings: Res<TileSettings>,
) {
    let tile_generator = TileGenerator::default();

    // test cubes
    let grid_size = 50;

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let x = 0.0 - ((grid_size as f32 * tile_settings.tile_size) / 2.0) + (row as f32 * tile_settings.tile_size);
            let z = 0.0 - ((grid_size as f32 * tile_settings.tile_size) / 2.0) + (col as f32 * tile_settings.tile_size);
            let position = Vec2::new(x, z);
            let tile = tile_generator.generate(TileType::Grass, &position);

            let material = StandardMaterial {
                base_color: tile.color,
                ..default()
            };

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::from_size(Vec3::new(
                        tile_settings.tile_size,
                        tile.height,
                        tile_settings.tile_size,
                    ))),
                    material: materials.add(material),
                    transform: Transform::from_xyz(
                        tile.position.x,
                        tile.position.y,
                        tile.position.z,
                    ),
                    ..default()
                },
                Collider::cuboid(tile_settings.tile_size / 2.0, tile.height / 2.0, tile_settings.tile_size / 2.0),
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
    tile_settings: Res<TileSettings>,
    state: Res<State<ToolModeState>>,
) {
    if *state.get() != ToolModeState::None && mouse_button_input.just_pressed(MouseButton::Left) {
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
                    let new_tile: Option<Tile> = match *state.get() {
                        ToolModeState::Grass => {
                            Some(tile_generator.generate(TileType::Grass, &to_top_down(tile.position)))
                        }
                        ToolModeState::Dirt => {
                            Some(tile_generator.generate(TileType::Dirt, &to_top_down(tile.position)))
                        }
                        ToolModeState::Path => {
                            Some(tile_generator.generate(TileType::Path, &to_top_down(tile.position)))
                        }
                        ToolModeState::Water => {
                            Some(tile_generator.generate(TileType::Water, &to_top_down(tile.position)))
                        }
                        _ => None
                    };

                    if let Some(new_tile) = new_tile {
                        *tile = new_tile.clone();

                        let material_handle = material_query.get(entity).unwrap();

                        let material = materials.get_mut(material_handle).unwrap();

                        material.base_color = new_tile.color;

                        if tile.tile_type == TileType::Water {
                            material.alpha_mode = AlphaMode::Blend;
                        } else {
                            material.alpha_mode = AlphaMode::Opaque;
                        }

                        let mut transform = transform_query.get_mut(entity).unwrap();
                        // using tile_size here because tiles are cubes
                        transform.scale.y = tile.height / tile_settings.tile_size;
                        transform.translation = tile.position;
                    }
                }
                Err(_) => {}
            }
        }
    }
}

pub fn to_top_down(vec_3d: Vec3) -> Vec2 {
    Vec2::new(vec_3d.x, vec_3d.z)
}
