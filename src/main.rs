use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod game_logic;
mod ui;
mod create_objects;

use crate::game_logic::GameLogicPlugin;
use crate::ui::UiPlugin;
use crate::create_objects::CreateObjectsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), PhysicsPlugins::default(), CreateObjectsPlugin, GameLogicPlugin, UiPlugin))
        .run();
}
