use crate::{CursorComponent, MainCamera};
use bevy::prelude::*;

pub fn update_cursor_coords(
    mut cursor_q: Query<(&mut Transform, &CursorComponent)>,
    windows: Query<&Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.get_single().unwrap();
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let (mut c_t, _) = cursor_q.get_single_mut().unwrap();
        c_t.translation = Vec3::new(world_position.x, world_position.y, -1.);
    }
}

// thanks refined
// https://github.com/RefinedDev/golf-rs/blob/main/src/misc/mathfuncs.rs
pub fn get_dist(a: Vec3, b: Vec3, t: f32) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt() / t
}
