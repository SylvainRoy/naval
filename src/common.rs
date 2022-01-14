use bevy::prelude::*;

pub const TIME_STEP: f32 = 1. / 60.;

pub const PROJECTILE_Z: f32 = 6.;
pub const WEAPON_Z: f32 = 5.;
pub const BOAT_Z: f32 = 4.;
// pub const MOUNTAIN_Z: f32 = 3.;
// pub const GROUND_Z: f32 = 2.;
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
}