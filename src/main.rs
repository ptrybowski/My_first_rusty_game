#![allow(clippy::redundant_field_names)]

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::core::FixedTimestep;

mod player;
mod debug;
mod ascii;
mod tilemap;
mod grass;
mod dialog;
mod fadeout;

use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;
use grass::GrassPlugin;
use dialog::DialogPlugin;
use fadeout::FadeoutPlugin;
use crate::tilemap::Lawn;

pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);
pub const RESOLUTION: f32 = 16.0/9.0;
pub const TILE_SIZE: f32 = 0.1;
pub const TIMESTEP_EVERY_5_SECONDS: f64 = 300.0/60.0;
pub const MAP_Y_SIZE: usize = 8;
pub const MAP_X_SIZE: usize = 228;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState{
    Overworld,
    Dialog,
}

fn main() {
    let height = 1080.0;
    App::new()
        .add_state(GameState::Overworld)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "game!".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(GrassPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DialogPlugin)
        .add_plugin(DebugPlugin)
        .init_resource::<Lawn>()
        .run();
    
}



fn spawn_camera (mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);

}
