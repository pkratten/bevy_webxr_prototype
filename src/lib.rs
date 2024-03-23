use bevy::{
    input::InputSystem,
    prelude::*,
    render::{
        camera::{camera_system, CameraProjectionPlugin},
        view::{update_frusta, VisibilitySystems},
    },
    transform::TransformSystem,
};
use bevy_xr::{
    pointer::{LeftHanded, RightHanded},
    space::XrOrigin,
};
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
            canvas: "canvas[alt=\"App\"]".to_string(),
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
        app.add_plugins(bevy_xr::render::FlipViewPlugin);

        app.add_plugins(bevy_xr::controller_input::XrControllerInputPlugin);

        app.insert_resource(self.settings.clone());
        app.set_runner(init::webxr_runner);

        events::add_events(app);

        app.add_systems(
            PreUpdate,
            (
                set_xr_mode,
                tracked::space::initialize_xr_space,
                tracked::camera::update_xr_cameras,
                tracked::controllers::update_xr_controllers.before(InputSystem),
                tracked::hands::update_xr_hands::<LeftHanded>.in_set(InputSystem),
                tracked::hands::update_xr_hands::<RightHanded>.in_set(InputSystem),
                bevy_xr::systems::substitute_local_palm::<LeftHanded>.in_set(InputSystem),
                bevy_xr::systems::substitute_local_palm::<RightHanded>.in_set(InputSystem),
            )
                .chain(),
        );

        app.add_systems(
            PostUpdate,
            update_frusta::<projection::WebXrProjection>
                .after(VisibilitySystems::UpdatePerspectiveFrusta)
                .after(camera_system::<projection::WebXrProjection>)
                .after(TransformSystem::TransformPropagate)
                .ambiguous_with(update_frusta::<Projection>),
        );
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
