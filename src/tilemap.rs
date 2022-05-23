use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;
use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, GameState, MAP_X_SIZE, MAP_Y_SIZE, TILE_SIZE};

pub struct Lawn{
    pub is_grass: Vec<Vec<bool>>,
}

impl FromWorld for Lawn {
    fn from_world(world: &mut World) -> Self {
        Lawn {
            is_grass: Vec::new()

        }
    }
}

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct EncounterSpawner;


fn get_y_size(mut data: &mut BufReader<File>)->usize{
    data.lines().count()
}

fn get_x_size(mut data: &mut BufReader<File>)->usize{
    let mut line = String::new();
    data.read_line(&mut line).unwrap()

}


impl Plugin for TileMapPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(create_simple_map)
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld)
                    .with_system(show_map)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld)
                    .with_system(hide_map)
            );
    }
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>
){
    if let Ok(children) = children_query.get_single(){
        for child in children.iter(){
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child){
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>
){
    if let Ok(children) = children_query.get_single(){
        for child in children.iter(){
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child){
                child_vis.is_visible = true;
            }
        }
    }
}


fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>, mut lawn: ResMut<Lawn>){
    let file = File::open("assets/map.txt").expect("No map file");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate(){
        let mut map_row = Vec::new();
        if let Ok(line) = line {
            for (x,char) in line.chars().enumerate(){
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9,0.9,0.9),
                    Vec3::new(x as f32*TILE_SIZE-(10.0*TILE_SIZE),-(y as f32)*TILE_SIZE+(2.0*TILE_SIZE),100.0)
                );
                if char == '#'{
                    commands.entity(tile).insert(TileCollider);
                }
                if char == 'B'{
                    commands.entity(tile).insert(EncounterSpawner);
                }
                if char as usize == 247{
                    map_row.push(true);
                }
                if char as usize != 247{
                    map_row.push(false);
                }
                tiles.push(tile);
            }
        }
        lawn.is_grass.push(map_row);
    }

    commands.spawn()
        .insert(Map)
        .insert(Name::new("map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);


}