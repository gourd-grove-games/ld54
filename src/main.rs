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
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
#[cfg(feature = "inspector")] // egui inspector does not work on wasm
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::*;
mod tilemap;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb_linear(0.5, 1.3, 1.9)))
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .insert_resource(AmbientLight {
            color: Color::rgb_u8(210, 220, 240),
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Permaculture Tycoon"),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(tilemap::GroundMapPlugin);

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::default());

    app.add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_emmissive_cube)
        .add_systems(Update, png_metadata)
        .run();
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let skybox_handle: Handle<Image> = asset_server.load("textures/Ryfjallet_cubemap.png");
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle.clone(),
    });

    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
                transform: Transform::from_xyz(8.0, 4.5, 8.0),
                ..default()
            },
            BloomSettings::default(),
            PanOrbitCamera { ..default() },
            Skybox(skybox_handle),
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());
}

fn png_metadata(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
) {
    if !cubemap.is_loaded && asset_server.get_load_state(&cubemap.image_handle) == LoadState::Loaded
    {
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
        cubemap.is_loaded = true;
    }
}

fn spawn_emmissive_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_emissive1,
        transform: Transform::from_xyz(0.0, 2.5, 0.0),
        ..default()
    });
}
