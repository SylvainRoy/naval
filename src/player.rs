use bevy::{prelude::*, sprite::collide_aabb::collide};

use lyon_geom::{LineSegment, Point, point};

use crate::canonball::{CanonBall, Energy};
use crate::island::Ground;
use crate::common::*;

const BOAT_INIT_POSITION: (f32, f32) = (0., 0.);
const BOAT_INIT_ANGLE: f32 = 0.;

const BOAT_MAX_SPEED_FORWARD: f32 = 1.5;
const BOAT_MAX_SPEED_BACKWARD: f32 = -0.5;
const BOAT_ACCELERATION: f32 = 0.5;
const BOAT_FRICTION: f32 = 0.2;
const BOAT_ROTATION_SPEED: f32 = std::f32::consts::PI / 6.;

const CANON_INIT_ANGLE: f32 = 0.;
const CANON_MIN_DISTANCE: f32 = 60.;
const CANON_MAX_DISTANCE: f32 = 500.;
const CANON_ROTATION_SPEED: f32 = std::f32::consts::PI / 2.;
const CANON_DISTANCE_SPEED: f32 = 100.;

const TORPEDO_INIT_ANGLE: f32 = 0.;

//
// Components
//

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component)]
struct Canon;

#[derive(Component)]
struct CanonReadyFire(bool);

#[derive(Component)]
struct CanonSight(f32);

#[derive(Component)]
struct TorpedoSight;

//
// Systems
//

fn player_spawn(
    mut commands: Commands,
    sprite_materials: Res<SpriteMaterials>,
    _win_size: Res<WinSize>,
) {
    // Spwan the boat, canon sight and torpedo sight.
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprite_materials.texture.clone(),
            sprite: TextureAtlasSprite {
                index: sprite_materials.boat_index,
                custom_size: Some(Vec2::new(40., 8.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(BOAT_INIT_POSITION.0, BOAT_INIT_POSITION.1, BOAT_Z),
                rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), BOAT_INIT_ANGLE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed::default())
        // Canon sight
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.canon_sight_index),
                    transform: Transform {
                        translation: Vec3::new(CANON_MIN_DISTANCE, 0., WEAPON_Z),
                        rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), CANON_INIT_ANGLE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CanonSight(CANON_MIN_DISTANCE))
                .insert(CanonReadyFire(true));
        })
        // Torpedo sight
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.torpedo_sight_index),
                    transform: Transform {
                        translation: Vec3::new(48., 0., WEAPON_Z),
                        rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), TORPEDO_INIT_ANGLE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TorpedoSight);
        });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((mut speed, mut transform, _)) = query.get_single_mut() {
        //Determine new direction of the boat.
        let delta_angle = if keyboard_input.pressed(KeyCode::A) {
            BOAT_ROTATION_SPEED * TIME_STEP
        } else if keyboard_input.pressed(KeyCode::D) {
            -BOAT_ROTATION_SPEED * TIME_STEP
        } else {
            0.
        };
        transform.rotate(Quat::from_rotation_z(delta_angle));
        // Determine new position of the boat.
        speed.0 = if keyboard_input.pressed(KeyCode::W) {
            (speed.0 + BOAT_ACCELERATION * TIME_STEP).min(BOAT_MAX_SPEED_FORWARD)
        } else if keyboard_input.pressed(KeyCode::S) {
            (speed.0 - BOAT_ACCELERATION * TIME_STEP).max(BOAT_MAX_SPEED_BACKWARD)
        } else {
            speed.0 - BOAT_FRICTION * speed.0.abs().copysign(speed.0) * TIME_STEP
        };
        let translation = transform.rotation.mul_vec3(Vec3::new(speed.0, 0., 0.));
        transform.translation += translation;
    };
}

fn canon_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query2: Query<(&mut Transform, &mut CanonSight)>,
) {
    // Determine new parameters of the canon.
    let delta_angle = if keyboard_input.pressed(KeyCode::J) {
        CANON_ROTATION_SPEED * TIME_STEP
    } else if keyboard_input.pressed(KeyCode::L) {
        -CANON_ROTATION_SPEED * TIME_STEP
    } else {
        0.
    };
    let delta_distance = if keyboard_input.pressed(KeyCode::I) {
        CANON_DISTANCE_SPEED * TIME_STEP
    } else if keyboard_input.pressed(KeyCode::K) {
        -CANON_DISTANCE_SPEED * TIME_STEP
    } else {
        0.
    };
    // Update canon sight
    for (mut transform, mut canon_sight) in query2.iter_mut() {
        canon_sight.0 =
            CANON_MAX_DISTANCE.min(CANON_MIN_DISTANCE.max(canon_sight.0 + delta_distance));
        transform.rotation = transform
            .rotation
            .mul_quat(Quat::from_axis_angle(Vec3::new(0., 0., 1.), delta_angle));
        transform.translation = transform
            .rotation
            .mul_vec3(Vec3::new(canon_sight.0, 0., 0.));
    }
}

fn player_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    sprite_materials: Res<SpriteMaterials>,
    query_boat: Query<&GlobalTransform>,
    mut query_sight: Query<(
        &Parent,
        &GlobalTransform,
        &mut CanonReadyFire,
        With<CanonSight>,
    )>,
) {
    for (parent, canon_sight_gtf, mut ready_fire, _) in query_sight.iter_mut() {
        if ready_fire.0 && kb.pressed(KeyCode::Space) {
            let boat_gtf = query_boat.get(parent.0).unwrap();
            // Compute origin and energy of canonball.
            let x_dest = canon_sight_gtf.translation.x;
            let y_dest = canon_sight_gtf.translation.y;
            let x_org = boat_gtf.translation.x;
            let y_org = boat_gtf.translation.y;
            let distance = Vec3::new(x_dest - x_org, y_dest - y_org, 0.).length();
            // Spawn the canonball
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.canonball_index),
                    transform: Transform {
                        translation: Vec3::new(x_org, y_org, PROJECTILE_Z),
                        rotation: canon_sight_gtf.rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CanonBall)
                .insert(Energy(distance));
            ready_fire.0 = false;
        }
        // No automatic fire: the key must be released before next shot.
        if kb.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn player_ground_collision(
    mut query_player: Query<(&mut Transform, &TextureAtlasSprite, &mut Speed), With<Player>>,
    query_ground: Query<&Transform, (With<Ground>, Without<Player>)>,
) {
    // For each boat
    for (mut player_tf, sprite, mut speed) in query_player.iter_mut() {

        // retrieve boat dimensions
        let boat_dimensions = sprite.custom_size.unwrap();
        let dx = boat_dimensions[0] / 2.;
        let dy = boat_dimensions[1] / 2.;
        let boat_max_dim = boat_dimensions[0].max(boat_dimensions[1]);

        // Compute relevants points of the boat skull
        let front_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(dx, dy, 0.));
        let front_right_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(dx, -dy, 0.));
        let middle_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(0., dy, 0.));
        let middle_right_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(0., -dy, 0.));
        let rear_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(-dx, dy, 0.));
        let rear_right_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(-dx, -dy, 0.));
        // Compute relevants segments of the boat skull
        let front = LineSegment {
            from: point(front_left_pt[0], front_left_pt[1]),
            to: point(front_right_pt[0], front_right_pt[1]),
        };
        let rear = LineSegment {
            from: point(front_left_pt[0], front_left_pt[1]),
            to: point(front_right_pt[0], front_right_pt[1]),
        };
        let front_left = LineSegment {
            from: point(middle_left_pt[0], middle_left_pt[1]),
            to: point(front_left_pt[0], front_left_pt[1]),
        };
        let front_right = LineSegment {
            from: point(middle_right_pt[0], middle_right_pt[1]),
            to: point(front_right_pt[0], front_right_pt[1]),
        };
        let rear_left = LineSegment {
            from: point(middle_left_pt[0], middle_left_pt[1]),
            to: point(rear_left_pt[0], rear_left_pt[1]),
        };
        let rear_right = LineSegment {
            from: point(middle_right_pt[0], middle_right_pt[1]),
            to: point(rear_right_pt[0], rear_right_pt[1]),
        };

        // For each ground tile, check for collision
        for ground_tf in query_ground.iter() {

            // Quickly filter out obvious non-overlap
            let collision = collide(
                player_tf.translation,
                Vec2::splat(boat_max_dim),
                ground_tf.translation,
                Vec2::new(16., 16.),
            );
            if collision.is_none() {
                continue;
            };

            // Compute segments of the ground tile
            let mut tile_segments = Vec::new();
            tile_segments.push(LineSegment {
                from: vec_to_point(&(ground_tf.translation + Vec3::new(-8., 8., 0.))),
                to: vec_to_point(&(ground_tf.translation + Vec3::new(8., 8., 0.))),
            });
            tile_segments.push(LineSegment {
                from: vec_to_point(&(ground_tf.translation + Vec3::new(-8., -8., 0.))),
                to: vec_to_point(&(ground_tf.translation + Vec3::new(8., -8., 0.))),
            });
            tile_segments.push(LineSegment {
                from: vec_to_point(&(ground_tf.translation + Vec3::new(-8., -8., 0.))),
                to: vec_to_point(&(ground_tf.translation + Vec3::new(-8., 8., 0.))),
            });
            tile_segments.push(LineSegment {
                from: vec_to_point(&(ground_tf.translation + Vec3::new(8., -8., 0.))),
                to: vec_to_point(&(ground_tf.translation + Vec3::new(8., 8., 0.))),
            });

            // Conpute collisions of the boat & tile
            let front_collision = check_collision(&front, &tile_segments);
            let rear_collision = check_collision(&rear, &tile_segments);
            let front_left_collision = check_collision(&front_left, &tile_segments);
            let front_right_collision = check_collision(&front_right, &tile_segments);
            let rear_left_collision = check_collision(&rear_left, &tile_segments);
            let rear_right_collision = check_collision(&rear_right, &tile_segments);

            // Change boat's speed & rotation accordingly
            if front_collision || front_left_collision || front_right_collision {
                speed.0 = speed.0.min(0.);
            }
            if rear_collision || rear_left_collision || rear_right_collision {
                speed.0 = speed.0.max(0.);
            }
            let mut delta_rotate = 0.;
            if front_left_collision || rear_right_collision {
                delta_rotate -= BOAT_ROTATION_SPEED * TIME_STEP;
            }
            if front_right_collision || rear_left_collision {
                delta_rotate += BOAT_ROTATION_SPEED * TIME_STEP;
            }
            player_tf.rotate(Quat::from_rotation_z(delta_rotate));
        }
    }
}

/// Check collision of a segment with a group of segments
fn check_collision(seg: &LineSegment<f32>, segs: &Vec<LineSegment<f32>>) -> bool {
    for tile_seg in segs {
        if seg.intersects(tile_seg) {
            return true;
        }
    }
    false
}

/// Utility fun to convert from Vec3 to Point.
fn vec_to_point(vec: &Vec3) -> Point<f32> {
    point(vec[0], vec[1])
}

//
// Plugin
//

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_actors", SystemStage::single(player_spawn))
            .add_system(player_movement)
            .add_system(canon_movement)
            .add_system(player_fire)
            .add_system(player_ground_collision);
    }
}
