use bevy::{ecs::query::QuerySingleError, prelude::*};

use bevy_xr::{
    controller::*,
    controller_input::{
        DigitalInputState, XrControllerAxis, XrControllerAxisChangedEvent, XrControllerAxisType,
        XrControllerEvent, XrControllerInfo, XrControllerInputType, XrControllerPressChangedEvent,
        XrControllerSettings, XrControllerState, XrControllerStateChangedEvent,
        XrControllerTouchChangedEvent, XrControllerTouchInput,
    },
    tracked::XrTrackedObject,
};
use wasm_bindgen::JsCast;
use web_sys::{Gamepad, GamepadButton};

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

    mut xr_controller_events: Event<XrControllerEvent>,
    mut analog_touch: ResMut<AnalogInput<XrControllerTouch>>,
    mut analog_press: ResMut<AnalogInput<XrControllerPress>>,
    mut analog_axes: ResMut<AnalogInput<XrControllerAxis>>,
    settings: Res<XrControllerSettings>,

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
                    if let Some(gamepad) = input_source.gamepad() {
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
                                                    .insert(XrTrackedObject(i as u8));
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
                                                    .insert(XrTrackedObject(i as u8));
                                            }
                                            Err(QuerySingleError::NoEntities(_)) => {
                                                let entity = commands
                                                    .spawn(
                                                        XrControllerBundle::<LeftHanded>::default(
                                                            i as u8,
                                                        ),
                                                    )
                                                    .id();
                                                commands.entity(origin.single()).add_child(entity);

                                                xr_controller_events.send(
                                                    XrControllerStateChangedEvent::new(
                                                        XrController::Left,
                                                        XrControllerState::Tracking(
                                                            XrControllerInfo {
                                                                name: "Xr Controller Left"
                                                                    .to_string(),
                                                            },
                                                        ),
                                                    )
                                                    .into(),
                                                );
                                            }
                                        };
                                        handle_input(
                                            XrController::Left,
                                            gamepad,
                                            xr_controller_events,
                                            analog_touch,
                                            analog_press,
                                            analog_axes,
                                            settings,
                                        )
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
                                                    .insert(XrTrackedObject(i as u8));
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
                                                    .insert(XrTrackedObject(i as u8));
                                            }
                                            Err(QuerySingleError::NoEntities(_)) => {
                                                let entity = commands
                                                    .spawn(
                                                        XrControllerBundle::<RightHanded>::default(
                                                            i as u8,
                                                        ),
                                                    )
                                                    .id();
                                                commands.entity(origin.single()).add_child(entity);

                                                xr_controller_events.send(
                                                    XrControllerStateChangedEvent::new(
                                                        XrController::Right,
                                                        XrControllerState::Tracking(
                                                            XrControllerInfo {
                                                                name: "Xr Controller Right"
                                                                    .to_string(),
                                                            },
                                                        ),
                                                    )
                                                    .into(),
                                                );
                                            }
                                        };
                                        handle_input(
                                            XrController::Right,
                                            gamepad,
                                            xr_controller_events,
                                            analog_touch,
                                            analog_press,
                                            analog_axes,
                                            settings,
                                        )
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
                                                .insert(XrTrackedObject(i as u8));
                                        } else {
                                            let entity = commands
                                                .spawn(XrControllerHandlessBundle::default(i))
                                                .id();

                                            controller_order.push(entity);
                                            commands.entity(origin.single()).add_child(entity);

                                            xr_controller_events.send(
                                                XrControllerStateChangedEvent::new(
                                                    XrController::Other(i),
                                                    XrControllerState::Tracking(XrControllerInfo {
                                                        name: "Xr Controller ".to_string()
                                                            + &i.to_string(),
                                                    }),
                                                )
                                                .into(),
                                            );
                                        }
                                        handle_input(
                                            XrController::Other(i),
                                            gamepad,
                                            xr_controller_events,
                                            analog_touch,
                                            analog_press,
                                            analog_axes,
                                            settings,
                                        );
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

fn index_to_input_type(i: u32) -> XrControllerInputType {
    match i {
        0 => XrControllerInputType::Trigger,
        1 => XrControllerInputType::Grip,
        2 => XrControllerInputType::Pad,
        3 => XrControllerInputType::Stick,
        4 => XrControllerInputType::AorX,
        5 => XrControllerInputType::BorY,
        i => XrControllerInputType::Other(i),
    }
}

fn index_to_axis_type(i: u32) -> XrControllerAxisType {
    match i {
        0 => XrControllerAxisType::PadX,
        1 => XrControllerAxisType::PadY,
        2 => XrControllerAxisType::StickX,
        3 => XrControllerAxisType::StickY,
        i => XrControllerAxisType::Other(i),
    }
}

fn handle_input(
    xr_controller: XrController,
    gamepad: Gamepad,
    mut xr_controller_events: Event<XrControllerEvent>,
    mut analog_touch: ResMut<AnalogInput<XrControllerTouch>>,
    mut analog_press: ResMut<AnalogInput<XrControllerPress>>,
    mut analog_axes: ResMut<AnalogInput<XrControllerAxis>>,
    settings: Res<XrControllerSettings>,
) {
    let buttons = gamepad.buttons();
    for i in 0..buttons.length() {
        if let Ok(button) = buttons.get(i).dyn_into::<GamepadButton>() {
            input_type = index_to_input_type(i);
            new_value = button.value();

            let touch = XrControllerTouch::new(xr_controller, input_type);
            let old_value = analog_touch.get(touch);
            let touch_settings = settings.get_touch_axis_settings(touch);
            // Only send events that pass the user-defined change threshold
            if let Some(filtered_value) = touch_settings.filter(new_value, old_value) {
                xr_controller_events.send(
                    XrControllerTouchChangedEvent::new(xr_controller, touch, filtered_value).into(),
                );
                // Update the current value prematurely so that `old_value` is correct in
                // future iterations of the loop.
                analog_touch.set(button, filtered_value);
            }

            let press = XrControllerPress::new(xr_controller, input_type);
            let old_value = analog_press.get(press);
            let press_settings = settings.get_press_axis_settings(press);
            // Only send events that pass the user-defined change threshold
            if let Some(filtered_value) = press_settings.filter(new_value, old_value) {
                xr_controller_events.send(
                    XrControllerPressChangedEvent::new(xr_controller, press, filtered_value).into(),
                );
                // Update the current value prematurely so that `old_value` is correct in
                // future iterations of the loop.
                analog_press.set(button, filtered_value);
            }
        }
    }

    let axes = gamepad.axes();
    for i in 0..axes.length() {
        axis_type = index_to_axis_type(i);
        new_value = axes.get(i).dyn_into();

        let axis = XrControllerAxis::new(xr_controller, axis_type);
        let old_value = analog_axes.get(axis);
        let axis_settings = settings.get_axis_settings(axis);

        // Only send events that pass the user-defined change threshold
        if let Some(filtered_value) = axis_settings.filter(new_value, old_value) {
            events.send(
                XrControllerAxisChangedEvent::new(xr_controller, axis_type, filtered_value).into(),
            );
        }
    }
}
