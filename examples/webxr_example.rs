//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::f32::consts::PI;

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{
        camera::CameraProjection, settings::WgpuSettings, texture::DefaultImageSampler,
        RenderPlugin,
    },
    winit::WinitPlugin,
};
use bevy_webxr::{error::WebXrError, WebXrPlugin, WebXrSettings};
use bevy_xr::{
    pointer::{LeftHanded, RightHanded},
    space::XrOrigin,
    XrActive, XrLocal,
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
            .disable::<LogPlugin>(),
    );

    app.add_plugins(WebXrPlugin {
        settings: WebXrSettings {
            canvas: "canvas[bevyxr=\"bevyxr\"]".to_string(),
            ..default()
        },
    })
    .add_systems(Startup, setup)
    .add_systems(Update, (gizmos, rotate_camera))
    .add_systems(Update, bevy_xr::systems::draw_hand_gizmos)
    .add_systems(
        PreUpdate,
        bevy_xr::systems::substitute_local_palm::<LeftHanded>,
    )
    .add_systems(
        PreUpdate,
        bevy_xr::systems::substitute_local_palm::<RightHanded>,
    )
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            double_sided: true,
            cull_mode: Some(wgpu::Face::Front),
            ..default()
        }),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            double_sided: true,
            cull_mode: Some(wgpu::Face::Front),
            ..default()
        }),
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
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, -8.0, -4.0),
        ..default()
    });

    // example instructions
    commands.spawn(
        TextBundle::from_section(
            "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
            Press 'P' to toggle perspective for line gizmos\n\
            Hold 'Left' or 'Right' to change the line width",
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).with_scale(Vec3::splat(20.0)),
            ..default()
        },
        XrOrigin::Other,
        XrLocal,
        XrActive(true),
    ));
}

fn gizmos(mut gizmos: Gizmos, time: Res<Time>) {
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.)),
        Color::BLACK,
    );
    gizmos.rect(
        Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
        Quat::from_rotation_y(PI / 2.),
        Vec2::splat(2.),
        Color::GREEN,
    );

    gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);

    for y in [0., 0.5, 1.] {
        gizmos.ray(
            Vec3::new(1., y, 0.),
            Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
            Color::BLUE,
        );
    }

    // Circles have 32 line-segments by default.
    gizmos.circle(Vec3::ZERO, Vec3::Y, 3., Color::BLACK);
    // You may want to increase this for larger circles or spheres.
    gizmos
        .circle(Vec3::ZERO, Vec3::Y, 3.1, Color::NAVY)
        .segments(64);
    gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
        .circle_segments(64);
}

fn rotate_camera(
    mut query: Query<&mut Transform, (With<Camera>, Without<XrLocal>)>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
}

pub fn initialize_canvas(canvas: &str) -> Result<web_sys::HtmlCanvasElement, WebXrError> {
    let window = web_sys::window().ok_or(WebXrError::NoWindow)?;
    let document = window.document().ok_or(WebXrError::NoDocument)?;

    if let Ok(Some(canvas_element)) = document.query_selector(canvas) {
        let canvas_element = canvas_element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|err| WebXrError::ElementNotCanvasElement(err))?;
        Ok(canvas_element)
    } else {
        let element = document
            .create_element("canvas")
            .map_err(|err| WebXrError::JsError(err))?;
        let canvas_element = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|err| WebXrError::ElementNotCanvasElement(err))?;
        canvas_element.set_id(canvas);
        canvas_element.set_attribute(canvas, canvas).unwrap();
        document
            .body()
            .ok_or(WebXrError::NoBody)?
            .append_child(&canvas_element)
            .map_err(|err| WebXrError::JsError(err))?;
        Ok(canvas_element)
    }
}
