//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::pbr::DirectionalLightShadowMap;
use bevy_panorbit_camera::*;

const BOARD_SIZE_I: usize = 5;
const BOARD_SIZE_J: usize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        //.add_systems(Update, adjust_directional_light_biases)
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .run();
}

enum BaseTileType {
    Grass,
    Stone,
    Wood
}

struct BaseTile {
    tile_type: BaseTileType,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cell_scene = asset_server.load("models/grass_tile.glb#Scene0");
    let board: Vec<Vec<BaseTile>> = (0..BOARD_SIZE_J)
        .map(|j| {
            (0..BOARD_SIZE_I)
                .map(|i| {  
                    let height = 0.0;//rand::thread_rng().gen_range(-0.1..0.1);
                    commands.spawn(SceneBundle {
                        transform: Transform::from_xyz(i as f32 - (BOARD_SIZE_I as f32 / 2.0), height - 0.2, j as f32 - (BOARD_SIZE_J as f32 / 2.0)),
                        scene: cell_scene.clone(),
                        ..default()
                    });
                    BaseTile { tile_type: BaseTileType::Grass }
                })
                .collect()
        })
        .collect();

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