use crate::common::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::{hash_map::Entry::Vacant, HashMap};

const NUM_ISLANDS:u32 = 20;
const SIZE_ISLANDS:u32 = 40;

//
// Components
//

#[derive(Component)]
struct Ground;

//
// Systems
//

fn islands_spawn(
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_materials: Res<SpriteMaterials>,
) {
    let mut rng = thread_rng();
    let w_tiles = (win_size.w / (2. * 16.)) as i32;
    let h_tiles = (win_size.h / (2. * 16.)) as i32;

    // Create N islands
    let mut drawn_tiles: HashMap<(i32, i32), bool> = HashMap::new();

    for _island in 0..rng.gen_range((NUM_ISLANDS / 2)..NUM_ISLANDS) {
        // Place a new island at random
        let x = rng.gen_range(-w_tiles..=w_tiles);
        let y = rng.gen_range(-h_tiles..=h_tiles);
        drawn_tiles.insert((x, y), true);
        // Grow the island
        for _tile in 0..rng.gen_range(0..SIZE_ISLANDS) {
            let (mut xx, mut yy) = (x, y);
            loop {
                match rng.gen_range(0..4) {
                    0 => xx += 1,
                    1 => xx -= 1,
                    2 => yy += 1,
                    _ => yy -= 1,
                };
                // Not within screen anymore
                if xx.abs() > w_tiles || yy.abs() > h_tiles {
                    break;
                };
                // If slot is empty, add a till
                if let Vacant(e) = drawn_tiles.entry((xx, yy)) {
                    e.insert(true);
                    break;
                };
            }
        }
    }

    // Spwan the ground tiles
    for (tile_x, tile_y) in drawn_tiles.keys() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprite_materials.texture.clone(),
                sprite: TextureAtlasSprite::new(sprite_materials.ground6_index),
                transform: Transform {
                    translation: Vec3::new(
                        16. * (*tile_x as f32),
                        16. * (*tile_y as f32),
                        GROUND_Z,
                    ),
                    // rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), BOAT_INIT_ANGLE),
                    // scale: Vec3::splat(1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Ground);
    }
}

//
// Plugin
//

pub struct IslandPlugin;

impl Plugin for IslandPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("game_setup_scene", SystemStage::single(islands_spawn));
    }
}
