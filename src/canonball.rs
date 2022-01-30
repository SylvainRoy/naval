use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::common::{WinSize, TIME_STEP};
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
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, &mut Energy, With<CanonBall>)>,
) {
    for (canonball_entity, mut canonball_tf, mut energy, _) in query.iter_mut() {
        if energy.0 != 0. {
            // Move canonball according to energy left.
            let distance = energy.0.min(CANONBALL_SPEED * TIME_STEP);
            energy.0 -= distance;
            let translation = canonball_tf.rotation.mul_vec3(Vec3::new(distance, 0., 0.));
            canonball_tf.translation += translation;
            // Remove canonball if off screen.
            if (translation.y < -0.5 * win_size.h)
                || (0.5 * win_size.h < translation.y)
                || (translation.x < -0.5 * win_size.w)
                || (0.5 * win_size.w < translation.x)
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
    // for each canonball & mountain.
    for (canonball_entity, canonball_tf) in query_canonball.iter_mut() {
        for mountain_tf in query_moutain.iter() {
            // Replace canonball by an explosion in case of collision.
            let collision = collide(
                canonball_tf.translation,
                Vec2::splat(6.),
                mountain_tf.translation,
                Vec2::splat(16.),
            );
            if collision.is_some() {
                commands.entity(canonball_entity).despawn();
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
