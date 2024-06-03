use bevy::prelude::*;
use bevy_scrolling_2d_camera::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(ScrollingCameraPlugin)
    .add_systems(Startup, spawn_sprite)
    .run()
    ;
}

fn spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn(
        SpriteBundle{
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture : asset_server.load("sprite.png"),
            ..default()
        },
    );
}