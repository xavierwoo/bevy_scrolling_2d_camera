use bevy::prelude::*;

mod resources;
mod systems;
mod states;

use resources::*;
use systems::*;
use states::*;

pub struct ScrollingCameraPlugin;

impl Plugin for ScrollingCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ScrollingCamera>()
        .init_resource::<CameraVelocity>()
        .init_resource::<CapturedMouseRightClickPosition>()
        .init_state::<CameraState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_move.run_if(in_state(CameraState::Scrolling)))
        .add_systems(Update, capture_mouse_right_click_for_scrolling.run_if(in_state(CameraState::Idling)))
        .add_systems(Update, control_camera_movment.run_if(in_state(CameraState::Scrolling)))
        .add_systems(Update, camera_zoom)
        ;
    }
}

