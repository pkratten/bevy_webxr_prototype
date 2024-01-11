use bevy::{
    prelude::*,
    render::{
        camera::{camera_system, CameraProjection, CameraProjectionPlugin},
        view::{update_frusta, VisibilitySystems},
    },
};
use bevy_xr::space::XrOrigin;
use projection::WebXrProjection;

pub mod error;
pub mod events;

mod dom_point;
mod init;
mod projection;
mod tracked;

#[derive(Clone, Copy)]
pub enum XrMode {
    VR,
    AR,
    Inline,
}

#[derive(Clone, Resource)]
pub struct WebXrSettings {
    pub vr_supported: bool,
    pub ar_supported: bool,
    pub inline_supported: bool,
    pub vr_button: String,
    pub ar_button: String,
    pub canvas: String,
    pub origin: XrOrigin,
}

impl Default for WebXrSettings {
    fn default() -> WebXrSettings {
        WebXrSettings {
            vr_supported: true,
            ar_supported: true,
            inline_supported: false, // Not implemented yet.
            vr_button: "vr_button".to_string(),
            ar_button: "ar_button".to_string(),
            canvas: "bevyxr".to_string(),
            origin: XrOrigin::Room,
        }
    }
}

#[derive(Default)]
pub struct WebXrPlugin {
    pub settings: WebXrSettings,
}

impl Plugin for WebXrPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraProjectionPlugin::<WebXrProjection>::default());
        app.insert_resource(self.settings.clone());
        app.set_runner(init::webxr_runner);

        events::add_events(app);

        app.add_systems(
            PreUpdate,
            (
                set_xr_mode,
                tracked::space::initialize_xr_space,
                tracked::camera::update_xr_cameras,
                print_projection_matrices,
            )
                .chain(),
        );

        app.add_systems(PostUpdate, update_frusta::<WebXrProjection>);
    }
}

fn print_projection_matrices(
    projections: Query<&Projection>,
    xr_projections: Query<&WebXrProjection>,
) {
    for projection in projections.iter() {
        info!("test");
        info!("{:?}", projection.get_projection_matrix());
    }

    for projection in xr_projections.iter() {
        info!("{:?}", projection.get_projection_matrix());
    }
}

pub struct WebXrFrame {
    pub time: f64,
    pub webxr_frame: web_sys::XrFrame,
    pub webxr_reference_space: web_sys::XrReferenceSpace,
}

fn set_xr_mode(mut event: EventReader<events::WebXrSessionInitialized>, mut commands: Commands) {
    for event in event.read() {
        commands.insert_resource(match event.mode {
            XrMode::VR => bevy_xr::XrMode::VR,
            XrMode::AR => bevy_xr::XrMode::AR,
            XrMode::Inline => bevy_xr::XrMode::None,
        })
    }
}
