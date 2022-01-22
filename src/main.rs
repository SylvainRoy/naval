//#![allow(unused)]

mod canonball;
mod common;
mod island;
mod player;

use bevy::prelude::*;
use canonball::CanonBallPlugin;
use common::{SpriteMaterials, WinSize};
use island::IslandPlugin;
use player::PlayerPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

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
        max: Vec2::new(80., 16.),
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
    let bunker_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(80., 16.),
        max: Vec2::new(96., 32.),
    });
    let ground1_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0., 32.),
        max: Vec2::new(16., 48.),
    });
    let ground2_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(16., 32.),
        max: Vec2::new(32., 48.),
    });
    let ground3_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(32., 32.),
        max: Vec2::new(48., 48.),
    });
    let ground4_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(48., 32.),
        max: Vec2::new(64., 48.),
    });
    let ground5_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(64., 32.),
        max: Vec2::new(80., 48.),
    });
    let ground6_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(80., 32.),
        max: Vec2::new(96., 48.),
    });
    let mountain_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0., 48.),
        max: Vec2::new(16., 64.),
    });

    let sprites_h = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteMaterials {
        texture: sprites_h,
        boat_index,
        canon_index,
        canonball_index,
        torpedo_index,
        bunker_index,
        ground1_index,
        ground2_index,
        ground3_index,
        ground4_index,
        ground5_index,
        ground6_index,
        mountain_index,
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0.4118, 0.5804)))
        .insert_resource(WindowDescriptor {
            title: "Naval".to_string(),
            width: 1000.0,
            height: 700.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(CanonBallPlugin)
        .add_plugin(IslandPlugin)
        .run();
}
