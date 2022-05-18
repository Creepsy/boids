#[path ="simulation_configs.rs"] mod simulation_configs;

use bevy::prelude::*;
use rand::prelude::*;
use simulation_configs::*;

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
                    scale: Vec3::new(0.5, 0.5, 0.0),
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

pub fn spawn_boids_randomly<const COUNT: usize>(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let x_range = -WINDOW_WIDTH as isize / 2..=WINDOW_WIDTH as isize / 2;
    let y_range = -WINDOW_HEIGHT as isize / 2..=WINDOW_HEIGHT as isize / 2;

    for _ in 0..COUNT {
        let x = rng.gen_range(x_range.clone()) as f32;
        let y = rng.gen_range(y_range.clone()) as f32;

        commands.spawn().insert_bundle(BoidBundle::new(Vec2::new(x, y), &asset_server));
    }
}

pub fn update_boid_positions(mut to_update: Query<(&mut Transform, &mut Velocity), With<Boid>>) {
    let boid_count = to_update.iter().size_hint().1.unwrap();
    let velocity_changes: Vec<(Vec2, Vec2, Vec2)> = (0..boid_count).map(|id| evaluate_rule_vectors(id, &to_update)).collect();

    for (mut boid, velocity_change) in to_update.iter_mut().zip(velocity_changes) {
        boid.1.0 += (velocity_change.0 * SEPERATION_FACTOR + velocity_change.1 * ALIGNMNET_FACTOR + velocity_change.2 * COHESION_FACTOR) * BOID_TURN_SPEED;
        boid.1.0 += get_bounds_restoring_force(boid.0.translation.truncate());
        boid.1.0 = boid.1.0.clamp_length(1.0, MAX_BOID_SPEED); //prevent them from standing still or going too fast

        boid.0.translation += BOID_SPEED_MULTIPLIER * boid.1.0.extend(0.0);
        boid.0.rotation = get_boid_rotation(boid.1.0);
    }
}

fn evaluate_rule_vectors(boid_id: usize, boids: &Query<(&mut Transform, &mut Velocity), With<Boid>>) -> (Vec2, Vec2, Vec2) {
    let mut visible_boids : usize = 0;

    let mut seperation = Vec2::ZERO;
    let mut alignment = Vec2::ZERO;
    let mut cohesion = Vec2::ZERO;

    let boid_position = boids.iter().skip(boid_id).next().unwrap().0.translation.truncate();
    let boid_alignment = boids.iter().skip(boid_id).next().unwrap().1.0;
    
    for (neighbour_id, neighbour) in boids.iter().enumerate() {
        let neighbour_position = neighbour.0.translation.truncate();
        let distance = Vec2::distance(boid_position, neighbour_position);

        if neighbour_id != boid_id && distance < VIEW_RANGE {
            visible_boids += 1;
            
            if distance < MIN_DISTANCE { seperation += boid_position - neighbour_position; }
            alignment += neighbour.1.0;
            cohesion += neighbour_position;
        }
    }

    if visible_boids > 0 {
        alignment /= visible_boids as f32;
        cohesion /= visible_boids as f32;
    } else {
        //prevent them moving to (0, 0)
        alignment = boid_position;
        cohesion = boid_position;
    }

    (seperation, alignment - boid_alignment, cohesion - boid_position)
}

fn get_bounds_restoring_force(boid_position : Vec2) -> Vec2 {
    let mut restoring_force = Vec2::ZERO;

    if boid_position.x < -WINDOW_WIDTH / 2.0 {
        restoring_force.x += MAP_BOUNDS_FORCE;
    } else if boid_position.x > WINDOW_WIDTH / 2.0 {
        restoring_force.x -= MAP_BOUNDS_FORCE;
    }

    if boid_position.y < -WINDOW_HEIGHT / 2.0 {
        restoring_force.y += MAP_BOUNDS_FORCE;
    } else if boid_position.y > WINDOW_HEIGHT / 2.0 {
        restoring_force.y -= MAP_BOUNDS_FORCE;
    }

    restoring_force
}

fn get_boid_rotation(direction : Vec2) -> Quat {    
    Quat::from_rotation_z(Vec2::angle_between(Vec2::Y, direction))
}