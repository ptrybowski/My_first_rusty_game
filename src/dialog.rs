use bevy::prelude::*;
use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::fadeout::create_fadeout;
use crate::{GameState, TILE_SIZE};

#[derive(Component)]
pub struct NPC;

pub struct DialogPlugin;

impl Plugin for DialogPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Dialog).with_system(test_exit_dialog))
            .add_system_set(SystemSet::on_enter(GameState::Dialog).with_system(spawn_npc)
                .with_system(dialog_camera))
            .add_system_set(SystemSet::on_exit(GameState::Dialog).with_system(despawn_npc));

    }
}

fn dialog_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
){
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_npc(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
){
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(1.0,0.2,0.2),
        Vec3::new(0.0,5.0*TILE_SIZE,999.0)
    );
    commands.entity(sprite)
        .insert(NPC)
        .insert(Name::new("Npc"));
}

fn despawn_npc(
    mut commands: Commands,
    npc_query: Query<Entity, With<NPC>>
){
    for entity in npc_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_dialog(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    ascii: Res<AsciiSheet>,

){
    if keyboard.just_pressed(KeyCode::Escape)&&*state.current()==GameState::Dialog{
        println!("Changing to overworld");
        create_fadeout(&mut commands, GameState::Overworld, &ascii);
        keyboard.clear();
    }
}