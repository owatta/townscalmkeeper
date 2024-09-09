use rand::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ConveyorGenerator;

#[derive(Component)]
pub struct ConveyorBelt;

#[derive(Component)]
pub struct ConveyorBin;

#[derive(Component, Clone, Copy)]
pub enum Shape {
    Triangle,
    Circle,
    Square,
    Star,
    Rectangle,
}

#[derive(Component, Clone, Copy)]
pub enum ShapeColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

pub fn random_shape() -> Shape {
    match rand::thread_rng().gen_range(0..5) {
        0 => Shape::Triangle,
        1 => Shape::Circle,
        2 => Shape::Square,
        3 => Shape::Star,
        4 => Shape::Rectangle,
        _ => unreachable!("Random generated number is out of range."),
    }
}

pub fn random_color() -> ShapeColor {
    match rand::thread_rng().gen_range(0..7) {
        0 => ShapeColor::Red,
        1 => ShapeColor::Orange,
        2 => ShapeColor::Yellow,
        3 => ShapeColor::Green,
        4 => ShapeColor::Blue,
        5 => ShapeColor::Purple,
        6 => ShapeColor::Pink,
        _ => unreachable!("Random generated number is out of range."),
    }
}
