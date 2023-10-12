use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use rand::{thread_rng, Rng};
use strum_macros::Display;

#[derive(Bundle)]
pub struct TileBundle {
    pub name: Name,
    pub scene: SceneBundle,
    pub tile_type: TileType,
    pub tile_pos: TilePos,
}

impl TileBundle {
    pub fn random(tile_pos: TilePos, asset_server: &Res<AssetServer>) -> Self {
        let tile_type = TileType::random();
        let height: f32 = rand::thread_rng().gen_range(0.0..0.05);
        Self {
            name: tile_type.name(),
            scene: SceneBundle {
                scene: tile_type.scene_handle(&asset_server),
                transform: Transform::from_xyz(tile_pos.x as f32, height, tile_pos.y as f32),
                ..default()
            },
            tile_type,
            tile_pos,
        }
    }
}

#[derive(Component, Default, Display)]
pub enum TileType {
    #[default]
    TileGrass,
    TileStone,
    TileWood,
}

impl TileType {
    fn asset_path(&self) -> String {
        format!("models/{self}.glb#Scene0")
    }

    pub fn name(&self) -> Name {
        Name::new(self.to_string())
    }

    pub fn scene_handle(&self, asset_server: &Res<AssetServer>) -> Handle<Scene> {
        asset_server.load(self.asset_path())
    }

    // Weighted random; 50% grass, 20% stone, 30% wood
    pub fn random() -> TileType {
        use TileType::*;
        match thread_rng().gen_range(0..100) {
            n if n < 50 => TileGrass,
            n if n < 70 => TileStone,
            n if n < 100 => TileWood,
            _ => TileGrass,
        }
    }

    pub fn is_plantable(&self) -> bool {
        use TileType::*;
        match self {
            TileStone => false,
            _ => true,
        }
    }
}

#[derive(Component)]
pub struct Plantable;
