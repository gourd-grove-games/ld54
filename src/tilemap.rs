use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

pub const BOARD_SIZE_I: u32 = 8;
pub const BOARD_SIZE_J: u32 = 8;
pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tilemap)
            .add_systems(Update, gizmos);
    }
}

fn gizmos(mut gizmos: Gizmos) {
    let axis_len = 100.0;
    gizmos.line(Vec3::ZERO, Vec3::X * axis_len, Color::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * axis_len, Color::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * axis_len, Color::BLUE);
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize {
        x: BOARD_SIZE_I,
        y: BOARD_SIZE_J,
    };

    // TilemapBundle requires TileStorage component; a grid of tile entities
    // Create TileStorage with pre-allocated capacity
    let mut tile_storage = TileStorage::empty(map_size);
    let map_type = TilemapType::Square;

    // Create empty tilemap entity
    // added to each tile as tilemap_id component
    let tilemap_entity = commands.spawn_empty().id();
    commands.entity(tilemap_entity).with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn((
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..Default::default()
                        },
                        SceneBundle {
                            scene: TileType::random().scene_handle(&asset_server),
                            transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                            ..default()
                        },
                        Name::new("Tile"),
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    let tile_size = TilemapTileSize { x: 1.0, y: 1.0 };
    let grid_size = tile_size.into();

    // Insert TilemapBundle to the tilemap entity
    commands.entity(tilemap_entity).insert((
        bevy_ecs_tilemap::StandardTilemapBundle {
            grid_size,
            tile_size,
            size: map_size,
            map_type,
            storage: tile_storage,
            transform: Transform::from_xyz(
                (BOARD_SIZE_I - 1) as f32 / -2.0,
                0.0,
                (BOARD_SIZE_J - 1) as f32 / -2.0,
            ),
            ..default()
        },
        Name::new("Ground Tilemap"),
    ));
}

#[derive(Default)]
pub enum TileType {
    #[default]
    Grass,
    Stone,
    Wood,
}

impl TileType {
    fn asset_path(&self) -> &'static str {
        match self {
            TileType::Grass => "models/grass_tile.glb#Scene0",
            TileType::Stone => "models/stone_tile.glb#Scene0",
            TileType::Wood => "models/wood_tile.glb#Scene0",
        }
    }

    pub fn scene_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Scene> {
        asset_server.load(self.asset_path())
    }

    // Weighted random; 50% grass, 20% stone, 30% wood
    fn random() -> TileType {
        use TileType::*;
        let mut random = thread_rng();
        match random.gen_range(0..100) {
            n if n < 50 => Grass,
            n if n < 70 => Stone,
            n if n < 100 => Wood,
            _ => Grass,
        }
    }
}
