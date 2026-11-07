mod actions;
mod audio;
mod camera;
mod finish_areas;
mod level;
mod physics;
mod player;

use bevy::prelude::*;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use camera::CameraPlugin;
use level::LevelPlugin;
// use finish_areas::FinishAreaPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            InternalAudioPlugin,
            CameraPlugin,
            // FinishAreaPlugin,
            LevelPlugin,
            PhysicsPlugin,
            PlayerPlugin,
        ));
    }
}
