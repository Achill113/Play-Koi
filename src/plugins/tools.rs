use bevy::prelude::*;

#[derive(Component)]
pub struct ToolModeStateDisplay;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolModeState {
    None,
    Grass,
    Dirt,
    Path,
    Water,
}

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(ToolModeState::None)
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_input, display_state));
    }
}

fn setup(mut commands: Commands) {
    let root_uinode = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,

                ..default()
            },
            ..default()
        })
        .id();

    let left_column = commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        },
        ..default()
    }).with_children(|builder| {
        builder.spawn((
            TextBundle::from_section(
                "Tool Mode: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                max_width: Val::Px(300.),
                ..default()
            }),
            ToolModeStateDisplay
        ));
    }).id();

    commands
        .entity(root_uinode)
        .push_children(&[left_column]);
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<ToolModeState>>,
    mut next_state: ResMut<NextState<ToolModeState>>
) {
    if keyboard_input.pressed(KeyCode::KeyG) && *state.get() != ToolModeState::Grass {
        next_state.set(ToolModeState::Grass);
    }

    if keyboard_input.pressed(KeyCode::KeyD) && *state.get() != ToolModeState::Dirt {
        next_state.set(ToolModeState::Dirt);
    }

    if keyboard_input.pressed(KeyCode::KeyP) && *state.get() != ToolModeState::Path {
        next_state.set(ToolModeState::Path);
    }

    if keyboard_input.pressed(KeyCode::KeyW) && *state.get() != ToolModeState::Water {
        next_state.set(ToolModeState::Water);
    }

    if keyboard_input.pressed(KeyCode::Escape) && *state.get() != ToolModeState::None {
        next_state.set(ToolModeState::None);
    }
}

fn display_state(state: Res<State<ToolModeState>>, mut text_query: Query<&mut Text, With<ToolModeStateDisplay>>) {
    let text = &mut text_query.single_mut();

    let mode = state.get();

    text.sections[0].value = format!("Tool Mode: {:?}", mode);
}
