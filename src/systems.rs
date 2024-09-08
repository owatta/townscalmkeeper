use bevy::{ prelude::*, sprite::{ MaterialMesh2dBundle, Mesh2dHandle } };

use crate::{ resources::*, components::* };

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>
) {
    windows.single_mut().resolution.set(600.0, 600.0);
    commands.spawn(Camera2dBundle::default());

    let player_handle = Mesh2dHandle(meshes.add(Rectangle::new(20.0, 50.0)));
    let player_bundle = MaterialMesh2dBundle {
        mesh: player_handle,
        material: materials.add(Color::srgb(0.0, 255.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };
    commands.spawn((Player, player_bundle));

    let conveyor_handle = Mesh2dHandle(meshes.add(Rectangle::new(200.0, 20.0)));
    let conveyor_bundle = MaterialMesh2dBundle {
        mesh: conveyor_handle,
        material: materials.add(Color::srgb(127.0, 127.0, 200.0)),
        transform: Transform::from_xyz(-100.0, 0.0, 0.0),
        ..default()
    };
    commands.spawn((Conveyor, conveyor_bundle));
}

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut binding = query.single_mut();
    let player_transform = binding.as_mut();
    if keys.pressed(KeyCode::ArrowUp) {
        player_transform.translation.y += 5.0;
        player_transform.translation.z += 0.01;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        player_transform.translation.y -= 5.0;
        player_transform.translation.z -= 0.01;
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
    time: Res<Time>,
    mut timer: ResMut<ConveyorTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Mesh2dHandle, &Transform), With<Conveyor>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mesh, transform) in &query {
            let shape_handle = Mesh2dHandle(
                meshes.add(
                    Triangle2d::new(Vec2::Y * 10.0, Vec2::new(-10.0, -10.0), Vec2::new(10.0, -10.0))
                )
            );
            let shape_transform = Transform::from_xyz(
                transform.translation.x - 100.0,
                transform.translation.y + 20.0,
                0.0
            );
            let shape_bundle = MaterialMesh2dBundle {
                mesh: shape_handle,
                material: materials.add(Color::srgb(255.0, 0.0, 0.0)),
                transform: shape_transform,
                ..default()
            };
            commands.spawn((Shape::Triangle, shape_bundle));
        }
    }
}

pub fn shapes_movement_system(mut query: Query<&mut Transform, With<Shape>>) {
    for mut shape_transform in &mut query {
        shape_transform.translation.x += 0.5;
    }
}
