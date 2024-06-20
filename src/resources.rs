use bevy::prelude::*;

#[derive(Resource)]
pub struct ScrollingCamera{
    pub entity: Option<Entity>,
}

impl Default for ScrollingCamera{
    fn default() -> ScrollingCamera {
        ScrollingCamera { entity: None }
    }
}

#[derive(Resource)]
pub struct CameraVelocity{
    pub v: Vec3,
}

impl Default for CameraVelocity{
    fn default() -> CameraVelocity{
        CameraVelocity{
            v: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}

#[derive(Resource)]
pub struct CapturedMouseRightClickPosition{
    pub pos: Option<Vec2>,
}

impl Default for CapturedMouseRightClickPosition{
    fn default() -> CapturedMouseRightClickPosition {
        CapturedMouseRightClickPosition{
            pos: None,
        }
    }
}

#[derive(Resource)]
pub struct ZoomBound{
    pub max: f32,
    pub min: f32,
}

impl Default for ZoomBound{
    fn default() -> ZoomBound {
        ZoomBound{
            max: 4.0,
            min: 0.4,
        }
    }
}