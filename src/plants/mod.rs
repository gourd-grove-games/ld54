use bevy::prelude::*;
pub mod logger;
use bevy_mod_picking::prelude::PointerButton;
use rand::{thread_rng, Rng};
use strum_macros::Display;

pub struct PlantingPlugin;
impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, plant_on_primary_click);
    }
}

use crate::tilemap::{pick::ClickTile, tile_type::TileType};
#[derive(Component, Default, Clone, Copy, Display)]
pub enum Plant {
    PlantTree,
    PlantShrub,
    #[default]
    PlantFlower,
}
impl Plant {
    pub fn random() -> Self {
        use Plant::*;
        match thread_rng().gen_range(0..3) {
            0 => PlantTree,
            1 => PlantShrub,
            2 => PlantFlower,
            _ => PlantFlower,
        }
    }

    fn asset_path(&self) -> String {
        format!("models/{self}/{self}.gltf#Scene0")
    }

    fn scale(&self) -> f32 {
        use Plant::*;
        match self {
            PlantTree => 0.2,
            PlantShrub => 0.01,
            PlantFlower => 0.01,
        }
    }

    pub fn scene_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Scene> {
        asset_server.load(self.asset_path())
    }
}

#[derive(Component)]
pub struct Plantable;

fn plant_on_primary_click(
    mut tile_clicks: EventReader<ClickTile>,
    plantable_query: Query<(&TileType, &Plantable), Without<Plant>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in tile_clicks.iter() {
        let Ok(_) = plantable_query.get(event.entity) else {
            continue;
        };
        if event.button == PointerButton::Primary {
            let plant = Plant::random();
            info!(
                "PLANTING {plant} ON {name} {:?}",
                event.tile_pos,                // unnamed arg tile_pos
                name = event.tile_type.name()  // named args must be last
            );
            commands
                .entity(event.entity)
                .insert(plant)
                .with_children(|parent| {
                    parent.spawn(SceneBundle {
                        scene: plant.scene_handle(&asset_server),
                        transform: Transform::from_scale(Vec3::splat(plant.scale())),
                        ..default()
                    });
                });
        }
    }
}
