mod boids;
use bevy::prelude::*;

const BOID_COUNT: usize = 10;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
                width: 800.0,
                height: 500.0,
                resizable: false,
                ..default()
            })
        .add_startup_system(init_scene)
        .add_startup_system(boids::spawn_boids_randomly::<BOID_COUNT>)
        .add_plugins(DefaultPlugins)
        .run();
}

fn init_scene(mut commands : Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}