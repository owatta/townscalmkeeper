use bevy::render::camera::ScalingMode;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

const GRID_SIZE: (usize, usize) = (30, 30);

const TILE_WIDTH: isize = 64;
const CAMERA_SPEED: f32 = 300.0;
const SELECTED_TILE_MARGIN: UiRect = UiRect::all(Val::Px(5.));
const DEFAULT_TILE_MARGIN: UiRect = UiRect::all(Val::Px(10.));

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
        (&Interaction, &mut Style, &Tile),
        (With<TileButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut style, kind) in &mut tile_transforms {
        style.padding = match interaction {
            Interaction::Pressed => {
                selected_tile_button.0 = panel_tiles.0.iter().position(|k| k == kind).unwrap();
                SELECTED_TILE_MARGIN
            }
            _ => DEFAULT_TILE_MARGIN,
        }
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

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                flex_wrap: FlexWrap::Wrap,
                // align_content: AlignContent::End,
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
                            "Text",
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
                            ..default()
                        },
                        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                        ..default()
                    },
                    BuildingPanel,
                ))
                .with_children(|parent| {
                    for kind in &panel_tiles.0 {
                        spawn_tile_button(parent, &asset_server, kind.clone());
                    }
                });
        });
}

fn spawn_tile_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, kind: Tile) {
    parent
        .spawn((
            TileButton,
            kind.clone(),
            ButtonBundle {
                style: Style {
                    height: Val::Percent(100.),
                    margin: UiRect::all(Val::Px(5.)) ,
                    padding: DEFAULT_TILE_MARGIN,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    height: Val::Percent(90.),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load(kind.sprite_path()),
                    ..default()
                },
                ..default()
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
