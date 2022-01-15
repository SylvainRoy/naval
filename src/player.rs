use bevy::prelude::*;

use crate::canonball::CanonBall;
use crate::common::*;

const BOAT_INIT_POSITION: (f32, f32) = (0., 0.);
const BOAT_INIT_ANGLE: f32 = 0.;

const BOAT_MAX_SPEED_FORWARD: f32 = 1.5;
const BOAT_MAX_SPEED_BACKWARD: f32 = -0.5;
const BOAT_ACCELERATION: f32 = 0.6;
const BOAT_FRICTION: f32 = 0.2;
const BOAT_ROTATION_SPEED: f32 = std::f32::consts::PI / 6.;

const CANON_INIT_ANGLE: f32 = 0.;
const CANON_POSITIONS: (f32, f32) = (16., -32.);
const CANON_ROTATION_SPEED: f32 = std::f32::consts::PI;

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
                scale: Vec3::splat(1.0),
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed::default())
        // Front canon
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.canon_index),
                    transform: Transform {
                        translation: Vec3::new(CANON_POSITIONS.0, 0., WEAPON_Z),
                        rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), CANON_INIT_ANGLE),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    ..Default::default()
                })
                .insert(Canon)
                .insert(CanonReadyFire(true));
        })
        // Rear canon
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.canon_index),
                    transform: Transform {
                        translation: Vec3::new(CANON_POSITIONS.1, 0., WEAPON_Z),
                        rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), CANON_INIT_ANGLE),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    ..Default::default()
                })
                .insert(Canon)
                .insert(CanonReadyFire(true));
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
    mut query: Query<(&mut Transform, With<Canon>)>,
) {
    // Determine new direction of the canon.
    let delta_angle = if keyboard_input.pressed(KeyCode::I) {
        CANON_ROTATION_SPEED * TIME_STEP
    } else if keyboard_input.pressed(KeyCode::P) {
        -CANON_ROTATION_SPEED * TIME_STEP
    } else {
        0.
    };
    for (mut transform, _) in query.iter_mut() {
        transform.rotation = transform
            .rotation
            .mul_quat(Quat::from_axis_angle(Vec3::new(0., 0., 1.), delta_angle));
    }
}

fn player_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    sprite_materials: Res<SpriteMaterials>,
    mut query: Query<(&GlobalTransform, &mut CanonReadyFire, With<Canon>)>,
) {
    for (canon_gtf, mut ready_fire, _) in query.iter_mut() {
        if ready_fire.0 && kb.pressed(KeyCode::Space) {
            let x = canon_gtf.translation.x;
            let y = canon_gtf.translation.y;
            let translation =
                Vec3::new(x, y, PROJECTILE_Z) + canon_gtf.rotation.mul_vec3(Vec3::new(8., 0., 0.));
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprite_materials.texture.clone(),
                    sprite: TextureAtlasSprite::new(sprite_materials.canonball_index),
                    transform: Transform {
                        translation,
                        rotation: canon_gtf.rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CanonBall);
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
