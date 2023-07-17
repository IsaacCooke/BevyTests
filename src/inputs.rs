use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (keyboard_input, grab_mouse));
    }
}

fn keyboard_input(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("The 'A' key is pressed");
    }
    if keyboard_input.just_pressed(KeyCode::A) {
        println!("The 'A' key was just pressed");
    }
    if keyboard_input.just_released(KeyCode::A) {
        println!("The 'A' key was just released");
    }
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}