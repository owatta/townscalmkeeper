use bevy::render::camera::ScalingMode;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

const GRID_SIZE: (usize, usize) = (30, 30);

const TILE_WIDTH: isize = 64;
const CAMERA_SPEED: f32 = 300.0;
const SELECTED_TILE_HEIGHT: Val = Val::Percent(95.);
const DEFAULT_TILE_HEIGHT: Val = Val::Percent(80.);
const DEFAULT_TILE_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);
const SELECTED_TILE_COLOR: Color = Color::srgb(0.50, 0.50, 0.75);

macro_rules! spawn_tile_button {
    ($parent:expr,$asset_server:expr,$kind:expr) => {{
        $parent
            .spawn((
                TileButton,
                $kind.clone(),
                ButtonBundle {
                    style: Style {
                        display: bevy::ui::Display::Grid,
                        height: DEFAULT_TILE_HEIGHT,
                        margin: UiRect::axes(Val::Px(5.), Val::Px(5.)),
                        padding: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.75, 0.75, 0.75)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        grid_row: GridPlacement::start(1),
                        ..default()
                    },
                    image: UiImage {
                        texture: $asset_server.load($kind.sprite_path()),
                        ..default()
                    },
                    ..default()
                });
                parent.spawn(
                    TextBundle::from_section(
                        format!("{} $", $kind.price()),
                        TextStyle {
                            font: $asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 18.0,
                            color: Color::srgb(0.0, 0.0, 0.0),
                            ..default()
                        },
                    )
                    .with_style(Style {
                        padding: UiRect::all(Val::Px(1.0)),
                        grid_row: GridPlacement::start(2),
                        justify_self: JustifySelf::Center,
                        ..default()
                    }),
                );
            });
    }};
}

#[derive(Component, Clone, PartialEq, Eq, Debug)]
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

#[derive(Resource)]
struct BuildingPanelTiles(Vec<Tile>);

#[derive(Resource)]
struct SelectedTileButton(usize);

#[derive(Component)]
struct Label;

#[derive(Component)]
struct WalletLabel;

#[derive(Component)]
struct TileButton;

#[derive(Component)]
struct BuildingPanel;

#[derive(Bundle)]
struct TileBundle {
    kind: Tile,
    sprite: SpriteBundle,
    position: Position,
}

impl Tile {
    fn sprite_path(&self) -> String {
        match &self {
	    Tile::SmallHouse => "sprites/small_house.png".to_string(),
	    Tile::PowerPlant => "sprites/powerplant.png".to_string(),
            _ => "sprites/empty.png".to_string(),
        }
    }

    fn price(&self) -> i32 {
        match &self {
            Tile::SmallHouse => 20,
            Tile::MediumHouse => 40,
            Tile::BigHouse => 80,
            Tile::Wiring => 10,
            Tile::Plumbing => 10,
            Tile::Road => 10,
            Tile::PowerPlant => 40,
            Tile::WaterSource => 40,
            Tile::Empty => 0,
        }
    }
}

#[derive(Component)]
struct Position(isize, isize);

fn put_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    kind: Tile,
    position: Position
) {
    let tile = TileBundle {
	kind: kind.clone(),
	sprite: SpriteBundle {
            texture: asset_server.load(kind.sprite_path()),
            ..default()
	},
	position,
    };
    commands.spawn(tile);
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
    tiles: Query<&Tile>
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for tile in &tiles {
        match tile {
            Tile::SmallHouse => {
                wallet.0 += 10;
            }
            _ => (),
        }
    }
}

fn update_wallet_label(wallet: Res<Wallet>, mut labels: Query<&mut Text, With<WalletLabel>>) {
    let text = labels.get_single_mut();
    match text {
        Ok(mut text) => {
            text.sections[0].value = format!("Wallet: {}", wallet.0);
        }
        Err(e) => warn!("Wallet label was not found. Error: {}", e),
    }
}

fn select_panel_tile(
    panel_tiles: Res<BuildingPanelTiles>,
    mut selected_tile_button: ResMut<SelectedTileButton>,
    mut tile_transforms: Query<
        (&Interaction, &mut BackgroundColor, &mut Style, &Tile),
        (With<TileButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg_color, mut style, kind) in &mut tile_transforms {
        let (height, color) = match interaction {
            Interaction::Pressed => {
                selected_tile_button.0 = panel_tiles.0.iter().position(|k| k == kind).unwrap();
                (SELECTED_TILE_HEIGHT, SELECTED_TILE_COLOR)
            }
            _ => (DEFAULT_TILE_HEIGHT, DEFAULT_TILE_COLOR),
        };
        style.height = height;
        bg_color.0  = color;
    }
}

fn main() {
    let _app = App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .insert_resource(IncomeTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(Wallet(0))
        .insert_resource(BuildingPanelTiles(vec![Tile::SmallHouse, Tile::PowerPlant]))
        .insert_resource(SelectedTileButton(0))
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                update_tile_sprite_positions,
                give_money,
                update_wallet_label,
                select_panel_tile.run_if(input_just_pressed(MouseButton::Left)),
            ),
        )
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

    put_tile(
        &mut commands,
        &asset_server,
        Tile::SmallHouse,
        Position(1, 0),
    );
    put_tile(
        &mut commands,
        &asset_server,
        Tile::PowerPlant,
        Position(1, 1),
    );
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    panel_tiles: Res<BuildingPanelTiles>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        margin: UiRect::all(Val::Px(5.)),
                        align_self: AlignSelf::Start,
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        WalletLabel,
                        TextBundle::from_section(
                            "Wallet: - $",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::srgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ),
                    ));
                });

            // Bottom panel
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(100.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                        ..default()
                    },
                    BuildingPanel,
                ))
                .with_children(|parent| {
                    for kind in &panel_tiles.0 {
                        spawn_tile_button!(parent, asset_server, kind.clone());
                    }
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
