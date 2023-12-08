use bevy::{prelude::*, render::renderer::RenderDevice};

use crate::WebXrContext;

pub(crate) fn spawn_webxr_camera(
    mut commands: Commands,
    webxr_context: NonSend<WebXrContext>,
    render_device: Res<RenderDevice>,
) {
    if let Some(Ok(session)) = &*webxr_context.session.clone().borrow() {
        let base_layer = session.render_state().base_layer().unwrap();
        let framebuffer: web_sys::WebGlFramebuffer =
            js_sys::Reflect::get(&base_layer, &"framebuffer".into())
                .unwrap()
                .into();
        // let framebuffer_colour_attachment = create_view_from_device_framebuffer(
        //     render_device,
        //     framebuffer.clone(),
        //     &base_layer,
        //     wgpu::TextureFormat::Rgba8Unorm,
        //     "device framebuffer (colour)",
        // );

        let texture = unsafe {
            render_device
                .wgpu_device()
                .create_texture_from_hal::<wgpu_hal::gles::Api>(
                    wgpu_hal::gles::Texture {
                        inner: wgpu_hal::gles::TextureInner::ExternalFramebuffer {
                            inner: framebuffer,
                        },
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
                        is_cubemap: false,
                        drop_guard: None,
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
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_formats: &[],
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::TEXTURE_BINDING,
                        // | wgpu::TextureUsages::COPY_SRC,
                        // | wgpu::TextureUsages::COPY_DST,
                    },
                )
        };

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(format!("color_texture").as_str()),
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: Some(1),
        });

        // let texture = unsafe {
        //     render_device
        //         .wgpu_device()
        //         .create_texture_from_hal::<wgpu_hal::gles::Api>(
        //             wgpu_hal::gles::Texture {
        //                 inner: wgpu_hal::gles::TextureInner::ExternalFramebuffer {
        //                     inner: framebuffer,
        //                 },
        //                 mip_level_count: 1,
        //                 array_layer_count: 1,
        //                 format: wgpu::TextureFormat::Rgba8Unorm,
        //                 format_desc: wgpu_hal::gles::TextureFormatDesc {
        //                     internal: glow::RGBA,
        //                     external: glow::RGBA,
        //                     data_type: glow::UNSIGNED_BYTE,
        //                 },
        //                 copy_size: wgpu_hal::CopyExtent {
        //                     width: base_layer.framebuffer_width(),
        //                     height: base_layer.framebuffer_height(),
        //                     depth: 1,
        //                 },
        //                 is_cubemap: false,
        //                 drop_guard: None,
        //             },
        //             &wgpu::TextureDescriptor {
        //                 label: Some("color"),
        //                 size: wgpu::Extent3d {
        //                     width: base_layer.framebuffer_width(),
        //                     height: base_layer.framebuffer_height(),
        //                     depth_or_array_layers: 1,
        //                 },
        //                 mip_level_count: 1,
        //                 sample_count: 1,
        //                 dimension: wgpu::TextureDimension::D2,
        //                 format: wgpu::TextureFormat::Rgba8Unorm,
        //                 usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //                 view_formats: &[],
        //             },
        //         )
        // };
    }
}
