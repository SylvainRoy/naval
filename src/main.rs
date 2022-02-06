//#![allow(unused)]

mod canonball;
mod common;
mod dashboard;
mod explosion;
mod island;
mod player;
mod torpedo;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use canonball::CanonBallPlugin;
use common::{AudioMaterials, SpriteMaterials, WinSize};
use dashboard::DashboardPlugin;
use explosion::ExplosionPlugin;
use island::IslandPlugin;
use player::PlayerPlugin;
use torpedo::TorpedoPlugin;

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Position cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

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
        min: Vec2::new(16., 48.),
        max: Vec2::new(56., 56.),
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
    let canon_sight_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0., 64.),
        max: Vec2::new(32., 96.),
    });
    let torpedo_sight_index = texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(32., 64.),
        max: Vec2::new(64., 96.),
    });
    // Read explosion spritesheet
    let texture_handle_explosion = asset_server.load("explosion.png");
    let texture_atlas_explosion =
        TextureAtlas::from_grid(texture_handle_explosion, Vec2::new(64.0, 64.0), 4, 4);

    commands.insert_resource(SpriteMaterials {
        texture: texture_atlases.add(texture_atlas),
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
        canon_sight_index,
        torpedo_sight_index,
        explosion: texture_atlases.add(texture_atlas_explosion),
    });

    // Ream audio files and create associated resources.
    let audio_materials = AudioMaterials {
        canon_sound: asset_server.load("GunShotGverb.ogg"),
        explosion_sound: asset_server.load("ExplosionMetalGverb.ogg"),
        torpedo_sound: asset_server.load("SplashGverb.ogg"),
        engine_sound: asset_server.load("BattleShipMovementAmbient.ogg"),
        weapon_channel: AudioChannel::new("weapon".to_string()),
        explosion_channel: AudioChannel::new("explosion".to_string()),
        engine_channel: AudioChannel::new("engine".to_string()),
    };
    // Prepare engine audio channel.
    commands.insert_resource(audio_materials.clone());
    audio.set_volume_in_channel(0.5, &audio_materials.engine_channel);
    audio.play_looped_in_channel(
        audio_materials.engine_sound.clone(),
        &audio_materials.engine_channel,
    );
    audio.pause_channel(&audio_materials.engine_channel);
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
        .add_plugin(AudioPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(CanonBallPlugin)
        .add_plugin(TorpedoPlugin)
        .add_plugin(IslandPlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(DashboardPlugin)
        .run();
}
