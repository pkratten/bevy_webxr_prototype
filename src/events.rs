use bevy::prelude::*;

use crate::XrMode;

pub(crate) fn add_events(app: &mut App) {
    app.add_event::<WebXrSessionInitialized>();
}

#[derive(Event)]
pub struct WebXrSessionInitialized(pub XrMode);
