use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_xr::{
    hands::{finger::*, finger_joint::*, hand_joint::*, Hand, *},
    IntoEnum,
};
use wasm_bindgen::JsCast;
use web_sys::{XrFrame, XrHand as WebXrHand, XrHandJoint, XrHandedness, XrJointSpace};

use crate::{
    dom_point::{dom_point_to_quat, dom_point_to_vec3},
    WebXrFrame,
};

pub fn update_xr_hands<Handedness: HandednessMarker + WebXrHandedness>(
    xr_frame: Option<NonSend<WebXrFrame>>,
    origin: Query<Entity, (With<XrOrigin>, With<XrLocal>, With<XrActive>)>,

    mut wrist: Query<
        (Entity, &mut Transform, &mut XrActive),
        (With<XrLocal>, With<Wrist>, With<Handedness>),
    >,
    thumb: (
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Thumb>,
                With<Metacarpal>,
                With<Handedness>,
                Without<Wrist>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Thumb>,
                With<ProximalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Thumb>,
                With<DistalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Thumb>,
                With<Tip>,
                With<Handedness>,
                Without<Wrist>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
            ),
        >,
    ),
    index: (
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Index>,
                With<Metacarpal>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Index>,
                With<ProximalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Index>,
                With<IntermediatePhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Index>,
                With<DistalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Index>,
                With<Tip>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Middle>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
            ),
        >,
    ),
    middle: (
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Middle>,
                With<Metacarpal>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Ring>,
                Without<Little>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Middle>,
                With<ProximalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Middle>,
                With<IntermediatePhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Middle>,
                With<DistalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Middle>,
                With<Tip>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Ring>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
            ),
        >,
    ),
    ring: (
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Ring>,
                With<Metacarpal>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Little>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Ring>,
                With<ProximalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Little>,
                Without<Metacarpal>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Ring>,
                With<IntermediatePhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Ring>,
                With<DistalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Ring>,
                With<Tip>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Little>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
            ),
        >,
    ),
    little: (
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Little>,
                With<Metacarpal>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Little>,
                With<ProximalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Metacarpal>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Little>,
                With<IntermediatePhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<DistalPhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Little>,
                With<DistalPhalanx>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<Tip>,
            ),
        >,
        Query<
            (Entity, &mut Transform, &mut XrActive),
            (
                With<XrLocal>,
                With<Little>,
                With<Tip>,
                With<Handedness>,
                Without<Wrist>,
                Without<Thumb>,
                Without<Index>,
                Without<Middle>,
                Without<Ring>,
                Without<Metacarpal>,
                Without<ProximalPhalanx>,
                Without<IntermediatePhalanx>,
                Without<DistalPhalanx>,
            ),
        >,
    ),

    mut commands: Commands,
) {
    if !origin.is_empty() {
        if let Some(frame) = xr_frame {
            let input_sources = frame.webxr_frame.session().input_sources();

            for i in 0..input_sources.length() {
                if let Some(input_source) = input_sources.get(i) {
                    if input_source.handedness() == Handedness::webxr_handedness() {
                        if let Some(hand) = input_source.hand() {
                            //Wrist

                            let joint_space = hand.get(XrHandJoint::Wrist);

                            if let Some(joint_pose) = frame.webxr_frame.get_joint_pose(
                                &joint_space,
                                frame.webxr_reference_space.dyn_ref().unwrap(),
                            ) {
                                let joint = match wrist.get_single_mut() {
                                    Ok((entity, mut transform, mut active)) => {
                                        transform.translation =
                                            dom_point_to_vec3(&joint_pose.transform().position());
                                        transform.rotation = dom_point_to_quat(
                                            &joint_pose.transform().orientation(),
                                        );
                                        active.0 = true;
                                        Some((entity, joint_space))
                                    }
                                    Err(QuerySingleError::MultipleEntities(_)) => {
                                        let mut wrist = wrist.iter();
                                        let (entity, _, _) = wrist.next().unwrap();
                                        for (entity, _, _) in wrist {
                                            commands.entity(entity).despawn();
                                        }
                                        Some((entity, joint_space))
                                    }
                                    Err(QuerySingleError::NoEntities(_)) => {
                                        let mut entity =
                                            commands.spawn(HandJointBundle::<Handedness, Wrist> {
                                                spatial_bundle: SpatialBundle {
                                                    transform: Transform {
                                                        translation: dom_point_to_vec3(
                                                            &joint_pose.transform().position(),
                                                        ),
                                                        rotation: dom_point_to_quat(
                                                            &joint_pose.transform().orientation(),
                                                        ),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                ..default()
                                            });
                                        entity.log_components();
                                        let entity = entity.id();

                                        commands.entity(origin.single()).add_child(entity);

                                        Some((entity, joint_space))
                                    }
                                };

                                let mut finger = thumb;
                                {
                                    let joint = update_xr_finger_joint(
                                        &mut finger.0,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.1,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.2,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    update_xr_finger_joint(
                                        &mut finger.3,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                }

                                let mut finger = index;
                                {
                                    let joint = update_xr_finger_joint(
                                        &mut finger.0,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.1,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.2,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.3,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    update_xr_finger_joint(
                                        &mut finger.4,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                }

                                let mut finger = middle;
                                {
                                    let joint = update_xr_finger_joint(
                                        &mut finger.0,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.1,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.2,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.3,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    update_xr_finger_joint(
                                        &mut finger.4,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                }

                                let mut finger = ring;
                                {
                                    let joint = update_xr_finger_joint(
                                        &mut finger.0,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.1,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.2,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.3,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    update_xr_finger_joint(
                                        &mut finger.4,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                }

                                let mut finger = little;
                                {
                                    let joint = update_xr_finger_joint(
                                        &mut finger.0,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.1,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.2,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    let joint = update_xr_finger_joint(
                                        &mut finger.3,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                    update_xr_finger_joint(
                                        &mut finger.4,
                                        &joint,
                                        &hand,
                                        &frame.webxr_frame,
                                        &mut commands,
                                    );
                                }

                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    for (_, _, mut active) in wrist.iter_mut() {
        active.0 = false;
    }

    let mut finger = thumb;
    {
        for (_, _, mut active) in finger.0.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.1.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.2.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.3.iter_mut() {
            active.0 = false;
        }
    }

    let mut finger = index;
    {
        for (_, _, mut active) in finger.0.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.1.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.2.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.3.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.4.iter_mut() {
            active.0 = false;
        }
    }

    let mut finger = middle;
    {
        for (_, _, mut active) in finger.0.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.1.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.2.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.3.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.4.iter_mut() {
            active.0 = false;
        }
    }

    let mut finger = ring;
    {
        for (_, _, mut active) in finger.0.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.1.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.2.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.3.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.4.iter_mut() {
            active.0 = false;
        }
    }

    let mut finger = little;
    {
        for (_, _, mut active) in finger.0.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.1.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.2.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.3.iter_mut() {
            active.0 = false;
        }
        for (_, _, mut active) in finger.4.iter_mut() {
            active.0 = false;
        }
    }
}

fn update_xr_finger_joint<
    Handedness: HandednessMarker,
    Finger: FingerMarker,
    Joint: FingerJointMarker,
    F1: FingerMarker,
    F2: FingerMarker,
    F3: FingerMarker,
    F4: FingerMarker,
    J1: FingerJointMarker,
    J2: FingerJointMarker,
    J3: FingerJointMarker,
    J4: FingerJointMarker,
>(
    joint: &mut Query<
        (Entity, &mut Transform, &mut XrActive),
        (
            With<XrLocal>,
            With<Finger>,
            With<Joint>,
            With<Handedness>,
            Without<Wrist>,
            Without<F1>,
            Without<F2>,
            Without<F3>,
            Without<F4>,
            Without<J1>,
            Without<J2>,
            Without<J3>,
            Without<J4>,
        ),
    >,
    previous_joint: &Option<(Entity, XrJointSpace)>,
    hand: &WebXrHand,
    frame: &XrFrame,
    commands: &mut Commands,
) -> Option<(Entity, XrJointSpace)>
where
    (Finger, Joint): IntoEnum<Hand> + WebXrHandJoint,
{
    if let Some((previous_entity, previous_joint_space)) = previous_joint {
        let joint_space = hand.get(<(Finger, Joint)>::webxr_hand_joint()); //(finger, joint).into_webxr_hand_joint());
        if let Some(joint_pose) =
            frame.get_joint_pose(&joint_space, previous_joint_space.dyn_ref().unwrap())
        {
            let entity = match joint.get_single_mut() {
                Ok((entity, mut transform, mut active)) => {
                    transform.translation = dom_point_to_vec3(&joint_pose.transform().position());
                    transform.rotation = dom_point_to_quat(&joint_pose.transform().orientation());
                    active.0 = true;
                    entity
                }
                Err(QuerySingleError::MultipleEntities(_)) => {
                    let mut joint = joint.iter();
                    let (entity, _, _) = joint.next().unwrap();
                    for (entity, _, _) in joint {
                        commands.entity(entity).despawn();
                    }
                    entity
                }
                Err(QuerySingleError::NoEntities(_)) => {
                    let mut entity =
                        commands.spawn(FingerJointBundle::<Handedness, Finger, Joint> {
                            spatial_bundle: SpatialBundle {
                                transform: Transform {
                                    translation: dom_point_to_vec3(
                                        &joint_pose.transform().position(),
                                    ),
                                    rotation: dom_point_to_quat(
                                        &joint_pose.transform().orientation(),
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        });
                    entity.log_components();
                    let entity = entity.id();
                    entity
                }
            };
            commands
                .entity(previous_entity.to_owned())
                .add_child(entity);
            return Some((entity, joint_space));
        }
        warn!("Failed to get joint space!");
    }

    for (_, _, mut active) in joint.iter_mut() {
        active.0 = false;
    }

    None
}

///
/// Traits
///

pub trait WebXrHandedness {
    fn webxr_handedness() -> XrHandedness;
}

impl WebXrHandedness for LeftHanded {
    fn webxr_handedness() -> XrHandedness {
        XrHandedness::Left
    }
}

impl WebXrHandedness for RightHanded {
    fn webxr_handedness() -> XrHandedness {
        XrHandedness::Right
    }
}

// Hand

trait WebXrHandJoint {
    fn webxr_hand_joint() -> XrHandJoint;
}

impl WebXrHandJoint for Wrist {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::Wrist
    }
}

// Thumb

impl WebXrHandJoint for (Thumb, Metacarpal) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::ThumbMetacarpal
    }
}

impl WebXrHandJoint for (Thumb, ProximalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::ThumbPhalanxProximal
    }
}

impl WebXrHandJoint for (Thumb, DistalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::ThumbPhalanxDistal
    }
}

impl WebXrHandJoint for (Thumb, Tip) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::ThumbTip
    }
}

// Index

impl WebXrHandJoint for (Index, Metacarpal) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::IndexFingerMetacarpal
    }
}

impl WebXrHandJoint for (Index, ProximalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::IndexFingerPhalanxProximal
    }
}

impl WebXrHandJoint for (Index, IntermediatePhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::IndexFingerPhalanxIntermediate
    }
}

impl WebXrHandJoint for (Index, DistalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::IndexFingerPhalanxDistal
    }
}

impl WebXrHandJoint for (Index, Tip) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::IndexFingerTip
    }
}

// Middle

impl WebXrHandJoint for (Middle, Metacarpal) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::MiddleFingerMetacarpal
    }
}

impl WebXrHandJoint for (Middle, ProximalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::MiddleFingerPhalanxProximal
    }
}

impl WebXrHandJoint for (Middle, IntermediatePhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::MiddleFingerPhalanxIntermediate
    }
}

impl WebXrHandJoint for (Middle, DistalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::MiddleFingerPhalanxDistal
    }
}

impl WebXrHandJoint for (Middle, Tip) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::MiddleFingerTip
    }
}

// Ring

impl WebXrHandJoint for (Ring, Metacarpal) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::RingFingerMetacarpal
    }
}

impl WebXrHandJoint for (Ring, ProximalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::RingFingerPhalanxProximal
    }
}

impl WebXrHandJoint for (Ring, IntermediatePhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::RingFingerPhalanxIntermediate
    }
}

impl WebXrHandJoint for (Ring, DistalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::RingFingerPhalanxDistal
    }
}

impl WebXrHandJoint for (Ring, Tip) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::RingFingerTip
    }
}

// Little

impl WebXrHandJoint for (Little, Metacarpal) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::PinkyFingerMetacarpal
    }
}

impl WebXrHandJoint for (Little, ProximalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::PinkyFingerPhalanxProximal
    }
}

impl WebXrHandJoint for (Little, IntermediatePhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::PinkyFingerPhalanxIntermediate
    }
}

impl WebXrHandJoint for (Little, DistalPhalanx) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::PinkyFingerPhalanxDistal
    }
}

impl WebXrHandJoint for (Little, Tip) {
    fn webxr_hand_joint() -> XrHandJoint {
        XrHandJoint::PinkyFingerTip
    }
}
