//#![allow(unused)]

mod canonball;
mod common;
mod player;

use bevy::prelude::*;
use canonball::CanonBallPlugin;
use common::{SpriteMaterials, WinSize};
use player::PlayerPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Position camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Manage window
    let window = windows.get_primary_mut().unwrap();
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
    // Position window (for dev only)
    // window.set_position(IVec2::new(1920, 0));

    // Read sprite sheet and create associated resource
    let mut texture_atlas =
        TextureAtlas::new_empty(asset_server.load("spritesheet.png"), Vec2::new(96., 32.));
    let boat_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0., 0.),
        max: Vec2::new(96., 16.),
    });
    let canon_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0., 16.),
        max: Vec2::new(16., 32.),
    });

    let canonball_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(16., 16.),
        max: Vec2::new(24., 24.),
    });
    let torpedo_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(24., 16.),
        max: Vec2::new(40., 24.),
    });
    let sprites_h = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteMaterials {
        texture: sprites_h,
        boat_index,
        canon_index,
        canonball_index,
        torpedo_index,
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::AQUAMARINE))
        .insert_resource(WindowDescriptor {
            title: "Naval".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(CanonBallPlugin)
        .run();
}
