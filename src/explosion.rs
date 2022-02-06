use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::common::*;

//
// Components
//

#[derive(Component)]
struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

//
// Systems
//

fn explosion_to_spawn(
    mut commands: Commands,
    audio: Res<Audio>,
    audio_materials: Res<AudioMaterials>,
    sprite_materials: Res<SpriteMaterials>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // Spwan an explosion
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprite_materials.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(Timer::from_seconds(0.05, true));
        // Play explosion sound.
        audio.play_in_channel(
            audio_materials.explosion_sound.clone(),
            &audio_materials.explosion_channel,
        );
        // Despawn explosion trigger.
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &mut Timer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Explosion>,
    >,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() {
                commands.entity(entity).despawn();
            }
        }
    }
}

//
// Plugin
//

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(explosion_to_spawn)
            .add_system(animate_explosion);
    }
}
