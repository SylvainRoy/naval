use crate::common::{WinSize, TIME_STEP};
use bevy::prelude::*;

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
            // No more energy: explode canonball
            commands.entity(canonball_entity).despawn();
            // TODO: Explosion here!
        }
    }
}

//
// Plugin
//

pub struct CanonBallPlugin;

impl Plugin for CanonBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(canonball_movement);
    }
}
