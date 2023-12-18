use bevy::prelude::*;

//mod camera;
pub mod error;
pub mod events;
mod init;
mod session;

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
}

impl Default for WebXrSettings {
    fn default() -> WebXrSettings {
        WebXrSettings {
            vr_supported: true,
            ar_supported: true,
            inline_supported: false, // Not implemented yet.
            vr_button: "vr_button".to_string(),
            ar_button: "ar_button".to_string(),
            canvas: "canvas[data-raw-handle]".to_string(),
        }
    }
}

#[derive(Default)]
pub struct WebXrPlugin {
    pub settings: WebXrSettings,
}

impl Plugin for WebXrPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::WebXrSessionInitialized>();
        app.insert_resource(self.settings.clone());
        app.set_runner(init::webxr_runner);
    }
}
