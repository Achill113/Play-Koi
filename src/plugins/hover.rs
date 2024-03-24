use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::tile::Tile;

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
struct Hovered;

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (hover, handle_hover_enter, handle_hover_exit));
    }
}

fn handle_hover_enter(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Handle<StandardMaterial>, Changed<Hovered>>,
) {
    for material_handle in query.iter() {
        let material = materials.get_mut(material_handle).unwrap();
        material.base_color = Color::BLUE;
    }
}

fn handle_hover_exit(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut removals: RemovedComponents<Hovered>,
    query: Query<(Option<&Tile>, &Handle<StandardMaterial>)>,
) {
    for entity in removals.read() {
        let (tile_option, material_handle) = query.get(entity).unwrap();

        if let Some(tile) = tile_option {
            let material = materials.get_mut(material_handle).unwrap();

            material.base_color = tile.color;
        }
    }
}

fn hover(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    windows: Query<&Window>,
    rapier_context: Res<RapierContext>,
    interactable_query: Query<Entity, With<Interactable>>,
    hovered_query: Query<Entity, With<Hovered>>,
) {
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
        match interactable_query.get(entity) {
            // handle hovering over entities
            Ok(_) => {
                match hovered_query.get_single() {
                    Ok(current_hovered) => {
                        if current_hovered != entity {
                            commands.entity(current_hovered).remove::<Hovered>();
                            commands.entity(entity).insert(Hovered);
                        }
                    }
                    Err(_) => {
                        commands.entity(entity).insert(Hovered);
                    }
                }
            }
            Err(_) => {}
        }
    }
}
