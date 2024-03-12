//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the [`CameraControllerPlugin`] and add the plugin to your App.
//! - Attach the [`CameraController`] component to an entity with a [`Camera3dBundle`].

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use std::{f32::consts::*, fmt};

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
    }
}

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but I'm guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub mouse_key_pan_grab: MouseButton,
    pub mouse_key_orbit_grab: MouseButton,
    pub speed: f32,
    pub scroll_factor: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyQ,
            mouse_key_pan_grab: MouseButton::Left,
            mouse_key_orbit_grab: MouseButton::Right,
            speed: 15.0,
            scroll_factor: 0.1,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Freecam Controls:
    Mouse\t- Move camera orientation
    Scroll\t- Adjust movement speed
    {:?}\t- Hold to grab cursor
    {:?} & {:?}\t- Fly forward & backwards
    {:?} & {:?}\t- Fly sideways left & right
    {:?} & {:?}\t- Fly up & down",
            self.mouse_key_pan_grab,
            self.key_forward,
            self.key_back,
            self.key_left,
            self.key_right,
            self.key_up,
            self.key_down,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_cursor_grab: Local<bool>,
    mut last_mouse_button_pressed: Local<Option<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();


    if let Ok((mut transform, mut controller)) = query.get_single_mut() {
        if !controller.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
            info!("{}", *controller);
        }
        if !controller.enabled {
            mouse_events.clear();
            return;
        }

        let mut scroll = 0.0;
        for scroll_event in scroll_events.read() {
            let amount = match scroll_event.unit {
                MouseScrollUnit::Line => scroll_event.y,
                MouseScrollUnit::Pixel => scroll_event.y / 16.0,
            };
            scroll += amount;
        }

        let mut axis_input = Vec3::ZERO;

        let scroll_speed = 5.0;
        axis_input.z += scroll * scroll_speed;

        // Handle key input
        if key_input.pressed(controller.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(controller.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(controller.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(controller.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(controller.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(controller.key_down) {
            axis_input.y -= 1.0;
        }


        let mut cursor_grab_change = false;
        if mouse_button_input.just_pressed(controller.mouse_key_pan_grab) || mouse_button_input.just_pressed(controller.mouse_key_orbit_grab) {
            *mouse_cursor_grab = true;
            cursor_grab_change = true;
            if mouse_button_input.just_pressed(controller.mouse_key_pan_grab) {
                *last_mouse_button_pressed = Some(controller.mouse_key_pan_grab);
            } else {
                *last_mouse_button_pressed = Some(controller.mouse_key_orbit_grab);
            }
        }
        if mouse_button_input.just_released(controller.mouse_key_pan_grab) || mouse_button_input.just_released(controller.mouse_key_orbit_grab)  {
            *mouse_cursor_grab = false;
            cursor_grab_change = true;
            *last_mouse_button_pressed = None;
        }
        let cursor_grab = *mouse_cursor_grab;

        // Apply movement update
        if axis_input != Vec3::ZERO {
            controller.velocity = axis_input.normalize() * controller.speed;
        } else {
            let friction = controller.friction.clamp(0.0, 1.0);
            controller.velocity *= 1.0 - friction;
            if controller.velocity.length_squared() < 1e-6 {
                controller.velocity = Vec3::ZERO;
            }
        }
        let forward = *transform.forward();
        let right = *transform.right();
        transform.translation += controller.velocity.x * dt * right
            + controller.velocity.y * dt * Vec3::Y
            + controller.velocity.z * dt * forward;

        // Handle cursor grab
        if cursor_grab_change {
            if cursor_grab {
                for mut window in &mut windows {
                    if !window.focused {
                        continue;
                    }

                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
            } else {
                for mut window in &mut windows {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if cursor_grab {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }
        } else {
            mouse_events.clear();
        }

        if mouse_delta != Vec2::ZERO {
            if let Some(mouse_button) = *last_mouse_button_pressed {
                match mouse_button {
                    MouseButton::Left => {
                        // Apply look update
                        controller.pitch = (controller.pitch
                                            - mouse_delta.y * RADIANS_PER_DOT * controller.sensitivity)
                            .clamp(-PI / 2., PI / 2.);
                        controller.yaw -= mouse_delta.x * RADIANS_PER_DOT * controller.sensitivity;
                        transform.rotation =
                            Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
                    }
                    MouseButton::Right => {
                        mouse_delta *= controller.speed;
                        transform.translation += -mouse_delta.x * dt * right
                            + mouse_delta.y * dt * Vec3::Y;
                    }
                    _ => {}
                }
            }
        }
    }
}
