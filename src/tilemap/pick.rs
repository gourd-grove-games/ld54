use bevy::math::vec4;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_mod_picking::prelude::*;

use super::tile_type::TileType;

#[derive(Event)]
pub struct ClickMesh {
    button: PointerButton,
    entity: Entity,
}

#[derive(Event)]
pub struct ClickTile {
    pub entity: Entity,
    pub tile_pos: TilePos,
    pub tile_type: TileType,
    pub button: PointerButton,
}

impl From<ListenerInput<Pointer<Click>>> for ClickMesh {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        ClickMesh {
            entity: event.target,
            button: event.button,
        }
    }
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
pub fn click_mesh_to_tile(
    tile_query: Query<(&TileType, &TilePos)>,
    parent_query: Query<&Parent>,
    mut mesh_clicks: EventReader<ClickMesh>,
    mut tile_clicks: EventWriter<ClickTile>,
) {
    for event in mesh_clicks.iter() {
        // Traverse 3 layers of parents to get the tile entity's components
        let entity = event.entity;
        for (i, ancestor) in parent_query.iter_ancestors(entity).enumerate() {
            if i == 2 {
                if let Ok((tile_type, tile_pos)) = tile_query.get(ancestor) {
                    tile_clicks.send(ClickTile {
                        entity: ancestor,
                        tile_pos: *tile_pos,
                        tile_type: *tile_type,
                        button: event.button,
                    });
                }
            }
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
pub fn make_pickable(
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
