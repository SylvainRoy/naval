use bevy::prelude::*;

pub const TIME_STEP: f32 = 1. / 30.;

// Layers to position the sprites
pub const WEAPON_Z: f32 = 6.;
pub const PROJECTILE_Z: f32 = 5.;
pub const BOAT_Z: f32 = 4.;
pub const MOUNTAIN_Z: f32 = 3.;
pub const GROUND_Z: f32 = 2.;
// pub const TORPEDO_Z: f32 = 1.;
// pub const WATER_Z: f32 = 0.;

//
// Resources
//

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct SpriteMaterials {
    pub texture: Handle<TextureAtlas>,
    pub boat_index: usize,
    pub canon_index: usize,
    pub canonball_index: usize,
    pub torpedo_index: usize,
    pub bunker_index: usize,
    pub ground1_index: usize,
    pub ground2_index: usize,
    pub ground3_index: usize,
    pub ground4_index: usize,
    pub ground5_index: usize,
    pub ground6_index: usize,
    pub mountain_index: usize,
    pub canon_sight_index: usize,
    pub torpedo_sight_index: usize,
    pub explosion: Handle<TextureAtlas>,
}
