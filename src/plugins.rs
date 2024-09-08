use bevy::prelude::*;

use crate::{ resources::*, systems::* };

pub struct ShapeConveyorPlugin;

impl Plugin for ShapeConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConveyorTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Update, (shape_spawner_system, shapes_movement_system));
    }
}
