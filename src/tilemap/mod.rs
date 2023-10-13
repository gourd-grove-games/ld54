use crate::plants::Plantable;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mod_picking::prelude::*;
use tile_type::TileBundle;
pub mod pick;
pub mod tile_type;

pub const BOARD_SIZE_I: u32 = 8;
pub const BOARD_SIZE_J: u32 = 8;

pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
        use pick::*;
        let picking_plugin = DefaultPickingPlugins.build();
        #[cfg(not(feature = "debug"))]
        let picking_plugin = picking_plugin.disable::<DebugPickingPlugin>();
        app.add_event::<ClickMesh>()
            .add_event::<ClickTile>()
            .add_plugins(picking_plugin)
            .add_systems(Startup, spawn_tilemap)
            .add_systems(Update, (make_pickable, click_mesh_to_tile));
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
            On::<Pointer<Click>>::send_event::<pick::ClickMesh>(),
        ))
        .with_children(|parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let tile_bundle = TileBundle::random(tile_pos, &asset_server);
                    let tile_entity = if tile_bundle.tile_type.is_plantable() {
                        parent.spawn((tile_bundle, Plantable)).id()
                    } else {
                        parent.spawn(tile_bundle).id()
                    };
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(tile_storage);

    info!("spawned tilemap");
}
