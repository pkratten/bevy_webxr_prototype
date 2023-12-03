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
        let framebuffer_colour_attachment = create_view_from_device_framebuffer(
            render_device,
            framebuffer.clone(),
            &base_layer,
            wgpu::TextureFormat::Rgba8Unorm,
            "device framebuffer (colour)",
        );

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
                        drop_guard: None,
                    },
                    &wgpu::TextureDescriptor {
                        label: Some("color"),
                        size: wgpu::Extent3d {
                            width: base_layer.framebuffer_width(),
                            height: base_layer.framebuffer_height(),
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    },
                )
        };
    }
}
