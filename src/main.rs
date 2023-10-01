//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

use std::f32::consts::PI;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    core_pipeline::Skybox,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    pbr::ScreenSpaceAmbientOcclusionBundle,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
    },
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::*;
mod camera;
mod tilemap;

const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    ("textures/bsb.png", CompressedImageFormats::NONE),
    (
        "textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR,
    ),
    (
        "textures/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC,
    ),
    (
        "textures/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2,
    ),
];

fn main() {
    App::new()
        // .init_resource::<Board>()
        // .register_type::<BoardPos>()
        // .register_type::<BaseTile>()
        // .register_type::<Board>()
        // .insert_resource(DirectionalLightShadowMap { size: 2048 })
        // .insert_resource(ClearColor(Color::rgb_linear(0.5, 1.3, 1.9)))
        // .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Permaculture Tycoon"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ) // .add_plugins(PanOrbitCameraPlugin)
        // .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(tilemap::GroundMapPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        // .add_systems(Startup, spawn_camera)
        // .add_systems(Startup, setup)
        // .add_systems(
        //     Update,
        //     (cycle_cubemap_asset, asset_loaded.after(cycle_cubemap_asset)),
        // )
        .run();
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Board {
    i_len: u32,
    j_len: u32,
}

// todo deprecate
impl Default for Board {
    fn default() -> Self {
        Board {
            i_len: tilemap::BOARD_SIZE_X,
            j_len: tilemap::BOARD_SIZE_Y,
        }
    }
}

impl Board {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

fn name_tile() -> Name {
    Name::new("Tile")
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
    i: u32,
    j: u32,
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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

    // let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    // commands
    //     .spawn((
    //         Camera3dBundle {
    //             camera: Camera {
    //                 hdr: true,
    //                 ..default()
    //             },
    //             tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
    //             transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
    //             ..default()
    //         },
    //         BloomSettings::default(),
    //         PanOrbitCamera::default(),
    //         Skybox(skybox_handle.clone()),
    //     ))
    //     .insert(ScreenSpaceAmbientOcclusionBundle::default())
    //     .insert(TemporalAntiAliasBundle::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
    board.spawn(&mut commands, &asset_server);
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });

    // let grass_scene = asset_server.load("models/grass_tile.glb#Scene0");
    // let stone_scene = asset_server.load("models/stone_tile.glb#Scene0");
    // let wood_scene = asset_server.load("models/wood_tile.glb#Scene0");
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_emissive1,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_seconds();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!("Skipping unsupported format: {:?}", CUBEMAPS[new_index]);
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.get_load_state(&cubemap.image_handle) == LoadState::Loaded
    {
        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
