use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_webxr::WebXrPlugin;
use bevy_xr::controller::XrController;
use bevy_xr::controller_input::{DigitalInput, XrControllerInputType, XrControllerPress};
use bevy_xr::handedness::*;
use bevy_xr::hands::{finger::*, finger_joint::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(WebXrPlugin::default());

    app.add_systems(Startup, setup);

    app.add_systems(Update, bevy_xr::systems::draw_hand_gizmos);
    app.add_systems(Update, bevy_xr::systems::draw_controller_gizmos);

    app.add_event::<ThumbsUpEvent>();
    app.add_systems(Update, thumbs_up_gesture::<RightHanded>);
    app.add_event::<ThumbsUpEvent>();
    app.add_systems(Update, thumbs_up_gesture::<LeftHanded>);

    app.add_event::<ControllerTriggerEvent>();
    app.add_systems(Update, controller_trigger::<RightHanded>);
    app.add_event::<ControllerTriggerEvent>();
    app.add_systems(Update, controller_trigger::<LeftHanded>);

    app.add_event::<BalloonEvent>();
    app.add_systems(PostUpdate, balloon_events.before(balloons));
    app.add_systems(PostUpdate, balloons);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });
}

#[derive(Clone)]
enum EventState {
    Enter,
    Stay,
    Exit,
}

#[derive(Event, Clone)]
struct ThumbsUpEvent {
    entity: Entity,
    position: GlobalTransform,
    state: EventState,
}

fn thumbs_up_gesture<Handed: HandednessMarker>(
    fingers: Query<
        &Transform,
        (
            With<Finger>,
            Without<Thumb>,
            Without<Metacarpal>,
            Without<Tip>,
            With<Handed>,
        ),
    >,
    proximal_index: Query<&GlobalTransform, (With<ProximalPhalanx>, With<Index>, With<Handed>)>,
    proximal_little: Query<&GlobalTransform, (With<ProximalPhalanx>, With<Little>, With<Handed>)>,
    thumb: Query<
        &GlobalTransform,
        (
            With<Thumb>,
            Or<(With<ProximalPhalanx>, With<DistalPhalanx>)>,
            With<Handed>,
        ),
    >,
    thumb_tip: Query<(Entity, &GlobalTransform), (With<Thumb>, With<Tip>, With<Handed>)>,
    mut was_thumbs_up: Local<bool>,
    mut events: EventWriter<ThumbsUpEvent>,
) {
    if proximal_index.is_empty() {
        return;
    }
    if proximal_little.is_empty() {
        return;
    }
    if thumb_tip.is_empty() {
        return;
    }

    let average_finger_angle = fingers
        .iter()
        .map(|transform| transform.rotation.angle_between(Quat::IDENTITY))
        .sum::<f32>()
        / fingers.iter().len() as f32;

    let up: Vec3 = proximal_index.single().translation() - proximal_little.single().translation();

    let thumb = thumb.iter().map(|transform| transform.forward()).sum();

    let thumb_angle = up.angle_between(thumb);

    info!(
        "{}",
        "average_finger_angle: ".to_string()
            + &average_finger_angle.to_string()
            + " | thumb_angle: "
            + &thumb_angle.to_string(),
    );

    let is_thumbs_up = average_finger_angle > PI / 3.0 && thumb_angle < PI / 4.0;

    let (entity, position) = thumb_tip.single().clone();

    match (is_thumbs_up, *was_thumbs_up) {
        (true, false) => {
            events.send(ThumbsUpEvent {
                entity,
                position: position.clone(),
                state: EventState::Enter,
            });
            *was_thumbs_up = true;
        }
        (true, true) => events.send(ThumbsUpEvent {
            entity,
            position: position.clone(),
            state: EventState::Stay,
        }),
        (false, true) => {
            events.send(ThumbsUpEvent {
                entity,
                position: position.clone(),
                state: EventState::Exit,
            });
            *was_thumbs_up = false;
        }
        _ => {}
    }
}

#[derive(Event, Clone)]
struct ControllerTriggerEvent {
    entity: Entity,
    position: GlobalTransform,
    state: EventState,
}

fn controller_trigger<Handed: HandednessMarker>(
    controller: Query<(Entity, &GlobalTransform, &XrController), With<Handed>>,
    input: Res<DigitalInput<XrControllerPress>>,
    mut was_pressed: Local<bool>,
    mut events: EventWriter<ControllerTriggerEvent>,
) {
    let Ok((entity, position, xr_controller)) = controller.get_single() else {
        return;
    };

    let mut buttons_presses = [
        XrControllerInputType::AorX,
        XrControllerInputType::BorY,
        XrControllerInputType::Bumper,
        XrControllerInputType::Trigger,
    ]
    .into_iter()
    .map(|button| XrControllerPress::new(xr_controller.clone(), button));

    let pressed = buttons_presses.any(|button| input.pressed(button));

    match (pressed, *was_pressed) {
        (true, false) => {
            events.send(ControllerTriggerEvent {
                entity,
                position: position.clone(),
                state: EventState::Enter,
            });
            *was_pressed = true;
        }
        (true, true) => events.send(ControllerTriggerEvent {
            entity,
            position: position.clone(),
            state: EventState::Stay,
        }),
        (false, true) => {
            events.send(ControllerTriggerEvent {
                entity,
                position: position.clone(),
                state: EventState::Exit,
            });
            *was_pressed = false;
        }
        _ => {}
    }
}

#[derive(Event)]
struct BalloonEvent {
    entity: Entity,
    position: GlobalTransform,
    state: EventState,
}

impl From<ThumbsUpEvent> for BalloonEvent {
    fn from(value: ThumbsUpEvent) -> Self {
        BalloonEvent {
            entity: value.entity,
            position: value.position,
            state: value.state,
        }
    }
}

impl From<ControllerTriggerEvent> for BalloonEvent {
    fn from(value: ControllerTriggerEvent) -> Self {
        BalloonEvent {
            entity: value.entity,
            position: value.position,
            state: value.state,
        }
    }
}

fn balloon_events(
    mut hand_events: EventReader<ThumbsUpEvent>,
    mut controller_events: EventReader<ControllerTriggerEvent>,
    mut balloon_events: EventWriter<BalloonEvent>,
) {
    for event in hand_events.read() {
        info!("Balloon!");
        balloon_events.send(event.to_owned().into());
    }
    for event in controller_events.read() {
        info!("Balloon!");
        balloon_events.send(event.to_owned().into());
    }
}

#[derive(Component)]
struct AttachedToEntity(Entity);

#[derive(Component)]
struct Balloon;

fn balloons(
    mut events: EventReader<BalloonEvent>,
    mut attached_balloons: Query<(Entity, &mut Transform, &AttachedToEntity), With<Balloon>>,
    mut free_balloons: Query<&mut Transform, (With<Balloon>, Without<AttachedToEntity>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mesh: Local<Option<Handle<Mesh>>>,
    mut material: Local<Option<Handle<StandardMaterial>>>,
) {
    for event in events.read() {
        if let Some((entity, mut transform, _)) = attached_balloons
            .iter_mut()
            .find(|(_, _, attached_to)| attached_to.0 == event.entity)
        {
            transform.translation = event.position.translation();
            transform.rotation = transform.rotation.lerp(
                Transform::default()
                    .looking_to(event.position.up(), event.position.forward())
                    .rotation,
                0.1,
            );
            transform.scale = transform.scale * 1.03;

            if let EventState::Exit = event.state {
                commands.entity(entity).remove::<AttachedToEntity>();
            }
        } else {
            let balloon_origin = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(event.position.translation())
                            .looking_to(event.position.up(), event.position.forward()),
                        ..default()
                    },
                    Balloon,
                    AttachedToEntity(event.entity),
                ))
                .id();

            let mesh_handle = if let Some(mesh) = mesh.clone() {
                mesh
            } else {
                let handle = meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..default()
                }));
                *mesh = Some(handle.clone());
                handle
            };

            let material_handle = if let Some(material) = material.clone() {
                material
            } else {
                let handle = materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                });
                *material = Some(handle.clone());
                handle
            };

            let balloon_mesh = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.00799, 0.00),
                        scale: Vec3::new(0.01, 0.01, 0.01),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(balloon_origin).add_child(balloon_mesh);

            let balloon_mesh = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.00554, 0.0),
                        scale: Vec3::new(0.00796, 0.00796, 0.00796),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(balloon_origin).add_child(balloon_mesh);

            let balloon_mesh = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0031, 0.0),
                        scale: Vec3::new(0.00624, 0.00624, 0.00621),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(balloon_origin).add_child(balloon_mesh);

            let balloon_mesh = commands
                .spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, -0.0003, 0.0),
                        scale: Vec3::new(0.001, 0.001, 0.001),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(balloon_origin).add_child(balloon_mesh);
        }
    }

    for mut transform in free_balloons.iter_mut() {
        transform.rotation = transform.rotation.lerp(Quat::IDENTITY, 0.03);
        transform.translation += Vec3::new(0.0, 0.007, 0.0);
    }
}
