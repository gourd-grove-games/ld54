use bevy::math::vec4;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mod_picking::prelude::*;
use rand::{thread_rng, Rng};

pub const BOARD_SIZE_I: u32 = 8;
pub const BOARD_SIZE_J: u32 = 8;

pub struct GroundMapPlugin;
impl Plugin for GroundMapPlugin {
    fn build(&self, app: &mut App) {
        let picking_plugin = DefaultPickingPlugins.build();
        #[cfg(not(feature = "inspector"))]
        let picking_plugin = picking_plugin.disable::<DebugPickingPlugin>();
        app.add_plugins(picking_plugin)
            .add_systems(Startup, spawn_tilemap)
            .add_systems(Update, make_pickable);
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
            On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>| {
                info!(
                    "Clicked on entity {:?}; pos: {:?}",
                    event.target, event.hit.position
                );
            }),
            Name::new("Ground Tilemap"),
        ))
        .with_children(|parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let height: f32 = rand::thread_rng().gen_range(0.0..0.05);
                    let tile_entity = parent
                        .spawn((
                            SceneBundle {
                                scene: TileType::random().scene_handle(&asset_server),
                                transform: Transform::from_xyz(x as f32, height, y as f32),
                                ..default()
                            },
                            Name::new("Tile"),
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        })
        .insert(tile_storage);

    info!("spawned tilemap");
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
        match thread_rng().gen_range(0..100) {
            n if n < 50 => Grass,
            n if n < 70 => Stone,
            n if n < 100 => Wood,
            _ => Grass,
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
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + vec4(-0.4, 0.8, -0.4, 0.0), // selected is green
        ..matl.to_owned()
    })),
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
