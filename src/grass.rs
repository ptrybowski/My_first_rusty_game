use bevy::prelude::*;
use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::{GameState, TILE_SIZE, TIMESTEP_EVERY_5_SECONDS};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin,Inspectable};
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;
use crate::player::Player;
use crate::tilemap::{TileCollider, Map, Lawn};


#[derive(Component, Inspectable)]
pub struct Grass{
    position: Vec3
}


pub struct GrassPlugin;

impl Plugin for GrassPlugin{
    fn build(&self, app: &mut App, ){
        app
            .add_startup_system(spawn_grass)
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld)
                    .with_system(show_grass)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld)
                    .with_system(hide_grass)
            )
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
                .with_system(grass_spread_left))

            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
                .with_system(grass_update))
            // .add_stage_after(CoreStage::Update, "grow1",SystemStage::single_threaded())
            // .add_stage_after("grow1", "grow2",SystemStage::single_threaded())
            // .add_stage_after("grow2", "grow3",SystemStage::single_threaded())
            // .add_stage_after("grow3", "grow4",SystemStage::single_threaded())
            //
            // .add_system_set_to_stage("grow1",SystemSet::new()
            //     .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
            //     .with_system(grass_spread_left.label("grow_left"))
            // )
            // .add_system_set_to_stage("grow2",SystemSet::new()
            //     .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
            //     .with_system(grass_spread_right.label("grow_right"))
            // )
            // .add_system_set_to_stage("grow3",SystemSet::new()
            //     .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
            //     .with_system(grass_spread_up.label("grow_up"))
            // )
            // .add_system_set_to_stage("grow4",SystemSet::new()
            //     .with_run_criteria(FixedTimestep::step(TIMESTEP_EVERY_5_SECONDS))
            //     .with_system(grass_spread_down.label("grow_down"))
            // )


        ;}
}


fn hide_grass(
    mut grass_query: Query<&mut Visibility, With<Grass>>,
    children_query: Query<&Children, With<Grass>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Grass>>
){
    for mut grass_vis in grass_query.iter_mut(){
        grass_vis.is_visible = false;
    }
    if let Ok(children) = children_query.get_single(){
        for child in children.iter(){
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child){
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_grass(
    mut grass_query: Query<&mut Visibility, With<Grass>>,
    children_query: Query<&Children, With<Grass>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Grass>>
){
    for mut grass_vis in grass_query.iter_mut() {
        grass_vis.is_visible = true;
    }
    if let Ok(children) = children_query.get_single(){
        for child in children.iter(){
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child){
                child_vis.is_visible = true;
            }
        }
    }
}


fn spawn_grass(mut commands:Commands, ascii:Res<AsciiSheet>){
    let mut translation = Vec3::new(0.0,0.0,500.0);
    let grass = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        247,
        Color::rgb(0.0,1.0,0.0),
        translation,
    );

    commands
        .entity(grass)
        .insert(Name::new("Grass"))
        .insert(Grass{position: translation})
        .insert(TileCollider)
        .id();
}

fn grass_update(
    mut lawn: ResMut<Lawn>,
    mut commands: Commands,
    ascii:Res<AsciiSheet>,


){
    for (x,row) in lawn.is_grass.iter().enumerate() {
        for (y, elem) in row.iter().enumerate(){
            if *elem{
                let mut translation = Vec3::new(-(x as f32)  * TILE_SIZE, y as f32 *TILE_SIZE, 0.0);
                let grass = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                247,
                Color::rgb(0.0, 1.0, 0.0),
                translation,
            );
            commands
                .entity(grass)
                .insert(Name::new("Grass"))
                .insert(Grass { position: translation })
                .insert(TileCollider)
                .id();
            }
        }
    }
}

fn grass_spread_left(
    mut query: Query<&Grass>,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
    mut lawn: ResMut<Lawn>
) {
    for grass in query.iter() {
        let target_xr = grass.position + Vec3::new(-1.05 * TILE_SIZE, 0.0, 0.0);

        if wall_grow_check(target_xr, &wall_query) & double_grow_check(target_xr, &query) {
            let mut translation = grass.position + Vec3::new(-1.0 * TILE_SIZE, 0.0, 0.0);
            lawn.is_grass[translation.x as usize ][translation.y as usize] = true
                }
        }
}
fn grass_spread_right(
    mut query: Query<&Grass>,
    mut commands:Commands,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
    ascii:Res<AsciiSheet>
) {
    for grass in query.iter() {
        let target_xl = grass.position + Vec3::new(1.05 * TILE_SIZE, 0.0, 0.0);
        if wall_grow_check(target_xl, &wall_query) & double_grow_check(target_xl, &query) {
            let mut translation = grass.position + Vec3::new(1.0 * TILE_SIZE, 0.0, 0.0);
            let grass = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                247,
                Color::rgb(0.0, 1.0, 0.0),
                translation,
            );
            commands
                .entity(grass)
                .insert(Name::new("Grass"))
                .insert(Grass { position: translation })
                .insert(TileCollider)
                .id();
        }
    }
}
fn grass_spread_up(
    mut query: Query<&Grass>,
    mut commands:Commands,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
    ascii:Res<AsciiSheet>
) {
    for grass in query.iter() {
        let target_yu = grass.position + Vec3::new(0.0, 1.05 * TILE_SIZE, 0.0);
        if wall_grow_check(target_yu, &wall_query) & double_grow_check(target_yu, &query) {
            let mut translation = grass.position + Vec3::new(0.0, 1.0 * TILE_SIZE, 0.0);
            let grass = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                247,
                Color::rgb(0.0, 1.0, 0.0),
                translation,
            );
            commands
                .entity(grass)
                .insert(Name::new("Grass"))
                .insert(Grass { position: translation })
                .insert(TileCollider)
                .id();
        }
    }
}
fn grass_spread_down(
    mut query: Query<&Grass>,
    mut commands:Commands,
    wall_query: Query<&Transform, (Without<Player>, With<TileCollider>)>,
    ascii:Res<AsciiSheet>
) {
    for grass in query.iter() {
        let target_yd = grass.position + Vec3::new(0.0, -1.05 * TILE_SIZE, 0.0);
        if wall_grow_check(target_yd, &wall_query) & double_grow_check(target_yd,&query){

            let mut translation = grass.position + Vec3::new(0.0,-1.0*TILE_SIZE,0.0);
            let grass = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                247,
                Color::rgb(0.0,1.0,0.0),
                translation,
            );
        commands
            .entity(grass)
            .insert(Name::new("Grass"))
            .insert(Grass{position: translation})
            .insert(TileCollider)
            .id();

        }
    }
}



fn wall_grow_check(
    target: Vec3,
    wall_query: &Query<&Transform, (Without<Player>,  With<TileCollider>)>,
)->bool{
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target,
            Vec2::splat(0.9*TILE_SIZE),
            wall_transform.translation,
            Vec2::splat(0.9*TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn double_grow_check(
    target:Vec3,
    grass_query: &Query<&Grass>
)->bool{
    for existing_grass in grass_query.iter(){
        let collision = collide(
            target,
            Vec2::splat(0.9*TILE_SIZE),
            existing_grass.position,
            Vec2::splat(0.9*TILE_SIZE),
        );
        if collision.is_some(){
            return false;
        }
    }
    true
}
