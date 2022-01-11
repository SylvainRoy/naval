use crate::common::{WinSize, TIME_STEP};
use bevy::prelude::*;

const CANONBALL_SPEED: f32 = 150.;

//
// Components
//

#[derive(Component)]
pub struct CanonBall;

//
// Systems
//

fn canonball_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, With<CanonBall>)>,
) {
    for (canonball_entity, mut canonball_tf, _) in query.iter_mut() {
        let translation =
            canonball_tf
                .rotation
                .mul_vec3(Vec3::new(CANONBALL_SPEED * TIME_STEP, 0., 0.));
        canonball_tf.translation += translation;
        if (translation.y < -0.5 * win_size.h)
            || (0.5 * win_size.h < translation.y)
            || (translation.x < -0.5 * win_size.w)
            || (0.5 * win_size.w < translation.x)
        {
            commands.entity(canonball_entity).despawn();
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
