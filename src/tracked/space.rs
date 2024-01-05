use bevy::prelude::*;
use bevy_xr::space::XrOrigin;
use web_sys::XrReferenceSpaceType;

use crate::{events::WebXrSessionInitialized, WebXrFrame, XrMode};

fn initialize_xr_space(
    mut event: EventReader<WebXrSessionInitialized>,
    frame: Option<NonSend<WebXrFrame>>,
    origin: Query<&mut XrOrigin>,
) {
    for event in event.read() {
        if let Some(frame) = frame {}
    }
}
