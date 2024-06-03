use bevy::prelude::*;

use super::resources::*;
use super::states::*;

pub fn spawn_camera(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    mut scrolling_camera: ResMut<ScrollingCamera>,
) {
    let entity = commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            projection: OrthographicProjection{
                ..default()
            },
            ..default()
        }
    ).id();
    scrolling_camera.entity = Some(entity);
    println!("Spawn camera");
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 10.0;
}

pub fn camera_move(
    mut query: Query<&mut Transform, With<Camera>>,
    velocity: Res<CameraVelocity>,
    time: Res<Time>,
    scrolling_camera: Res<ScrollingCamera>,
) {
    if velocity.v == Vec3::ZERO {
        return;
    }
    if let Some(entity) = scrolling_camera.entity {
        query.get_mut(entity).unwrap().translation 
            += velocity.v * time.delta_seconds();
    }
}

pub fn capture_mouse_right_click_for_scrolling(
    mut commands: Commands,
    windows: Query<&Window>,
    input:  Res<ButtonInput<MouseButton>>,
    mut click_pos : ResMut<CapturedMouseRightClickPosition>,
) {
    if ! input.pressed(MouseButton::Right) {return}

    let window = windows.single();
    click_pos.pos =  window.cursor_position();

    commands.insert_resource(NextState(Some(CameraState::Scrolling)));
}

pub fn control_camera_movment(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    input:  Res<ButtonInput<MouseButton>>,
    click_pos : Res<CapturedMouseRightClickPosition>,
    mut velocity: ResMut<CameraVelocity>,
    mut gizmos: Gizmos,
    scrolling_camera: Res<ScrollingCamera>,
){
    if let Some(camera_entity) = scrolling_camera.entity{
        if ! input.pressed(MouseButton::Right){
            commands.insert_resource(NextState(Some(CameraState::Idling)));
            return
        }
        
        let window = window_query.single();
        let cursor_position = window.cursor_position();
        if cursor_position == None {return}
        let velocity2 = cursor_position.unwrap() - click_pos.pos.unwrap();
        velocity.v.x = velocity2.x;
        velocity.v.y = - velocity2.y;
        
        let (camera, camera_transform) = camera_query.get(camera_entity).unwrap();

        let arrow_start = click_pos.pos.
                                and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)).unwrap();

        let arrow_end = cursor_position
                                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)).unwrap();

        gizmos.arrow_2d(
            arrow_start,
            arrow_end,
            Color::BLACK,
            
        );
    }
}