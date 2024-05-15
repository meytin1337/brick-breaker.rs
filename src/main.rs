use bevy::prelude::*;

mod circle;

use crate::circle::shape::ShapePlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, ShapePlugin)).run();
}
