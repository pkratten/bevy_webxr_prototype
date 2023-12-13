//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, texture::DefaultImageSampler, RenderPlugin},
    winit::WinitPlugin,
};
use bevy_webxr::WebXrPlugin;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

fn main() {
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");

    let canvas = document.create_element("canvas").unwrap();
    canvas.set_id("test213");
    canvas.set_class_name("test213");
    canvas.set_node_value(Some("test213"));
    canvas.set_attribute("test213", "test213");

    let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
    canvas.set_height(200);
    canvas.set_width(200);
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&canvas);

    canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap()
        .make_xr_compatible();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(".test213".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WebXrPlugin::default())
        .add_systems(Startup, setup)
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
