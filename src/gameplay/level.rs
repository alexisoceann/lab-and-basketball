use crate::GameState;
// use crate::gameplay::actions::Actions;
// use crate::loading::TextureAssets;
use avian3d::prelude::*;
use bevy::{
    prelude::*,
    color::palettes::*,
};

pub struct LevelPlugin;

#[derive(Component, Default, Clone)]
pub struct Level;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                spawn_level,
            ),
        );
        // .add_systems(
        //     Update,
        //     move_player.run_if(in_state(GameState::Playing))
        // );
    }
}

pub fn spawn_level(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        RigidBody::Static
        Collider::cylinder(4.0, 0.1)
        Mesh3d(asset_value(Cylinder::new(4.0, 0.1)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::from(tailwind::ORANGE_400)))
        Level
        DespawnOnExit<GameState>(GameState::Playing)
    });

    // Light
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
