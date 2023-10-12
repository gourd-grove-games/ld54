use bevy::math::vec4;
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_mod_picking::prelude::*;

use crate::plants::{self, Plant};

use super::tile_type::Plantable;

#[derive(Event)]
pub struct ClickTile {
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
pub fn handle_tile_click(
    tile_query: Query<(&Name, &TilePos)>,
    parent_query: Query<&Parent>,
    plantable_query: Query<&Plantable>,
    plant_query: Query<&plants::Plant>,
    mut greetings: EventReader<ClickTile>,
    mut commands: Commands,
) {
    for event in greetings.iter() {
        // Traverse 3 layers of parents to get the tile entity's components
        let entity = event.entity;
        for (i, ancestor) in parent_query.iter_ancestors(entity).enumerate() {
            if i == 2 {
                if let Ok((name, tile_pos)) = tile_query.get(ancestor) {
                    let plantable = match plantable_query.get(ancestor) {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    if plantable && event.button == PointerButton::Primary {
                        if let Ok(_) = plant_query.get(ancestor) {
                            info!("Plant already exists at {:?} {:?}", tile_pos, name);
                            return;
                        } else {
                            commands.entity(ancestor).insert(Plant);
                            info!("Planting at {:?} {:?}", tile_pos, name);
                            return;
                        }
                    }
                    info!(
                        "CLICK {:?} {name} {:?}; {:?}; depth: {:?}; plantable: {plantable}",
                        event.button, ancestor, tile_pos, event.depth
                    );
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
