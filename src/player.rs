use bevy::prelude::*;

use crate::canonball::{CanonBall, Energy};
use crate::common::*;

const BOAT_INIT_POSITION: (f32, f32) = (0., 0.);
const BOAT_INIT_ANGLE: f32 = 0.;
const BOAT_SCALE: f32 = 0.5;

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
    // Spwan the boat with both canons
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprite_materials.texture.clone(),
            sprite: TextureAtlasSprite::new(sprite_materials.boat_index),
            transform: Transform {
                translation: Vec3::new(BOAT_INIT_POSITION.0, BOAT_INIT_POSITION.1, BOAT_Z),
                rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), BOAT_INIT_ANGLE),
                scale: Vec3::splat(BOAT_SCALE),
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
                        translation: Vec3::new(CANON_MIN_DISTANCE, 0., PROJECTILE_Z),
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
                        translation: Vec3::new(48., 0., PROJECTILE_Z),
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
            let x_dest = canon_sight_gtf.translation.x;
            let y_dest = canon_sight_gtf.translation.y;
            let x_org = boat_gtf.translation.x;
            let y_org = boat_gtf.translation.y;
            let distance = Vec3::new(x_dest - x_org, y_dest - y_org, 0.).length();
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
        if kb.just_released(KeyCode::Space) {
            ready_fire.0 = true;
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
            .add_system(player_fire);
    }
}
