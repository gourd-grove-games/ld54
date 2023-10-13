use super::{Plant, Plantable};
use crate::tilemap::{pick::ClickTile, tile_type::TileType};
use bevy::prelude::*;

/// Examples of how to query for components alongside events.
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                log_plantable_tile_clicks,
                log_planted_tile_clicks,
                log_non_plantable_tile_clicks,
            ),
        );
    }
}

fn log_non_plantable_tile_clicks(
    mut tile_clicks: EventReader<ClickTile>,
    plantable_query: Query<&TileType, Without<Plantable>>, // TileType without Plantable
) {
    for event in tile_clicks.iter() {
        let Ok(_) = plantable_query.get(event.entity) else {
            continue;
        };
        let name = event.tile_type.name();
        info!(
            "CLICK {:?} NON-PLANTABLE {name} {:?}",
            event.button, event.tile_pos
        );
    }
}

fn log_plantable_tile_clicks(
    mut tile_clicks: EventReader<ClickTile>,
    plantable_query: Query<&Plantable, Without<Plant>>,
) {
    for event in tile_clicks.iter() {
        let Ok(_) = plantable_query.get(event.entity) else {
            continue;
        };
        let name = event.tile_type.name();
        info!(
            "CLICK {:?} PLANTABLE {name} {:?}",
            event.button, event.tile_pos
        );
    }
}

fn log_planted_tile_clicks(
    mut tile_clicks: EventReader<ClickTile>,
    plant_query: Query<&Plant, With<Plantable>>,
) {
    for event in tile_clicks.iter() {
        let Ok(plant) = plant_query.get(event.entity) else {
            continue;
        };
        let name = event.tile_type.name();
        info!(
            "CLICK {:?} ALREADY PLANTED {plant} on {name} {:?}",
            event.button, event.tile_pos
        );
    }
}
