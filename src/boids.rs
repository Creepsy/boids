use bevy::prelude::*;

use rand::prelude::*;

#[derive(Component, Default)]
pub struct Boid;

#[derive(Component, Default)]
pub struct Velocity(Vec2);

#[derive(Bundle, Default)]
pub struct BoidBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    boid: Boid,
    velocity: Velocity
}

impl BoidBundle {
    pub fn new(position: Vec2, asset_server: &Res<AssetServer>) -> BoidBundle {
        let mut rng = rand::thread_rng();

        BoidBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    ..default()
                },
                texture: asset_server.load("textures/boid.png"),
                ..default()
            },
            velocity: Velocity(Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize()),
            ..default()
        }
    }
}

pub fn spawn_boids_randomly<const BOID_COUNT: usize>(windows: Res<Windows>, asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let main_window: &Window = windows.get_primary().unwrap();

    let x_range = -main_window.width() as isize / 2..=main_window.width() as isize / 2;
    let y_range = -main_window.height() as isize / 2..=main_window.height() as isize / 2;

    for _ in 0..BOID_COUNT {
        let x = rng.gen_range(x_range.clone()) as f32;
        let y = rng.gen_range(y_range.clone()) as f32;

        commands.spawn().insert_bundle(BoidBundle::new(Vec2::new(x, y), &asset_server));
    }
}

pub fn update_boid_positions(mut to_update: Query<(&mut Transform, &mut Velocity), With<Boid>>) {
    for (mut boid_transform, boid_velocity) in to_update.iter_mut() {
        boid_transform.translation += 0.25 * boid_velocity.0.extend(0.0);
        boid_transform.rotation = get_boid_rotation(boid_velocity.0);
    }
}

fn get_boid_rotation(direction : Vec2) -> Quat {    
    Quat::from_rotation_z(Vec2::angle_between(Vec2::Y, direction))
}