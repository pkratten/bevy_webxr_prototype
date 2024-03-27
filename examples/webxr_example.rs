//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::f32::consts::PI;

use bevy::{
    log::LogPlugin,
    pbr::DefaultOpaqueRendererMethod,
    prelude::*,
    render::{
        camera::{ManualTextureViewHandle, ManualTextureViews},
        renderer::{RenderAdapter, RenderContext, RenderDevice, RenderQueue},
    },
    window::{PrimaryWindow, RawHandleWrapper},
    winit::WinitPlugin,
};
use bevy_webxr::{bTexture, error::WebXrError, WebXrPlugin, WebXrSettings};
use bevy_xr::{
    pointer::{LeftHanded, RightHanded},
    space::XrOrigin,
    XrActive, XrLocal,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::{RenderPass, RenderPassDescriptor};

fn main() {
    let mut app = App::new();
    app.add_plugins(LogPlugin::default());

    info!("{:?}", initialize_canvas("bevyxr"));

    // let web = raw_window_handle::WebWindowHandle::new(1);
    // let window_handle = raw_window_handle::RawWindowHandle::Web(web);
    // let display_handle =
    //     raw_window_handle::RawDisplayHandle::Web(raw_window_handle::WebDisplayHandle::new());

    // app.world.spawn((
    //     RawHandleWrapper {
    //         display_handle,
    //         window_handle,
    //     },
    //     PrimaryWindow,
    // ));

    app.insert_resource(Msaa::Off);
    app.insert_resource(DefaultOpaqueRendererMethod::deferred());

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("canvas[bevyxr=\"bevyxr\"]".to_string()),
                    ..default()
                }),
                ..default()
            })
            .disable::<LogPlugin>(), // .disable::<WinitPlugin>(),
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
    .add_systems(Update, test_framebuffer)
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
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(5.0, 5.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            ..default()
        }),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 6000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
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
        LinearRgba::GREEN,
    );

    gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, LinearRgba::RED);

    for y in [0., 0.5, 1.] {
        gizmos.ray(
            Vec3::new(1., y, 0.),
            Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
            LinearRgba::BLUE,
        );
    }

    // Circles have 32 line-segments by default.
    gizmos.circle(Vec3::ZERO, Dir3::Y, 3., LinearRgba::BLACK);
    // You may want to increase this for larger circles or spheres.
    gizmos
        .circle(Vec3::ZERO, Dir3::Y, 3.1, LinearRgba::BLUE)
        .segments(64);
    gizmos
        .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, LinearRgba::BLACK)
        .circle_segments(64);
}

fn rotate_camera(
    mut query: Query<&mut Transform, (With<Camera>, Without<XrLocal>)>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
    }
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
        canvas_element.set_attribute("alt", "App");
        canvas_element.set_attribute("data-raw-handle", "1");
        document
            .body()
            .ok_or(WebXrError::NoBody)?
            .append_child(&canvas_element)
            .map_err(|err| WebXrError::JsError(err))?;
        Ok(canvas_element)
    }
}

pub(crate) const FRAMEBUFFER_HANDLE: ManualTextureViewHandle = ManualTextureViewHandle(5724242);

fn test_framebuffer(
    texture_views: Res<ManualTextureViews>,
    render_device: Res<RenderDevice>,
    render_adapter: Res<RenderAdapter>,
    render_queue: Res<RenderQueue>,
    texture: Option<Res<bTexture>>,
) {
    if let Some(texture) = texture {
        if let Some(texture_view) = texture_views.get(&FRAMEBUFFER_HANDLE) {
            let mut ctx =
                RenderContext::new(render_device.clone(), render_adapter.get_info(), None);

            ctx.command_encoder()
                .begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view.texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            let test = ctx.finish();

            info!("Hi!");

            render_queue.submit(test.0);
        }
    }
}
