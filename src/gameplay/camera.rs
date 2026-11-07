use bevy::prelude::*;
use crate::GameState;
use crate::gameplay::level::spawn_level;
use crate::gameplay::player::{spawn_player};

pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera {
    target: Entity,
    slot: usize,
    player_count: usize,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_game_camera
                .after(spawn_player)
                .after(spawn_level)
        );
            // .add_systems(
            //     Update,
            //     (follow_target)
            //         .run_if(in_state(GameState::Playing)),
            // );
    }
}

pub fn spawn_game_camera(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Camera3d::default()
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        }
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y)
        DespawnOnExit<GameState>(GameState::Playing)
    });
}
