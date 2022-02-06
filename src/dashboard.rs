use crate::player::{Amunitions, Life, Speed, Player, Torpedos, AMUNITIONS, LIFE, TORPEDOS};
use bevy::prelude::*;

//
// Misc functions
//

fn dashboard_string(life: u32, speed: f32, amunitions: u32, torpedos: u32) -> String {
    String::from(format!(
        "Life: {}\nSpeed: {}\nAmunitions: {}\nTorpedos: {}",
        life, speed, amunitions, torpedos
    ))
}

//
// Components
//

#[derive(Component)]
struct Dashboard;

//
// Systems
//

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the dasboard.
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                dashboard_string(LIFE, 0., AMUNITIONS, TORPEDOS),
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::BLACK,
                },
                Default::default()
            ),
            ..Default::default()
        })
        .insert(Dashboard);
}

fn update_dashboard(
    mut query_dashboard: Query<&mut Text, With<Dashboard>>,
    query_player: Query<(&Life, &Speed), With<Player>>,
    query_canon: Query<&Amunitions>,
    query_torpedo: Query<&Torpedos>,
) {
    let mut text = query_dashboard.single_mut();
    let (life, speed) = query_player.single();
    let amunitions = query_canon.single();
    let torpedos = query_torpedo.single();
    text.sections[0].value = dashboard_string(life.0, speed.0, amunitions.0, torpedos.0);
}

//
// Plugin
//

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update_dashboard);
    }
}
