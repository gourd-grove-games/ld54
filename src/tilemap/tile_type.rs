use bevy::prelude::*;
use rand::{thread_rng, Rng};
use strum_macros::Display;

#[derive(Default, Display)]
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
}
