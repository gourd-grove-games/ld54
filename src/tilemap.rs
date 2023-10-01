use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::prelude::*;

use crate::camera;

pub const BOARD_SIZE_X: u32 = 15;
pub const BOARD_SIZE_Y: u32 = 15;
pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, spawn_tilemap)
            .add_systems(Update, camera::movement)
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_rotation(Quat::from_rotation_x(PI / 3.0)),
        projection: OrthographicProjection {
            far: 1000.0,
            near: -1000.0,
            scaling_mode: ScalingMode::WindowSize(5.0),
            ..default()
        },
        ..default()
    });
    let grass_tex_handle: Handle<Image> = asset_server.load("textures/grass.png");
    let map_size = TilemapSize {
        x: BOARD_SIZE_X,
        y: BOARD_SIZE_Y,
    };

    // TilemapBundle requires TileStorage component; a grid of tile entities
    // Create TileStorage with pre-allocated capacity
    let mut tile_storage = TileStorage::empty(map_size);
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    // Create empty tilemap entity
    // added to each tile as tilemap_id component
    let tilemap_entity = commands.spawn_empty().id();

    // Fill the tile storage with tiles
    helpers::filling::fill_tilemap(
        TileTextureIndex(75),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    // Insert TilemapBundle to the tilemap entity
    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            tile_size,
            size: map_size,
            map_type,
            storage: tile_storage,
            texture: TilemapTexture::Single(grass_tex_handle),
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..default()
        },
        Name::new("Ground Tilemap"),
    ));
}
