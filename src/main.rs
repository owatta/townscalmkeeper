use bevy::{ prelude::*, sprite::Wireframe2dPlugin };

use systems::*;
use plugins::*;

mod resources;
mod components;
mod systems;
mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(ShapeConveyorPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement_system, shape_spawner_system, shapes_movement_system))
        .run();
}
