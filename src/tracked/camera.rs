use bevy::{prelude::*, render::{
        camera::{ManualTextureView, ManualTextureViewHandle, ManualTextureViews, Viewport},
        renderer::RenderDevice,
    }
};
use bevy_xr::{
    handedness::{Handedness, LeftHanded, RightHanded}, head::XrEye, render::FlipView, space::XrOrigin, window::XrWindow, XrActive, XrLocal
};
use web_sys::XrView;
use wgpu::TextureUsages;

use crate::{
    dom_point::{dom_point_to_quat, dom_point_to_vec3},
    projection::WebXrProjection,
    WebXrFrame,
};

pub(crate) const FRAMEBUFFER_HANDLE: ManualTextureViewHandle = ManualTextureViewHandle(5724242);
const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

pub(crate) fn update_xr_cameras(
    xr_frame: Option<NonSend<WebXrFrame>>,
    origin: Query<Entity, (With<XrOrigin>, With<XrLocal>, With<XrActive>)>,
    mut eyes_left: Query<
        (
            Entity,
            &mut Transform,
            &mut Camera,
            &mut WebXrProjection,
            &mut XrActive,
        ),
        (
            With<XrEye>,
            With<LeftHanded>,
            With<XrLocal>,
            Without<RightHanded>,
            Without<XrWindow>,
        ),
    >,
    mut eyes_right: Query<
        (
            Entity,
            &mut Transform,
            &mut Camera,
            &mut WebXrProjection,
            &mut XrActive,
        ),
        (
            With<XrEye>,
            With<RightHanded>,
            With<XrLocal>,
            Without<LeftHanded>,
            Without<XrWindow>,
        ),
    >,
    mut windows: Query<
        (
            Entity,
            &mut Transform,
            &mut Camera,
            &mut WebXrProjection,
            &mut XrActive,
        ),
        (With<XrWindow>, With<XrLocal>, Without<XrEye>),
    >,
    render_device: Res<RenderDevice>,
    mut texture_views: ResMut<ManualTextureViews>,
    mut commands: Commands,
) {
    if !origin.is_empty() {
        if let Some(frame) = xr_frame {
            if let Some(viewer_pose) = frame
                .webxr_frame
                .get_viewer_pose(&frame.webxr_reference_space)
            {
                let views = viewer_pose.views();

                if views.length() > 0 {
                    if let Some(base_layer) =
                        frame.webxr_frame.session().render_state().base_layer()
                    {
                        info!("{:?}", base_layer);

                        if let Some(framebuffer) = base_layer.framebuffer()
                        {
                            //Update framebuffer:

                            info!("{:?}", framebuffer);

                            let texture = unsafe {
                                render_device
                                    .wgpu_device()
                                    .create_texture_from_hal::<wgpu_hal::gles::Api>(
                                        wgpu_hal::gles::Texture {
                                            inner:
                                                wgpu_hal::gles::TextureInner::ExternalFramebuffer {
                                                    inner: framebuffer,
                                                },
                                            mip_level_count: 1,
                                            array_layer_count: 1,
                                            format: TEXTURE_FORMAT,
                                            format_desc: wgpu_hal::gles::TextureFormatDesc {
                                                internal: glow::RGBA, // TODO: Test alternatives.
                                                external: glow::RGBA,
                                                data_type: glow::UNSIGNED_BYTE,
                                            },
                                            copy_size: wgpu_hal::CopyExtent {
                                                width: base_layer.framebuffer_width(),
                                                height: base_layer.framebuffer_height(),
                                                depth: 1,
                                            },
                                            drop_guard: None,
                                        },
                                        &wgpu::TextureDescriptor {
                                            label: Some("webxr framebuffer (color)"),
                                            size: wgpu::Extent3d {
                                                width: base_layer.framebuffer_width(),
                                                height: base_layer.framebuffer_height(),
                                                depth_or_array_layers: 1,
                                            },
                                            mip_level_count: 1,
                                            sample_count: 1,
                                            dimension: wgpu::TextureDimension::D2,
                                            format: TEXTURE_FORMAT,
                                            view_formats: &[],
                                            usage: TextureUsages::RENDER_ATTACHMENT
                                                | TextureUsages::TEXTURE_BINDING
                                                | TextureUsages::COPY_SRC,
                                        },
                                    )
                            };

                            let texture_view =
                                texture.create_view(&wgpu::TextureViewDescriptor::default());

                            texture_views.insert(
                                FRAMEBUFFER_HANDLE,
                                ManualTextureView::with_default_format(
                                    texture_view.into(),
                                    UVec2 {
                                        x: base_layer.framebuffer_width(),
                                        y: base_layer.framebuffer_height(),
                                    },
                                ),
                            );

                            let views: Vec<XrView> = views.iter().map(|view| view.into()).collect();

                            let mut eyes_left = eyes_left.iter_mut();
                            let mut eyes_right = eyes_right.iter_mut();
                            let mut windows = windows.iter_mut();

                            let mut eye_left_index = 0;
                            let mut eye_right_index = 0;
                            let mut window_index = 0;

                            for (i, view) in views.iter().enumerate() {
                                let viewport = base_layer.get_viewport(view).unwrap();
                                match view.eye() {
                                    web_sys::XrEye::Left => {
                                        if let Some((
                                            _,
                                            mut transform,
                                            mut camera,
                                            mut projection,
                                            mut active,
                                        )) = eyes_left.next()
                                        {
                                            transform.translation =
                                                dom_point_to_vec3(&view.transform().position());
                                            transform.rotation =
                                                dom_point_to_quat(&view.transform().orientation());
                                            camera.viewport = Some(Viewport {
                                                physical_position: UVec2 {
                                                    x: viewport.x() as u32,
                                                    y: viewport.y() as u32,
                                                },
                                                physical_size: UVec2 {
                                                    x: viewport.width() as u32,
                                                    y: viewport.height() as u32,
                                                },
                                                ..default()
                                            });
                                            camera.is_active = true;
                                            active.0 = true;
                                            projection.update_matrix(view.projection_matrix());
                                        } else {
                                            let mut eye = commands
                                                .spawn((
                                                    Camera3dBundle {
                                                        // TODO: What is msaa_writeback?
                                                        camera: Camera {
                                                            viewport: Some(Viewport{
                                                                physical_position: UVec2 { x: viewport.x() as u32, y: viewport.y() as u32 },
                                                                physical_size: UVec2 { x: viewport.width() as u32, y: viewport.height() as u32},
                                                                ..default()
                                                            }),
                                                            target: bevy::render::camera::RenderTarget::TextureView(FRAMEBUFFER_HANDLE),
                                                            //clear_color: ClearColorConfig::Custom(Color::NONE),
                                                            order: i as isize,
                                                            
                                                            ..default()
                                                        },
                                                        transform: Transform {
                                                        translation: dom_point_to_vec3(
                                                            &view.transform().position(),
                                                        ),
                                                        rotation: dom_point_to_quat(
                                                            &view.transform().orientation(),
                                                        ),
                                                        ..Default::default()
                                                    },
                                                        ..default()
                                                    },
                                                    WebXrProjection::from(view.projection_matrix()),
                                                    //PostProcessFlipY,
                                                    XrEye(eye_left_index),
                                                    LeftHanded,
                                                    Handedness::Left,
                                                    XrLocal,
                                                    XrActive(true),
                                                ));
                                            eye.remove::<Projection>();
                                            eye.log_components();
                                            let eye = eye.id();

                                            commands.entity(origin.single()).add_child(eye);
                                        }
                                        eye_left_index += 1;
                                    }
                                    web_sys::XrEye::Right => {
                                        if let Some((
                                            _,
                                            mut transform,
                                            mut camera,
                                            mut projection,
                                            mut active,
                                        )) = eyes_right.next()
                                        {
                                            transform.translation =
                                                dom_point_to_vec3(&view.transform().position());
                                            transform.rotation =
                                                dom_point_to_quat(&view.transform().orientation());
                                            camera.viewport = Some(Viewport {
                                                physical_position: UVec2 {
                                                    x: viewport.x() as u32,
                                                    y: viewport.y() as u32,
                                                },
                                                physical_size: UVec2 {
                                                    x: viewport.width() as u32,
                                                    y: viewport.height() as u32,
                                                },
                                                ..default()
                                            });
                                            camera.is_active = true;
                                            active.0 = true;
                                            projection.update_matrix(view.projection_matrix());
                                        } else {
                                            let mut eye = commands
                                                .spawn((
                                                    Camera3dBundle {
                                                        // TODO: What is msaa_writeback?
                                                        camera: Camera {
                                                            viewport: Some(Viewport{
                                                                physical_position: UVec2 { x: viewport.x() as u32, y: viewport.y() as u32 },
                                                                physical_size: UVec2 { x: viewport.width() as u32, y: viewport.height() as u32},
                                                                ..default()
                                                            }),
                                                            target: bevy::render::camera::RenderTarget::TextureView(FRAMEBUFFER_HANDLE),
                                                            //clear_color: ClearColorConfig::None,
                                                            order: i as isize,
                                                            ..default()
                                                        },
                                                        transform: Transform {
                                                        translation: dom_point_to_vec3(
                                                            &view.transform().position(),
                                                        ),
                                                        rotation: dom_point_to_quat(
                                                            &view.transform().orientation(),
                                                        ),
                                                        ..Default::default()
                                                    },
                                                        ..default()
                                                    },
                                                    WebXrProjection::from(view.projection_matrix()),
                                                    FlipView::Y,
                                                    XrEye(eye_right_index),
                                                    RightHanded,
                                                    Handedness::Right,
                                                    XrLocal,
                                                    XrActive(true),
                                                ));
                                            eye.remove::<Projection>();
                                            eye.log_components();
                                            let eye = eye.id();

                                            commands.entity(origin.single()).add_child(eye);
                                        }
                                        eye_right_index += 1;
                                    }
                                    web_sys::XrEye::None => {
                                        if let Some((
                                            _,
                                            mut transform,
                                            mut camera,
                                            mut projection,
                                            mut active,
                                        )) = windows.next()
                                        {
                                            transform.translation =
                                                dom_point_to_vec3(&view.transform().position());
                                            transform.rotation =
                                                dom_point_to_quat(&view.transform().orientation());
                                            camera.viewport = Some(Viewport {
                                                physical_position: UVec2 {
                                                    x: viewport.x() as u32,
                                                    y: viewport.y() as u32,
                                                },
                                                physical_size: UVec2 {
                                                    x: viewport.width() as u32,
                                                    y: viewport.height() as u32,
                                                },
                                                ..default()
                                            });
                                            camera.is_active = true;
                                            active.0 = true;
                                            projection.update_matrix(view.projection_matrix());
                                        } else {
                                            let mut window = commands
                                                .spawn((
                                                    Camera3dBundle {
                                                        // TODO: What is msaa_writeback?
                                                        camera: Camera {
                                                            viewport: Some(Viewport{
                                                                physical_position: UVec2 { x: viewport.x() as u32, y: viewport.y() as u32 },
                                                                physical_size: UVec2 { x: viewport.width() as u32, y: viewport.height() as u32},
                                                                ..default()
                                                            }),
                                                            target: bevy::render::camera::RenderTarget::TextureView(FRAMEBUFFER_HANDLE),
                                                            //clear_color: ClearColorConfig::Custom(Color::NONE),
                                                            order: i as isize,
                                                            
                                                            ..default()
                                                        },
                                                        transform: Transform {
                                                        translation: dom_point_to_vec3(
                                                            &view.transform().position(),
                                                        ),
                                                        rotation: dom_point_to_quat(
                                                            &view.transform().orientation(),
                                                        ),
                                                        ..Default::default()
                                                    },
                                                        ..default()
                                                    },
                                                    WebXrProjection::from(view.projection_matrix()),
                                                    FlipView::Y,
                                                    XrWindow(window_index),
                                                    XrLocal,
                                                    XrActive(true),
                                                ));
                                            window.remove::<Projection>();
                                            window.log_components();
                                            let window = window.id();

                                            commands.entity(origin.single()).add_child(window);
                                        }
                                        window_index += 1;
                                    }
                                    web_sys::XrEye::__Nonexhaustive => {}
                                }
                            }

                            for (_, _, mut camera, _, mut active) in eyes_left {
                                camera.is_active = false;
                                active.0 = false;
                            }

                            for (_, _, mut camera, _, mut active) in eyes_right {
                                camera.is_active = false;
                                active.0 = false;
                            }

                            for (_, _, mut camera, _, mut active) in windows {
                                camera.is_active = false;
                                active.0 = false;
                            }

                            return;
                        } else {
                            warn!("Failed to get webxr framebuffer!");
                        }
                    } else {
                        warn!("Failed to get webxr render state base layer!");
                    }
                } else {
                    warn!("The amount of webxr views is 0!");
                }
            } else {
                warn!("Failed to get webxr viewer pose!");
            }
        } else {
            {
                warn!("Failed to get WebXrFrame!");
            }
        }
    } else {
        warn!("No XrOrigin!");
    }

    //There was a problem updating cameras that has been logged as a warning. Disable all cameras:
    for (_, _, mut camera, _, mut active) in eyes_left.iter_mut() {
        camera.is_active = false;
        active.0 = false;
    }

    for (_, _, mut camera, _, mut active) in eyes_right.iter_mut() {
        camera.is_active = false;
        active.0 = false;
    }

    for (_, _, mut camera, _, mut active) in windows.iter_mut() {
        camera.is_active = false;
        active.0 = false;
    }
}
