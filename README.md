# Bevy scrolling 2d camera plugin

Here is a simple 2d camera plugin for bevy engine, supporting scrolling with mouse right dragging and zooming with mouse wheel.

It is suitable for RTS or Simulation 2D games.

Check the cargo project in _example_ folder to see how it works.

| bevy  | bevy_scrolling_2d_camera |
|-------|---------------------|
| 0.13  | 0.1.*                 |

## Idea of the dragging function

By pressing down the right mouse button, the plugin records the cursor position.
The vector from this record position to the current cursor position is used as the camera velocity.
The camera updates its translation based on the calculated veclocity.

## Things added to your project with this plugin

This plugin is minimum, but it adds the following things to your project.

1. A resource holding the entity of the camera.
```rust
#[derive(Resource)]
pub struct ScrollingCamera{
    pub entity: Option<Entity>,
}
```

2. A resource storing the velocity of the camera

```rust
#[derive(Resource)]
pub struct CameraVelocity{
    pub v: Vec3,
}
```

3. A resource marking the cursor position when pressing down the right mouse button.

```rust
#[derive(Resource)]
pub struct CapturedMouseRightClickPosition{
    pub pos: Option<Vec2>,
}
```

4. A state enum marking the current state of the camera.

```rust
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    Idling,
    Scrolling,
}
```

5. Four updating systems to control the movement of the camera.

```rust
pub fn camera_move(
    mut query: Query<&mut Transform, With<Camera>>,
    velocity: Res<CameraVelocity>,
    time: Res<Time>,
    scrolling_camera: Res<ScrollingCamera>,
) {/*...*/}
```

```rust
pub fn capture_mouse_right_click_for_scrolling(
    mut commands: Commands,
    windows: Query<&Window>,
    input:  Res<ButtonInput<MouseButton>>,
    mut click_pos : ResMut<CapturedMouseRightClickPosition>,
) {/*...*/}
```

```rust
pub fn control_camera_movment(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    input:  Res<ButtonInput<MouseButton>>,
    click_pos : Res<CapturedMouseRightClickPosition>,
    mut velocity: ResMut<CameraVelocity>,
    mut gizmos: Gizmos,
    scrolling_camera: Res<ScrollingCamera>,
){/*...*/}
```

```rust
pub fn camera_zoom(
    mut mouse_wheel_event: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection>,
    time: Res<Time>,
    scrolling_camera: Res<ScrollingCamera>,
) {/*...*/}
```