use bevy::{
    prelude::*,
    render::{
        camera::{ManualTextureView, ManualTextureViewHandle, ManualTextureViews, Viewport},
        renderer::RenderDevice,
    },
};

use crate::WebXrFrame;

pub fn spawn_webxr_camera(
    xr_frame: NonSend<WebXrFrame>,
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_views: ResMut<ManualTextureViews>,
) {
    let session = xr_frame.webxr_frame.session();
    let base_layer = session.render_state().base_layer().unwrap();
    info!("{:?}", base_layer);
    let framebuffer: web_sys::WebGlFramebuffer =
        js_sys::Reflect::get(&base_layer, &"framebuffer".into())
            .unwrap()
            .into();

    let texture = unsafe {
        render_device
            .wgpu_device()
            .create_texture_from_hal::<wgpu_hal::gles::Api>(
                wgpu_hal::gles::Texture {
                    inner: wgpu_hal::gles::TextureInner::ExternalFramebuffer { inner: framebuffer },
                    mip_level_count: 1,
                    array_layer_count: 1,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    format_desc: wgpu_hal::gles::TextureFormatDesc {
                        internal: glow::RGBA,
                        external: glow::RGBA,
                        data_type: glow::UNSIGNED_BYTE,
                    },
                    copy_size: wgpu_hal::CopyExtent {
                        width: base_layer.framebuffer_width(),
                        height: base_layer.framebuffer_height(),
                        depth: 1,
                    },
                    drop_guard: None,
                    is_cubemap: false,
                },
                &wgpu::TextureDescriptor {
                    label: Some("framebuffer (color)"),
                    size: wgpu::Extent3d {
                        width: base_layer.framebuffer_width(),
                        height: base_layer.framebuffer_height(),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    // | wgpu::TextureUsages::COPY_SRC,
                    // | wgpu::TextureUsages::COPY_DST,
                },
            )
    };

    // let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
    //     label: Some(format!("color_texture").as_str()),
    //     format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
    //     dimension: Some(wgpu::TextureViewDimension::D2),
    //     aspect: wgpu::TextureAspect::All,
    //     base_mip_level: 0,
    //     mip_level_count: None,
    //     base_array_layer: 0,
    //     array_layer_count: Some(1),
    // });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let handle = ManualTextureViewHandle(0);

    texture_views.insert(
        handle,
        ManualTextureView::with_default_format(
            texture_view.into(),
            UVec2 {
                x: base_layer.framebuffer_width(),
                y: base_layer.framebuffer_height(),
            },
        ),
    );

    commands.spawn(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 { x: 0, y: 0 },
                physical_size: UVec2 {
                    x: base_layer.framebuffer_width() / 2,
                    y: base_layer.framebuffer_height(),
                },
                ..default()
            }),
            target: bevy::render::camera::RenderTarget::TextureView(handle),
            ..default()
        },
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: base_layer.framebuffer_width() / 2,
                    y: 0,
                },
                physical_size: UVec2 {
                    x: base_layer.framebuffer_width() / 2,
                    y: base_layer.framebuffer_height(),
                },
                ..default()
            }),
            target: bevy::render::camera::RenderTarget::TextureView(handle),
            ..default()
        },
        transform: Transform::from_xyz(2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn update_webxr_camera(
    xr_frame: NonSend<WebXrFrame>,
    render_device: Res<RenderDevice>,
    mut texture_views: ResMut<ManualTextureViews>,
) {
    let session = xr_frame.webxr_frame.session();
    let base_layer = session.render_state().base_layer().unwrap();
    let framebuffer: web_sys::WebGlFramebuffer =
        js_sys::Reflect::get(&base_layer, &"framebuffer".into())
            .unwrap()
            .into();

    let texture = unsafe {
        render_device
            .wgpu_device()
            .create_texture_from_hal::<wgpu_hal::gles::Api>(
                wgpu_hal::gles::Texture {
                    inner: wgpu_hal::gles::TextureInner::ExternalFramebuffer { inner: framebuffer },
                    mip_level_count: 1,
                    array_layer_count: 1,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    format_desc: wgpu_hal::gles::TextureFormatDesc {
                        internal: glow::RGBA,
                        external: glow::RGBA,
                        data_type: glow::UNSIGNED_BYTE,
                    },
                    copy_size: wgpu_hal::CopyExtent {
                        width: base_layer.framebuffer_width(),
                        height: base_layer.framebuffer_height(),
                        depth: 1,
                    },
                    drop_guard: None,
                    is_cubemap: false,
                },
                &wgpu::TextureDescriptor {
                    label: Some("framebuffer (color)"),
                    size: wgpu::Extent3d {
                        width: base_layer.framebuffer_width(),
                        height: base_layer.framebuffer_height(),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    // | wgpu::TextureUsages::COPY_SRC,
                    // | wgpu::TextureUsages::COPY_DST,
                },
            )
    };

    // let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
    //     label: Some(format!("color_texture").as_str()),
    //     format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
    //     dimension: Some(wgpu::TextureViewDimension::D2),
    //     aspect: wgpu::TextureAspect::All,
    //     base_mip_level: 0,
    //     mip_level_count: None,
    //     base_array_layer: 0,
    //     array_layer_count: Some(1),
    // });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let handle = ManualTextureViewHandle(0);

    texture_views.insert(
        handle,
        ManualTextureView::with_default_format(
            texture_view.into(),
            UVec2 {
                x: base_layer.framebuffer_width(),
                y: base_layer.framebuffer_height(),
            },
        ),
    );
}
