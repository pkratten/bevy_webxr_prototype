use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_xr::{space::XrOrigin, XrActive, XrLocal};

use crate::events::WebXrSessionInitialized;

pub fn initialize_xr_space(
    mut event: EventReader<WebXrSessionInitialized>,
    mut origin: Query<(Entity, &mut XrActive), (With<XrOrigin>, With<XrLocal>)>,
    mut commands: Commands,
) {
    for event in event.iter() {
        match origin.get_single_mut() {
            Ok((entity, mut active)) => {
                commands.entity(entity).insert(event.origin);
                active.0 = true;
            }
            Err(err) => {
                if let QuerySingleError::MultipleEntities(_) = err {
                    for (entity, _) in origin.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                commands
                    .spawn((
                        SpatialBundle::default(),
                        event.origin,
                        XrLocal,
                        XrActive(true),
                    ))
                    .log_components();
            }
        }
    }
}
