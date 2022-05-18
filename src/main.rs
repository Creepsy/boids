mod boids;
mod simulation_configs;

use bevy::prelude::*;
use simulation_configs::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                resizable: false,
                title: "Boids".to_string(),
                ..default()
            })
        .add_startup_system(init_scene)
        .add_startup_system(boids::spawn_boids_randomly::<{simulation_configs::BOID_COUNT}>)
        .add_system(boids::update_boid_positions)
        .add_plugins(DefaultPlugins)
        .run();
}

fn init_scene(mut commands : Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}