use bevy::prelude::*;

const GRID_SIZE: (usize, usize) = (30, 30);
const TILE_WIDTH: usize = 32;

enum Tile {
    SmallHouse,
    MediumHouse,
    BigHouse,
    Wiring,
    Plumbing,
    Road,
    PowerStation,
    WaterSource,
    Empty,
}

#[derive(Component)]
struct TileBundle {
    kind: Tile,
    sprite: SpriteBundle,
}

impl Tile {
    fn sprite_path(&self) -> String {
        match &self {
            Tile::SmallHouse => "sprites/small_house.png".to_string(),
            Tile::MediumHouse => "sprites/medium_house.png".to_string(),
            _ => "blah".to_string(),
        }
    }
}

#[derive(Component)]
struct Position(usize, usize);

fn put_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    kind: Tile,
    pos: (usize, usize)
) {
    let sprite = SpriteBundle {
        texture: asset_server.load(kind.sprite_path()),
        ..default()
    };
    commands.spawn((Position(pos.0, pos.1), sprite));
}

fn update_tile_sprite_positions(mut tiles: Query<(&Position, &mut Transform)>) {
    // determines each tile's position
    // puts appropriate Transform value
    for (pos, mut transform) in &mut tiles {
        transform.translation.x = (pos.0 * TILE_WIDTH) as f32;
        transform.translation.y = (pos.1 * TILE_WIDTH) as f32;
    }
}

#[derive(Resource)]
struct Wallet(i32);

fn give_money(mut wallet: ResMut<Wallet>, tiles: Query<&TileBundle>) {
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
    let _app = App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).run();
}

fn setup(commands: Commands, asset_server: Res<AssetServer>) {
    put_tile(commands, asset_server, Tile::SmallHouse, (3, 3));
}

// TODOs:
// - make a house

// unimportant:
// - find a better way to handle Resources (stuff like `resource.0`
// for value access)
