use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mod_picking::prelude::*;
use rand::Rng;
mod pick;
mod tile_type;

pub const BOARD_SIZE_I: u32 = 8;
pub const BOARD_SIZE_J: u32 = 8;

pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
        use pick::*;
        let picking_plugin = DefaultPickingPlugins.build();
        #[cfg(not(feature = "inspector"))]
        let picking_plugin = picking_plugin.disable::<DebugPickingPlugin>();
        app.add_event::<ClickTile>()
            .add_plugins(picking_plugin)
            .add_systems(Startup, spawn_tilemap)
            .add_systems(Update, (make_pickable, handle_tile_click));
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize {
        x: BOARD_SIZE_I,
        y: BOARD_SIZE_J,
    };

    // TilemapBundle requires TileStorage component; a grid of tile entities
    // Create TileStorage with pre-allocated capacity
    let mut tile_storage = TileStorage::empty(map_size);

    // Create empty tilemap entity
    // added to each tile as tilemap_id component
    commands
        .spawn((
            Visibility::Visible,
            ComputedVisibility::default(),
            Transform::from_xyz(
                (BOARD_SIZE_I - 1) as f32 / -2.0,
                0.0,
                (BOARD_SIZE_J - 1) as f32 / -2.0,
            ),
            GlobalTransform::default(),
            Name::new("Ground Tilemap"),
            On::<Pointer<Click>>::send_event::<pick::ClickTile>(),
        ))
        .with_children(|parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let height: f32 = rand::thread_rng().gen_range(0.0..0.05);
                    let tile_type = tile_type::TileType::random();
                    let tile_entity = parent
                        .spawn((
                            SceneBundle {
                                scene: tile_type.scene_handle(&asset_server),
                                transform: Transform::from_xyz(x as f32, height, y as f32),
                                ..default()
                            },
                            tile_pos,
                            tile_type.name(),
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(tile_storage);

    info!("spawned tilemap");
}
