use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct Colored {
    pub color: Color
}

#[derive(Component)]
struct Hovered;

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hover);
    }
}

fn hover(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    rapier_context: Res<RapierContext>,
    material_query: Query<&Handle<StandardMaterial>, With<Interactable>>,
    hovered_query: Query<(Entity, Option<&Colored>, &Handle<StandardMaterial>), (With<Hovered>, With<Colored>)>,
    interactable_query: Query<Option<Entity>, With<Interactable>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // reset entity colors
    for (entity, color, material_handle) in &hovered_query {
        if let Some(color) = color {
            let material = materials.get_mut(material_handle).unwrap();

            material.base_color = color.color;

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
        match interactable_query.get(entity) {
            // handle hovering over entities
            Ok(_) => {
                commands.entity(entity).insert(Hovered);

                let material_handle = material_query.get(entity).unwrap();

                let material = materials.get_mut(material_handle).unwrap();
                material.base_color = Color::BLUE;
            }
            Err(_) => {}
        }
    }
}
