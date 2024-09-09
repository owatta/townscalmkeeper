use bevy::{ prelude::*, sprite::{ MaterialMesh2dBundle, Mesh2dHandle } };

use crate::{ resources::*, components::* };

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut windows: Query<&mut Window>
) {
    windows.single_mut().resolution.set(1280.0, 720.0);
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("textures/human.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    spawn_conveyor(commands, &asset_server, Transform::from_xyz(-200.0, 0.0, 0.0));
}

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut binding = query.single_mut();
    let player_transform = binding.as_mut();
    if keys.pressed(KeyCode::ArrowUp) {
        player_transform.translation.y += 5.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        player_transform.translation.y -= 5.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= 5.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += 5.0;
    }
}

pub fn shape_spawner_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<ConveyorTimer>,
    query: Query<&Transform, With<ConveyorGenerator>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for transform in &query {
            let shape_transform = Transform::from_xyz(
                transform.translation.x,
                transform.translation.y + 10.0,
                0.1
            );
            let shape = random_shape();
            let shape_color = random_color();
            commands.spawn((
                shape,
                shape_color,
                generate_shape_sprite(&asset_server, shape, shape_color, shape_transform),
            ));
        }
    }
}

pub fn shapes_movement_system(mut query: Query<&mut Transform, With<Shape>>) {
    for mut shape_transform in &mut query {
        shape_transform.translation.x += 0.5;
    }
}

pub fn shapes_despawning_system(
    mut commands: Commands,
    shapes: Query<(Entity, &Transform), With<Shape>>,
    bins: Query<&Transform, With<ConveyorBin>>
) {
    // Check if shape collides with any bin
    for (shape_entity, shape) in &shapes {
        for bin in &bins {
            if
                shape.translation.x > bin.translation.x - 64.0 &&
                shape.translation.x < bin.translation.x + 64.0
            {
                commands.entity(shape_entity).despawn();
                break;
            }
        }
    }
}

fn spawn_conveyor(
    mut commands: Commands,
    asset_server: &ResMut<AssetServer>,
    generator_transform: Transform
) {
    let belt_transform = Transform::from_xyz(
        generator_transform.translation.x + 128.0,
        generator_transform.translation.y,
        0.0
    );
    let bin_transform = Transform::from_xyz(
        generator_transform.translation.x + 300.0,
        generator_transform.translation.y,
        0.0
    );
    commands.spawn((
        ConveyorBelt,
        SpriteBundle {
            texture: asset_server.load("textures/conveyor_belt.png"),
            transform: belt_transform,
            ..default()
        },
    ));
    commands.spawn((
        ConveyorGenerator,
        SpriteBundle {
            texture: asset_server.load("textures/conveyor_generator.png"),
            transform: generator_transform,
            ..default()
        },
    ));
    commands.spawn((
        ConveyorBin,
        SpriteBundle {
            texture: asset_server.load("textures/conveyor_bin.png"),
            transform: bin_transform,
            ..default()
        },
    ));
}

fn generate_shape_sprite(
    asset_server: &ResMut<AssetServer>,
    shape: Shape,
    color: ShapeColor,
    transform: Transform
) -> SpriteBundle {
    let shape_name = match shape {
        Shape::Triangle => "triangle",
        Shape::Circle => "circle",
        Shape::Square => "square",
        Shape::Star => "circle",
        Shape::Rectangle => "circle",
    };
    let color_name = match color {
        ShapeColor::Red => "gray",
        ShapeColor::Orange => "gray",
        ShapeColor::Yellow => "gray",
        ShapeColor::Green => "gray",
        ShapeColor::Blue => "gray",
        ShapeColor::Purple => "gray",
        ShapeColor::Pink => "gray",
    };

    SpriteBundle {
        texture: asset_server.load(format!("textures/shape_{}_{}.png", shape_name, color_name)),
        transform,
        ..default()
    }
}
