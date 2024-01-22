use bevy::{ecs::query::QuerySingleError, prelude::*};

use bevy_xr::{controller::*, tracked::XrTrackedObject};
use wasm_bindgen::JsCast;

use crate::{
    dom_point::{dom_point_to_quat, dom_point_to_vec3},
    WebXrFrame,
};

//Some assumptions about controller order are made...
pub fn update_xr_controllers(
    xr_frame: Option<NonSend<WebXrFrame>>,
    origin: Query<Entity, (With<XrOrigin>, With<XrLocal>, With<XrActive>)>,
    mut controller_left: Query<
        (Entity, &mut Transform, &mut XrActive),
        (
            With<XrController>,
            With<LeftHanded>,
            With<XrLocal>,
            Without<RightHanded>,
        ),
    >,
    mut controller_right: Query<
        (Entity, &mut Transform, &mut XrActive),
        (
            With<XrController>,
            With<RightHanded>,
            With<XrLocal>,
            Without<LeftHanded>,
        ),
    >,
    mut controllers: Query<
        (Entity, &mut Transform, &mut XrActive),
        (
            With<XrController>,
            With<XrLocal>,
            Without<LeftHanded>,
            Without<RightHanded>,
        ),
    >,

    mut controller_order: Local<Vec<Entity>>,
    mut commands: Commands,
) {
    if !origin.is_empty() {
        if let Some(frame) = xr_frame {
            let input_sources = frame.webxr_frame.session().input_sources();

            let no_left_controller = true;
            let no_right_controller = true;

            let current_controller_order = controller_order.clone();
            let mut controllers = controllers.iter_many_mut(current_controller_order.as_slice());

            for i in 0..input_sources.length() {
                if let Some(input_source) = input_sources.get(i) {
                    if let Some(_) = input_source.gamepad() {
                        if let Some(space) = input_source.grip_space() {
                            if let Some(pose) = frame
                                .webxr_frame
                                .get_pose(&space, frame.webxr_reference_space.dyn_ref().unwrap())
                            {
                                match input_source.handedness() {
                                    web_sys::XrHandedness::Left if no_left_controller => {
                                        match controller_left.get_single_mut() {
                                            Ok((entity, mut transform, mut active)) => {
                                                transform.translation =
                                                    dom_point_to_vec3(&pose.transform().position());
                                                transform.rotation = dom_point_to_quat(
                                                    &pose.transform().orientation(),
                                                );
                                                active.0 = true;
                                                commands
                                                    .entity(entity)
                                                    .insert(XrTrackedObject::LeftController);
                                            }
                                            Err(QuerySingleError::MultipleEntities(_)) => {
                                                let mut controllers = controller_left.iter_mut();
                                                let (entity, mut transform, mut active) =
                                                    controllers.next().unwrap();
                                                for (entity, _, _) in controllers {
                                                    commands.entity(entity).despawn();
                                                }

                                                transform.translation =
                                                    dom_point_to_vec3(&pose.transform().position());
                                                transform.rotation = dom_point_to_quat(
                                                    &pose.transform().orientation(),
                                                );
                                                active.0 = true;
                                                commands
                                                    .entity(entity)
                                                    .insert(XrTrackedObject::LeftController);
                                            }
                                            Err(QuerySingleError::NoEntities(_)) => {
                                                let entity = commands
                                                    .spawn(
                                                        XrControllerBundle::<LeftHanded>::default(),
                                                    )
                                                    .id();
                                                commands.entity(origin.single()).add_child(entity);
                                            }
                                        };
                                    }
                                    web_sys::XrHandedness::Right if no_right_controller => {
                                        match controller_right.get_single_mut() {
                                            Ok((entity, mut transform, mut active)) => {
                                                transform.translation =
                                                    dom_point_to_vec3(&pose.transform().position());
                                                transform.rotation = dom_point_to_quat(
                                                    &pose.transform().orientation(),
                                                );
                                                active.0 = true;
                                                commands
                                                    .entity(entity)
                                                    .insert(XrTrackedObject::RightController);
                                            }
                                            Err(QuerySingleError::MultipleEntities(_)) => {
                                                let mut controllers = controller_left.iter_mut();
                                                let (entity, mut transform, mut active) =
                                                    controllers.next().unwrap();
                                                for (entity, _, _) in controllers {
                                                    commands.entity(entity).despawn();
                                                }

                                                transform.translation =
                                                    dom_point_to_vec3(&pose.transform().position());
                                                transform.rotation = dom_point_to_quat(
                                                    &pose.transform().orientation(),
                                                );
                                                active.0 = true;
                                                commands
                                                    .entity(entity)
                                                    .insert(XrTrackedObject::RightController);
                                            }
                                            Err(QuerySingleError::NoEntities(_)) => {
                                                let entity = commands
                                                    .spawn(
                                                        XrControllerBundle::<RightHanded>::default(
                                                        ),
                                                    )
                                                    .id();
                                                commands.entity(origin.single()).add_child(entity);
                                            }
                                        };
                                    }
                                    web_sys::XrHandedness::__Nonexhaustive => {}
                                    _ => {
                                        if let Some((entity, mut transform, mut active)) =
                                            controllers.fetch_next()
                                        {
                                            transform.translation =
                                                dom_point_to_vec3(&pose.transform().position());
                                            transform.rotation =
                                                dom_point_to_quat(&pose.transform().orientation());
                                            active.0 = true;
                                            commands
                                                .entity(entity)
                                                .insert(XrTrackedObject::Other(i as usize));
                                        } else {
                                            let entity = commands
                                                .spawn(XrControllerHandlessBundle::default(
                                                    i as usize,
                                                ))
                                                .id();

                                            controller_order.push(entity);
                                            commands.entity(origin.single()).add_child(entity);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if no_left_controller {
                    for (_, _, mut active) in controller_left.iter_mut() {
                        active.0 = false;
                    }
                }

                if no_right_controller {
                    for (_, _, mut active) in controller_right.iter_mut() {
                        active.0 = false;
                    }
                }

                while let Some((_, _, mut active)) = controllers.fetch_next() {
                    active.0 = false;
                }

                return;
            }
        }
    }
    for (_, _, mut active) in controller_left.iter_mut() {
        active.0 = false;
    }

    for (_, _, mut active) in controller_right.iter_mut() {
        active.0 = false;
    }

    for (_, _, mut active) in controllers.iter_mut() {
        active.0 = false;
    }
}

fn update_xr_controller_input() {}
