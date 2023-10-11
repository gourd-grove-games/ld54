use bevy::math::vec4;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mod_picking::prelude::*;
use rand::{thread_rng, Rng};
use strum_macros::Display;

pub const BOARD_SIZE_I: u32 = 8;
pub const BOARD_SIZE_J: u32 = 8;

pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
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
        ))
        .with_children(|parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let height: f32 = rand::thread_rng().gen_range(0.0..0.05);
                    let tile_type = TileType::random();
                    let tile_entity = parent
                        .spawn((
                            SceneBundle {
                                scene: tile_type.scene_handle(&asset_server),
                                transform: Transform::from_xyz(x as f32, height, y as f32),
                                ..default()
                            },
                            tile_pos,
                            tile_type.name(),
                            PickableBundle::default(),
                            On::<Pointer<Click>>::send_event::<ClickTile>(),
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(tile_storage);

    info!("spawned tilemap");
}

#[derive(Default, Display)]
pub enum TileType {
    #[default]
    TileGrass,
    TileStone,
    TileWood,
}

impl TileType {
    fn asset_path(&self) -> &'static str {
        use TileType::*;
        match self {
            TileGrass => "models/grass_tile.glb#Scene0",
            TileStone => "models/stone_tile.glb#Scene0",
            TileWood => "models/wood_tile.glb#Scene0",
        }
    }

    fn name(&self) -> Name {
        Name::new(self.to_string())
    }

    pub fn scene_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Scene> {
        asset_server.load(self.asset_path())
    }

    // Weighted random; 50% grass, 20% stone, 30% wood
    fn random() -> TileType {
        use TileType::*;
        match thread_rng().gen_range(0..100) {
            n if n < 50 => TileGrass,
            n if n < 70 => TileStone,
            n if n < 100 => TileWood,
            _ => TileGrass,
        }
    }
}

/// Used to tint the mesh instead of simply replacing the mesh's material with a single color. See
/// `tinted_highlight` for more details.
const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.5, -0.3, 0.9, 0.8), // hovered is blue
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, -0.4, 0.8, 0.8), // pressed is a different blue
        ..matl.to_owned()
    })),
    // selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
    //     base_color: matl.base_color + vec4(-0.4, 0.8, -0.4, 0.0), // selected is green
    //     ..matl.to_owned()
    // })),
};

/// Makes everything in the scene with a mesh pickable
fn make_pickable(
    mut commands: Commands,
    meshes: Query<(Entity, &Name), (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    for (entity, name) in meshes.iter() {
        info!("Setting Pickable {name} entity: {:?}", entity);
        commands.entity(entity).insert((
            PickableBundle::default(),
            RaycastPickTarget::default(),
            HIGHLIGHT_TINT.clone(),
        ));
    }
}

#[derive(Event)]
struct ClickTile {
    button: PointerButton,
    entity: Entity,
    depth: f32,
}

impl From<ListenerInput<Pointer<Click>>> for ClickTile {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        ClickTile {
            entity: event.target,
            depth: event.hit.depth,
            button: event.button,
        }
    }
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
fn handle_tile_click(
    tile_query: Query<(&Name, &TilePos)>,
    parent_query: Query<&Parent>,
    mut greetings: EventReader<ClickTile>,
) {
    for event in greetings.iter() {
        // Traverse 3 layers of parents to get the tile entity's components
        let entity = event.entity;
        for (i, ancestor) in parent_query.iter_ancestors(entity).enumerate() {
            if i == 2 {
                if let Ok((name, tile_pos)) = tile_query.get(ancestor) {
                    info!(
                        "CLICK {:?} {name} {:?}; {:?}; depth: {:?}",
                        event.button, ancestor, tile_pos, event.depth
                    );
                }
            }
        }
    }
}
