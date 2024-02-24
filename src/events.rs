use bevy::prelude::*;
use bevy_xr::space::XrOrigin;

use crate::XrMode;

pub(crate) fn add_events(app: &mut App) {
    app.add_event::<WebXrSessionInitialized>();
}

#[derive(Event)]
pub struct WebXrSessionInitialized {
    pub mode: XrMode,
    pub origin: XrOrigin,
}
