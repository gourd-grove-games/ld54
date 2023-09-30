//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

use std::f32::consts::PI;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::*;

const BOARD_SIZE_I: usize = 5;
const BOARD_SIZE_J: usize = 5;

fn main() {
    App::new()
        .init_resource::<Board>()
        .register_type::<BoardPos>()
        .register_type::<BaseTile>()
        .register_type::<Board>()
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, tick_tiles)
        //.add_systems(Update, adjust_directional_light_biases)
        .add_plugins(WorldInspectorPlugin::default())
        .run();
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
enum BaseTile {
    #[default]
    Grass,
    Stone,
    Wood,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct BoardPos {
    i: usize,
    j: usize,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Board {
    i_len: usize,
    j_len: usize,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            i_len: BOARD_SIZE_I,
            j_len: BOARD_SIZE_I,
        }
    }
}

fn name_tile() -> Name {
    Name::new("Tile")
}

impl Board {
    fn spawn(&self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let cell_scene = asset_server.load("models/grass_tile.glb#Scene0");
        for j in 0..self.i_len {
            for i in 0..self.j_len {
                let height = 0.0; //rand::thread_rng().gen_range(-0.1..0.1);
                commands.spawn((
                    SceneBundle {
                        transform: Transform::from_xyz(
                            i as f32 - (self.i_len as f32 / 2.0),
                            height - 0.2,
                            j as f32 - (self.j_len as f32 / 2.0),
                        ),
                        scene: cell_scene.clone(),
                        ..default()
                    },
                    BaseTile::default(),
                    BoardPos { i, j },
                    name_tile(),
                ));
            }
        }
    }
}

fn tick_tiles(query: Query<(&BoardPos, &BaseTile)>) {
    for (pos, tile) in query.iter() {
        info!("Tile at {},{} is {:?}", pos.i, pos.j, tile);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    board.spawn(&mut commands, asset_server);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI / 2.,
            -PI / 4.,
        )),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
