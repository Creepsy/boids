use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands : Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color : Color::rgb(0.17, 0.44, 0.74),
            custom_size: Some(Vec2::new(50.0, 50.05)),
            ..default()
        },
        ..default()
    });
}