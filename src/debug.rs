use bevy::prelude::*;
use bevy::utils::tracing::callsite::register;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin,Inspectable};
use crate::player::Player;

pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App){
        if cfg!(debug_assertions){
            app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Player>();
            ;
        }

    }
}