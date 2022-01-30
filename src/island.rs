use crate::common::*;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::{hash_map::Entry::Vacant, HashMap};

const NUM_ISLANDS: u32 = 20;
const SIZE_ISLANDS: u32 = 40;

//
// Components
//

#[derive(Component)]
pub struct Ground;

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
    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();
    for _island in 0..rng.gen_range((NUM_ISLANDS / 2)..NUM_ISLANDS) {
        // Place a new island at random
        let x = rng.gen_range(-w_tiles..=w_tiles);
        let y = rng.gen_range(-h_tiles..=h_tiles);
        tiles.insert((x, y), true);
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
                if let Vacant(e) = tiles.entry((xx, yy)) {
                    e.insert(true);
                    break;
                };
            }
        }
    }

    // Spwan the ground tiles
    for (tile_x, tile_y) in tiles.keys() {
        // Determine sprite & orientation based on adjacent tiles
        let right = tiles.contains_key(&(*tile_x + 1, *tile_y));
        let up = tiles.contains_key(&(*tile_x, *tile_y + 1));
        let left = tiles.contains_key(&(*tile_x - 1, *tile_y));
        let down = tiles.contains_key(&(*tile_x, *tile_y - 1));
        let (index, rotation) = match (right, up, left, down) {
            // 0 adjacent
            (false, false, false, false) => (sprite_materials.ground6_index, 0.),
            // 1
            (true, false, false, false) => (sprite_materials.ground5_index, -2.),
            (false, true, false, false) => (sprite_materials.ground5_index, -1.),
            (false, false, true, false) => (sprite_materials.ground5_index, 0.),
            (false, false, false, true) => (sprite_materials.ground5_index, 1.),
            // 2
            (true, true, false, false) => (sprite_materials.ground3_index, -2.),
            (false, true, true, false) => (sprite_materials.ground3_index, -1.),
            (false, false, true, true) => (sprite_materials.ground3_index, 0.),
            (true, false, false, true) => (sprite_materials.ground3_index, 1.),

            (true, false, true, false) => (sprite_materials.ground4_index, 0.),
            (false, true, false, true) => (sprite_materials.ground4_index, 1.),
            // 3
            (true, true, true, false) => (sprite_materials.ground2_index, -1.),
            (true, true, false, true) => (sprite_materials.ground2_index, 2.),
            (true, false, true, true) => (sprite_materials.ground2_index, 1.),
            (false, true, true, true) => (sprite_materials.ground2_index, 0.),
            // 4
            (true, true, true, true) => (sprite_materials.ground1_index, 0.),
        };
        // Spawn the ground tile
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprite_materials.texture.clone(),
                sprite: TextureAtlasSprite::new(index),
                transform: Transform {
                    translation: Vec3::new(
                        16. * (*tile_x as f32),
                        16. * (*tile_y as f32),
                        GROUND_Z,
                    ),
                    rotation: Quat::from_axis_angle(
                        Vec3::new(0., 0., 1.),
                        rotation * std::f32::consts::PI / 2.,
                    ),
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
