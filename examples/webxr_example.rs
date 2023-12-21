//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{settings::WgpuSettings, texture::DefaultImageSampler, RenderPlugin},
    winit::WinitPlugin,
};
use bevy_webxr::{
    camera::update_webxr_camera, init::initialize_canvas, WebXrPlugin, WebXrSettings,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

fn main() {
    let mut app = App::new();
    app.add_plugins(LogPlugin::default());

    info!("{:?}", initialize_canvas("bevyxr"));
    info!("HI!");

    app.insert_resource(ClearColor(Color::GOLD));

    app.insert_resource(Msaa::Off).add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("canvas[bevyxr=\"bevyxr\"]".to_string()),
                    ..default()
                }),
                ..default()
            })
            //.disable::<WinitPlugin>()
            .disable::<LogPlugin>(),
    );

    app.add_plugins(WebXrPlugin {
        settings: WebXrSettings {
            canvas: "bevyxr".to_string(),
            ..default()
        },
    })
    .add_systems(Startup, setup)
    .add_systems(
        PreUpdate,
        bevy_webxr::camera::spawn_webxr_camera
            .run_if(on_event::<bevy_webxr::events::WebXrSessionInitialized>()),
    )
    .add_systems(Update, update_webxr_camera)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
