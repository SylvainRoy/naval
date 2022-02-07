use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::Audio;

use lyon_geom::{point, LineSegment, Point};

use crate::canonball::{CanonBall, Energy};
use crate::common::*;
use crate::island::Ground;
use crate::torpedo::Torpedo;

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
const CANON_RELOAD: u64 = 2;

const TORPEDO_INIT_ANGLE: f32 = 0.;
const TORPEDO_SIGHT_DIST: f32 = 48.;
const TORPEDO_RELOAD: u64 = 5;

pub const AMUNITIONS: u32 = 50;
pub const TORPEDOS: u32 = 15;
pub const LIFE: u32 = 100;

//
// Misc functions
//

/// Check collision of a segment with a group of segments
fn check_collision(seg: &LineSegment<f32>, segs: &Vec<LineSegment<f32>>) -> bool {
    for tile_seg in segs {
        if seg.intersects(tile_seg) {
            return true;
        }
    }
    false
}

/// Utility to convert from Vec3 to Point.
fn vec_to_point(vec: &Vec3) -> Point<f32> {
    point(vec[0], vec[1])
}

//
// Components
//

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Speed(pub f32);

#[derive(Component)]
struct Canon;

#[derive(Component)]
struct CanonSight(f32);

#[derive(Component)]
struct TorpedoSight;

#[derive(Component)]
struct CollisionReady(bool);

#[derive(Component)]
pub struct Life(pub u32);

#[derive(Component)]
pub struct Amunitions(pub u32);

#[derive(Component)]
pub struct Torpedos(pub u32);

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
        .insert(Life(LIFE))
        .insert(CollisionReady(true))
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
                .insert(Timer::from_seconds(0.0, false))
                .insert(Amunitions(AMUNITIONS));
        })
        // Torpedo sight
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.torpedo_sight_index),
                    transform: Transform {
                        translation: Vec3::new(TORPEDO_SIGHT_DIST, 0., WEAPON_Z),
                        rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), TORPEDO_INIT_ANGLE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TorpedoSight)
                .insert(Timer::from_seconds(0.0, false))
                .insert(Torpedos(TORPEDOS));
        });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    audio_materials: Res<AudioMaterials>,
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
        // Start/stop engine sound
        if speed.0.abs() < 0.1 {
            audio.pause_channel(&audio_materials.engine_channel);
        } else {
            audio.resume_channel(&audio_materials.engine_channel);
        }
    };
}

fn canon_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CanonSight)>,
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
    for (mut transform, mut canon_sight) in query.iter_mut() {
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

fn torpedo_sight_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<TorpedoSight>>,
) {
    // Determine new parameters of the torpedo sight.
    let delta_angle = if keyboard_input.pressed(KeyCode::U) {
        CANON_ROTATION_SPEED * TIME_STEP
    } else if keyboard_input.pressed(KeyCode::O) {
        -CANON_ROTATION_SPEED * TIME_STEP
    } else {
        0.
    };
    // Update torpedo sight
    for mut transform in query.iter_mut() {
        transform.rotation = transform
            .rotation
            .mul_quat(Quat::from_axis_angle(Vec3::new(0., 0., 1.), delta_angle));
        transform.translation = transform
            .rotation
            .mul_vec3(Vec3::new(TORPEDO_SIGHT_DIST, 0., 0.));
    }
}

fn canon_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    time: Res<Time>,
    sprite_materials: Res<SpriteMaterials>,
    audio_materials: Res<AudioMaterials>,
    query_boat: Query<&GlobalTransform, With<Player>>,
    mut query_sight: Query<
        (&Parent, &GlobalTransform, &mut Amunitions, &mut Timer),
        With<CanonSight>,
    >,
) {
    let (parent, canon_sight_gtf, mut amunitions, mut timer) = query_sight.single_mut();
    // Increment timer measuring time to reload.
    timer.tick(time.delta());

    // If ready to fire, amunitions left and key pressed, trigger fire.
    if timer.finished() && amunitions.0 > 0 && kb.pressed(KeyCode::Space) {
        // Compute origin and energy of canonball.
        let boat_gtf = query_boat.get(parent.0).unwrap();
        let x_dest = canon_sight_gtf.translation.x;
        let y_dest = canon_sight_gtf.translation.y;
        let x_org = boat_gtf.translation.x;
        let y_org = boat_gtf.translation.y;
        let distance = Vec3::new(x_dest - x_org, y_dest - y_org, 0.).length();
        // Spawn the canonball.
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
        // Play canon sound.
        audio.play_in_channel(
            audio_materials.canon_sound.clone(),
            &audio_materials.weapon_channel,
        );
        // Decrease number of amunitions.
        amunitions.0 -= 1;
        // Player will have to wait for reload to fire again.
        timer.set_duration(Duration::from_secs(CANON_RELOAD));
        timer.reset();
    }
}

fn torpedo_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    time: Res<Time>,
    sprite_materials: Res<SpriteMaterials>,
    audio_materials: Res<AudioMaterials>,
    query_boat: Query<&GlobalTransform, With<Player>>,
    mut query_sight: Query<
        (&Parent, &GlobalTransform, &mut Torpedos, &mut Timer),
        With<TorpedoSight>,
    >,
) {
    let (parent, torpedo_sight_gtf, mut torpedos, mut timer) = query_sight.single_mut();
    // Increment timer measuring time to reload.
    timer.tick(time.delta());

    // If ready to fire, amunitions left and key pressed, trigger fire.
    if timer.finished() && torpedos.0 > 0 && kb.pressed(KeyCode::Return) {
        // Spawn the torpedos
        let boat_gtf = query_boat.get(parent.0).unwrap();
        for angle in [-0.1, 0., 0.1] {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.torpedo_index),
                    transform: Transform {
                        translation: Vec3::new(
                            boat_gtf.translation.x,
                            boat_gtf.translation.y,
                            TORPEDO_Z,
                        ),
                        rotation: torpedo_sight_gtf
                            .rotation
                            .mul_quat(Quat::from_rotation_z(angle)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Torpedo);
        }
        // Play torpedo sound.
        audio.play_in_channel(
            audio_materials.torpedo_sound.clone(),
            &audio_materials.weapon_channel,
        );
        // Decrease number of torpedos
        torpedos.0 -= 1;
        // Player will have to wait for reload to fire again.
        timer.set_duration(Duration::from_secs(TORPEDO_RELOAD));
        timer.reset();
    }
}

fn player_ground_collision(
    mut query_player: Query<
        (
            &mut Transform,
            &TextureAtlasSprite,
            &mut Speed,
            &mut Life,
            &mut CollisionReady,
        ),
        With<Player>,
    >,
    query_ground: Query<&Transform, (With<Ground>, Without<Player>)>,
) {
    let (mut player_tf, sprite, mut speed, mut life, mut collision_ready) =
        query_player.single_mut();

    // retrieve boat dimensions
    let boat_dimensions = sprite.custom_size.unwrap();
    let dx = boat_dimensions[0] / 2.;
    let dy = boat_dimensions[1] / 2.;
    let boat_max_dim = boat_dimensions[0].max(boat_dimensions[1]);

    // Compute relevants points of the boat skull
    let front_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(dx, dy, 0.));
    let front_right_pt =
        player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(dx, -dy, 0.));
    let middle_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(0., dy, 0.));
    let middle_right_pt =
        player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(0., -dy, 0.));
    let rear_left_pt = player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(-dx, dy, 0.));
    let rear_right_pt =
        player_tf.translation + player_tf.rotation.mul_vec3(Vec3::new(-dx, -dy, 0.));

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

        // Decrease player life (once for a given collision)
        // TODO: player should have X seconds before to loose life again.
        if front_collision
            || rear_collision
            || front_left_collision
            || front_right_collision
            || rear_left_collision
            || rear_right_collision
        {
            if collision_ready.0 {
                life.0 -= life.0.min(10);
                collision_ready.0 = false;
            }
        } else {
            collision_ready.0 = true;
        }
    }
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
            .add_system(canon_fire)
            .add_system(torpedo_fire)
            .add_system(player_ground_collision)
            .add_system(torpedo_sight_movement);
    }
}
