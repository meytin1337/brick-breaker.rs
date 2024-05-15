use bevy::prelude::*;

mod game_logic;

mod create_objects;

use crate::game_logic::GameLogicPlugin;

use crate::create_objects::CreateObjectsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CreateObjectsPlugin, GameLogicPlugin))
        .run();
}
