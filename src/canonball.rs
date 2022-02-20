use std::collections::HashMap;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::common::WinSize;
use crate::explosion::ExplosionToSpawn;
use crate::island::Mountain;

const CANONBALL_SPEED: f32 = 150.;

//
// Components
//

#[derive(Component)]
pub struct CanonBall;

#[derive(Component)]
pub struct Energy(pub f32);

//
// Systems
//

fn canonball_movement(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, &mut Energy, With<CanonBall>)>,
) {
    for (canonball_entity, mut canonball_tf, mut energy, _) in query.iter_mut() {
        if energy.0 != 0. {
            // Move canonball according to energy left.
            let distance = energy.0.min(CANONBALL_SPEED * time.delta_seconds());
            energy.0 -= distance;
            let translation = canonball_tf.rotation.mul_vec3(Vec3::new(distance, 0., 0.));
            canonball_tf.translation += translation;
            // Remove canonball if off screen.
            if (canonball_tf.translation.y < -0.5 * win_size.h)
                || (0.5 * win_size.h < canonball_tf.translation.y)
                || (canonball_tf.translation.x < -0.5 * win_size.w)
                || (0.5 * win_size.w < canonball_tf.translation.x)
            {
                commands.entity(canonball_entity).despawn();
            }
        } else {
            // No more energy: replace canonball by an explosion
            commands.entity(canonball_entity).despawn();
            commands
                .spawn()
                .insert(ExplosionToSpawn(canonball_tf.translation.clone()));
        }
    }
}

fn canonball_mountain_collision(
    mut commands: Commands,
    mut query_canonball: Query<(Entity, &Transform), With<CanonBall>>,
    query_moutain: Query<&Transform, With<Mountain>>,
) {
    let mut despawned = HashMap::new();
    // for each canonball & mountain.
    for (canonball_entity, canonball_tf) in query_canonball.iter_mut() {
        for mountain_tf in query_moutain.iter() {
            // Check for collision.
            let collision = collide(
                canonball_tf.translation,
                Vec2::splat(6.),
                mountain_tf.translation,
                Vec2::splat(16.),
            );
            // If collision, replace canonball by an explosion.
            if collision.is_some() {
                if !despawned.contains_key(&canonball_entity) {
                    despawned.insert(canonball_entity, true);
                    commands.entity(canonball_entity).despawn();
                }
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(canonball_tf.translation.clone()));
            }
        }
    }
}

//
// Plugin
//

pub struct CanonBallPlugin;

impl Plugin for CanonBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(canonball_movement)
            .add_system(canonball_mountain_collision);
    }
}
