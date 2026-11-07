use crate::GameState;
// use crate::gameplay::actions::Actions;
// use crate::loading::TextureAssets;
use avian3d::prelude::*;
use bevy::{
    prelude::*,
    color::palettes::*,
};

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                spawn_player,
            ),
        );
        // .add_systems(
        //     Update,
        //     move_player.run_if(in_state(GameState::Playing))
        // );
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        RigidBody::Dynamic
        Collider::cuboid(1.0, 1.0, 1.0)
        Mesh3d(asset_value(Cuboid::new(1., 1., 1.)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::from(tailwind::EMERALD_600)))
        Player
        Transform::from_xyz(0.0, 4.0, 0.0)
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5))
        DespawnOnExit<GameState>(GameState::Playing)
    });
}
