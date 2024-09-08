use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(f32, f32);

#[derive(Component)]
pub struct Conveyor;

#[derive(Component)]
pub enum Shape {
    Triangle,
    Circle,
    Square,
    Star,
    Rectangle,
}
