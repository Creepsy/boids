use bevy::prelude::*;

use rand::prelude::*;

#[derive(Component, Default)]
pub struct Boid;

#[derive(Bundle, Default)]
pub struct BoidBundle {
    #[bundle]
    sprite_bundle : SpriteBundle,
    boid : Boid
}

impl BoidBundle {
    pub fn new(position : Vec2) -> BoidBundle {
        BoidBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.4, 0.2, 0.5),
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}

pub fn spawn_boids_randomly<const BOID_COUNT : usize>(windows: Res<Windows>, mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let main_window : &Window = windows.get_primary().unwrap();

    let x_range = -main_window.width() as isize / 2..=main_window.width() as isize / 2;
    let y_range = -main_window.height() as isize / 2..=main_window.height() as isize / 2;

    for _ in 0..BOID_COUNT {
        let x = rng.gen_range(x_range.clone()) as f32;
        let y = rng.gen_range(y_range.clone()) as f32;

        commands.spawn().insert_bundle(BoidBundle::new(Vec2::new(x, y)));
    }
}