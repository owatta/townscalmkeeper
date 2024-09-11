use bevy::render::camera::ScalingMode;
use bevy::prelude::*;

const GRID_SIZE: (usize, usize) = (30, 30);

const TILE_WIDTH: isize = 64;
const CAMERA_SPEED: f32 = 300.0;

enum Tile {
    SmallHouse,
    MediumHouse,
    BigHouse,
    Wiring,
    Plumbing,
    Road,
    PowerPlant,
    WaterSource,
    Empty,
}

#[derive(Resource)]
struct IncomeTimer(Timer);

#[derive(Component)]
struct Label;

#[derive(Component)]
struct TileBundle {
    kind: Tile,
    sprite: SpriteBundle,
}

impl Tile {
    fn sprite_path(&self) -> String {
        match &self {
	    Tile::SmallHouse => "sprites/small_house.png".to_string(),
	    Tile::PowerPlant => "sprites/powerplant.png".to_string(),
            _ => "sprites/empty.png".to_string(),
        }
    }
}

#[derive(Component)]
struct Position(isize, isize);

fn put_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    kind: Tile,
    pos: (isize, isize)
) {
    let sprite = SpriteBundle {
        texture: asset_server.load(kind.sprite_path()),
        ..default()
    };
    commands.spawn((Position(pos.0, pos.1), sprite));
}

fn update_tile_sprite_positions(mut tiles: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut tiles {
        transform.translation.x = (pos.0 * TILE_WIDTH) as f32;
        transform.translation.y = (pos.1 * TILE_WIDTH) as f32;
    }
}

#[derive(Resource)]
struct Wallet(i32);

fn give_money(
    time: Res<Time>,
    mut timer: ResMut<IncomeTimer>,
    mut wallet: ResMut<Wallet>,
    tiles: Query<&TileBundle>
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for tile in &tiles {
        match tile.kind {
            Tile::SmallHouse => {
                wallet.0 += 10;
            }
            _ => (),
        }
    }
}

fn main() {
    let _app = App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .insert_resource(IncomeTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(Wallet(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_tile_sprite_positions, give_money))
        .add_systems(FixedUpdate, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    setup_ui(&mut commands, &asset_server);
    
    put_tile(&mut commands, &asset_server, Tile::SmallHouse, (1, 0));
    put_tile(&mut commands, &asset_server, Tile::PowerPlant, (1, 1));
}

fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer> ) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            // align_content: AlignContent::End,
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.),
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            background_color: Color::srgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("Text", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                }),
                Label,
            ));
        });
    });
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut camera_transform = query.single_mut();
    let mut direction = (0.0, 0.0); // x, y

    if keys.pressed(KeyCode::KeyW) {
	direction.1 += 1.0;
    };
    if keys.pressed(KeyCode::KeyA) {
	direction.0 -= 1.0;
    };
    if keys.pressed(KeyCode::KeyS) {
	direction.1 -= 1.0;
    };
    if keys.pressed(KeyCode::KeyD) {
	direction.0 += 1.0;
    };

    direction.0 = direction.0 * CAMERA_SPEED * time.delta_seconds();
    direction.1 = direction.1 * CAMERA_SPEED * time.delta_seconds();
    camera_transform.translation.x += direction.0;
    camera_transform.translation.y += direction.1;
}

// TODOs:
// - find a better way to handle Resources (stuff like `resource.0`
// for value access looks like shit)
