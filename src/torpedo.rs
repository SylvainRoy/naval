use std::collections::HashMap;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::common::WinSize;
use crate::explosion::ExplosionToSpawn;
use crate::island::Ground;

const TORPEDO_SPEED: f32 = 50.;

//
// Components
//

#[derive(Component)]
pub struct Torpedo;

//
// Systems
//

fn torpedo_movement(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform), With<Torpedo>>,
) {
    for (torpedo_entity, mut torpedo_tf) in query.iter_mut() {
        // Compute new torpedo position.
        let translation =
            torpedo_tf
                .rotation
                .mul_vec3(Vec3::new(TORPEDO_SPEED * time.delta_seconds(), 0., 0.));
        torpedo_tf.translation += translation;
        // Remove torpedo if off screen.
        if (torpedo_tf.translation.y < -0.5 * win_size.h)
            || (0.5 * win_size.h < torpedo_tf.translation.y)
            || (torpedo_tf.translation.x < -0.5 * win_size.w)
            || (0.5 * win_size.w < torpedo_tf.translation.x)
        {
            commands.entity(torpedo_entity).despawn();
        }
    }
}

fn torpedo_ground_collision(
    mut commands: Commands,
    mut query_torpedo: Query<(Entity, &Transform), With<Torpedo>>,
    query_moutain: Query<&Transform, With<Ground>>,
) {
    let mut despawned = HashMap::new();
    // for each torpedo & ground.
    for (torpedo_entity, torpedo_tf) in query_torpedo.iter_mut() {
        for ground_tf in query_moutain.iter() {
            // Check for collision.
            let collision = collide(
                torpedo_tf.translation,
                Vec2::splat(6.),
                ground_tf.translation,
                Vec2::splat(16.),
            );
            // If collision, replace torpedo by an explosion.
            if collision.is_some() {
                if !despawned.contains_key(&torpedo_entity) {
                    despawned.insert(torpedo_entity, true);
                    commands.entity(torpedo_entity).despawn();
                }
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(torpedo_tf.translation.clone()));
            }
        }
    }
}

//
// Plugin
//

pub struct TorpedoPlugin;

impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(torpedo_movement)
            .add_system(torpedo_ground_collision);
    }
}
