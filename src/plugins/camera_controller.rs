// use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use bevy::window::CursorGrabMode;
// use std::fmt;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup);
            // .add_systems(Update, run_camera_controller);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 15.0)),
            ..default()
        },
        PanOrbitCamera::default()
    ));
}

// /// This is stollen code. I don't know why its 1.0 / 180.0.
// pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;
//
// #[derive(Component)]
// pub struct CameraController {
//     pub enabled: bool,
//     pub initialized: bool,
//     pub sensitivity: f32,
//     pub key_forward: KeyCode,
//     pub key_back: KeyCode,
//     pub key_left: KeyCode,
//     pub key_right: KeyCode,
//     pub key_up: KeyCode,
//     pub key_down: KeyCode,
//     pub mouse_key_pan_grab: MouseButton,
//     pub mouse_key_orbit_grab: MouseButton,
//     pub speed: f32,
//     pub scroll_factor: f32,
//     pub friction: f32,
//     pub pitch: f32,
//     pub yaw: f32,
//     pub velocity: Vec3,
//     pub azimuth: f32,
//     pub elevation: f32,
// }
//
// impl Default for CameraController {
//     fn default() -> Self {
//         Self {
//             enabled: true,
//             initialized: false,
//             sensitivity: 1.0,
//             key_forward: KeyCode::KeyW,
//             key_back: KeyCode::KeyS,
//             key_left: KeyCode::KeyA,
//             key_right: KeyCode::KeyD,
//             key_up: KeyCode::KeyE,
//             key_down: KeyCode::KeyQ,
//             mouse_key_pan_grab: MouseButton::Left,
//             mouse_key_orbit_grab: MouseButton::Right,
//             speed: 15.0,
//             scroll_factor: 0.1,
//             friction: 0.5,
//             pitch: 0.0,
//             yaw: 0.0,
//             velocity: Vec3::ZERO,
//             azimuth: 0.0,
//             elevation: 0.0,
//         }
//     }
// }
//
// impl fmt::Display for CameraController {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "
// Freecam Controls:
//     Mouse\t- Move camera orientation
//     Scroll\t- Adjust movement speed
//     {:?}\t- Hold to grab cursor
//     {:?} & {:?}\t- Fly forward & backwards
//     {:?} & {:?}\t- Fly sideways left & right
//     {:?} & {:?}\t- Fly up & down",
//             self.mouse_key_pan_grab,
//             self.key_forward,
//             self.key_back,
//             self.key_left,
//             self.key_right,
//             self.key_up,
//             self.key_down,
//         )
//     }
// }
//
// #[allow(clippy::too_many_arguments)]
// fn run_camera_controller(
//     time: Res<Time>,
//     mut windows: Query<&mut Window>,
//     mut mouse_events: EventReader<MouseMotion>,
//     mut scroll_events: EventReader<MouseWheel>,
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     key_input: Res<ButtonInput<KeyCode>>,
//     mut mouse_cursor_grab: Local<bool>,
//     mut last_mouse_button_pressed: Local<Option<MouseButton>>,
//     mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
// ) {
//     let dt = time.delta_seconds();
//
//
//     if let Ok((mut transform, mut controller)) = query.get_single_mut() {
//         if !controller.initialized {
//             let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
//             controller.yaw = yaw;
//             controller.pitch = pitch;
//             controller.initialized = true;
//             info!("{}", *controller);
//         }
//         if !controller.enabled {
//             mouse_events.clear();
//             return;
//         }
//
//         let mut axis_input = Vec3::ZERO;
//
//         // Handle key input
//         if key_input.pressed(controller.key_forward) {
//             axis_input.z += 1.0;
//         }
//         if key_input.pressed(controller.key_back) {
//             axis_input.z -= 1.0;
//         }
//         if key_input.pressed(controller.key_right) {
//             axis_input.x += 1.0;
//         }
//         if key_input.pressed(controller.key_left) {
//             axis_input.x -= 1.0;
//         }
//         if key_input.pressed(controller.key_up) {
//             axis_input.y += 1.0;
//         }
//         if key_input.pressed(controller.key_down) {
//             axis_input.y -= 1.0;
//         }
//
//         // Apply movement update
//         if axis_input != Vec3::ZERO {
//             controller.velocity = axis_input.normalize() * controller.speed;
//         } else {
//             let friction = controller.friction.clamp(0.0, 1.0);
//             controller.velocity *= 1.0 - friction;
//             if controller.velocity.length_squared() < 1e-6 {
//                 controller.velocity = Vec3::ZERO;
//             }
//         }
//
//         // Apply transformation
//         transform.translation += controller.velocity.x * dt * Vec3::X
//             + controller.velocity.y * dt * Vec3::Y
//             + -controller.velocity.z * dt * Vec3::Z;
//
//         // Handle scroll zoom
//         let mut scroll = 0.0;
//         for scroll_event in scroll_events.read() {
//             let amount = match scroll_event.unit {
//                 MouseScrollUnit::Line => scroll_event.y,
//                 MouseScrollUnit::Pixel => scroll_event.y / 16.0,
//             };
//             scroll += amount;
//         }
//
//         let scroll_speed = 5.0;
//         let forward = transform.forward();
//
//         transform.translation += forward * scroll * scroll_speed;
//
//         // Handle mouse controls
//         let mut cursor_grab_change = false;
//         if mouse_button_input.just_pressed(controller.mouse_key_pan_grab) || mouse_button_input.just_pressed(controller.mouse_key_orbit_grab) {
//             *mouse_cursor_grab = true;
//             cursor_grab_change = true;
//             if mouse_button_input.just_pressed(controller.mouse_key_pan_grab) {
//                 *last_mouse_button_pressed = Some(controller.mouse_key_pan_grab);
//             } else {
//                 *last_mouse_button_pressed = Some(controller.mouse_key_orbit_grab);
//             }
//         }
//         if mouse_button_input.just_released(controller.mouse_key_pan_grab) || mouse_button_input.just_released(controller.mouse_key_orbit_grab)  {
//             *mouse_cursor_grab = false;
//             cursor_grab_change = true;
//             *last_mouse_button_pressed = None;
//         }
//         let cursor_grab = *mouse_cursor_grab;
//
//         if cursor_grab_change {
//             if cursor_grab {
//                 for mut window in &mut windows {
//                     if !window.focused {
//                         continue;
//                     }
//
//                     window.cursor.grab_mode = CursorGrabMode::Locked;
//                     window.cursor.visible = false;
//                 }
//             } else {
//                 for mut window in &mut windows {
//                     window.cursor.grab_mode = CursorGrabMode::None;
//                     window.cursor.visible = true;
//                 }
//             }
//         }
//
//         // Apply mouse input
//         let mut mouse_delta = Vec2::ZERO;
//         if cursor_grab {
//             for mouse_event in mouse_events.read() {
//                 mouse_delta += mouse_event.delta;
//             }
//         } else {
//             mouse_events.clear();
//         }
//
//         if mouse_delta != Vec2::ZERO {
//             if let Some(mouse_button) = *last_mouse_button_pressed {
//                 match mouse_button {
//                     MouseButton::Left | MouseButton::Right => {
//                         transform.translation += controller.speed * -mouse_delta.x * dt * Vec3::X
//                             + controller.speed * -mouse_delta.y * dt * Vec3::Z;
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
// }
