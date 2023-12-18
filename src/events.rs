use bevy::prelude::*;

use crate::XrMode;

#[derive(Event)]
pub struct WebXrSessionInitialized(pub XrMode);
