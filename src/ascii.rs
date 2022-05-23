use bevy::prelude::*;

use crate::{TILE_SIZE};
use crate::player::Player;

pub struct AsciiPlugin;

pub struct AsciiSheet(pub(crate) Handle<TextureAtlas>);

impl Plugin for AsciiPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);

    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3
)-> Entity {

    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.index = index;

    commands.spawn_bundle(SpriteSheetBundle{
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
            },
        ..Default::default()
    }).id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>){
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0)
    );
    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
