use std::net::TcpStream;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        shape::{Circle, Quad},
        *,
    },
    sprite::MaterialMesh2dBundle, input::mouse::MouseMotion,
};
use bevy_console::PrintConsoleLine;
use com::read_response;
use console::CliPlugin;
use cyproto_core::{ObjectData, Response};

mod com;
mod console;

const CYBOT_RADIUS_CM: f32 = 16.;

#[derive(Resource)]
pub struct Socket(TcpStream);

#[derive(Clone, Copy, Debug, Resource, PartialEq)]
pub enum State {
    Normal,
    SentDrive { distance: f32 },
    SentTurn { angle: f32 },
    SentScan { start: u8, end: u8 },
}

#[derive(Component)]
pub struct Cybot;

#[derive(Component)]
pub struct PreviousCybot;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Obstacle;

#[derive(Clone, Copy)]
pub struct CliffEvent {
    color: Color,
}

#[derive(Clone, Copy)]
pub struct PathEvent;

fn cm_to_unit(cm: f32) -> f32 {
    cm * 2.0
}

/*fn unit_to_cm(unit: f32) -> f32 {
    unit / 2.0
}*/

/// Spawn the path that the robot followed
fn spawn_path(
    mut ev_path: EventReader<PathEvent>,
    cybot_pos: Query<&Transform, (With<Cybot>, Without<PreviousCybot>)>,
    prev_pos: Query<&Transform, (With<PreviousCybot>, Without<Cybot>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let prev_pos = prev_pos.single();
    let cybot_pos = cybot_pos.single();

    const LINE_WIDTH: f32 = 2.;
    let line_height = (prev_pos.translation - cybot_pos.translation)
        .length()
        .abs();
    let mut mid: Transform =
        Transform::from_translation(prev_pos.translation.lerp(cybot_pos.translation, 0.5));
    mid.translation.z = 0.;
    mid.rotate(cybot_pos.rotation);

    for _ in ev_path.iter() {
        commands.spawn(
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Quad::new(Vec2::new(LINE_WIDTH, line_height)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: mid,
                ..default()
            }
        );
    }
}

/// Spawn a cliff or bump object in the field
fn spawn_cliff(
    mut ev_cliffs: EventReader<CliffEvent>,
    cybot_pos: Query<&Transform, With<Cybot>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cybot_pos = cybot_pos.single();
    for ev in ev_cliffs.iter() {
        let mut obj_pos = cybot_pos.clone();
        obj_pos.translation +=
            cybot_pos
                .rotation
                .mul_vec3(Vec3::new(0., cm_to_unit(CYBOT_RADIUS_CM / 2.), 0.));
        obj_pos.translation.z += 2.;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        Quad::new(Vec2::new(
                            cm_to_unit(CYBOT_RADIUS_CM * 2.),
                            cm_to_unit(CYBOT_RADIUS_CM / 5.),
                        ))
                        .into(),
                    )
                    .into(),
                material: materials.add(ColorMaterial::from(ev.color)),
                transform: obj_pos,
                ..default()
            },
            Obstacle,
        ));
    }
}

/// Spawn an object that was detected by the scanner
fn spawn_object(
    mut ev_objs: EventReader<ObjectData>,
    cybot_pos: Query<&Transform, (With<Cybot>, Without<PreviousCybot>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cybot_pos = cybot_pos.single();
    let scanner_pos = {
        let mut sp = cybot_pos.clone();
        sp.translation += sp
            .rotation
            .mul_vec3(Vec3::new(0., cm_to_unit(CYBOT_RADIUS_CM - 2.), 0.));
        sp.translation
    };
    for object in ev_objs.iter() {
        let obj_radius = object.width / 2.;
        let mut obj_pos = cybot_pos.clone();
        obj_pos.translation += obj_pos.rotation.mul_vec3(Vec3::new(
            cm_to_unit(object.distance + obj_radius),
            cm_to_unit(CYBOT_RADIUS_CM - 2.),
            0.,
        ));
        obj_pos.rotate_around(
            scanner_pos,
            Quat::from_rotation_z(f32::from(object.angle).to_radians()),
        );

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Circle::new(cm_to_unit(obj_radius)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: obj_pos,
                    ..default()
                },
                Object,
            ))
            .with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Circle::new(cm_to_unit(obj_radius / 2.)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                });
            });
    }
}

/// Initial setup of the data required for the GUI
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Circle::new(cm_to_unit(CYBOT_RADIUS_CM)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            Cybot,
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        Quad::new(Vec2::new(
                            cm_to_unit(CYBOT_RADIUS_CM / 5.),
                            cm_to_unit(CYBOT_RADIUS_CM),
                        ))
                        .into(),
                    )
                    .into(),
                //mesh: meshes.add(Circle::new(cm_to_unit(CYBOT_RADIUS_CM / 2.5)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., CYBOT_RADIUS_CM, 1.)),
                ..default()
            });
        });

    commands.spawn((
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        PreviousCybot,
    ));
}


/// Allows the user to drag the camera around with their mouse and move the field
fn cursor_drag(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut ev_mouse_move: EventReader<MouseMotion>,
    btn: Res<Input<MouseButton>>,
) {
    let mut cam_pos = query.single_mut();
    for ev in ev_mouse_move.iter() {
        if btn.pressed(MouseButton::Right) {
            cam_pos.translation.x -= ev.delta.x;
            cam_pos.translation.y += ev.delta.y;
        }
    }
}

/// Update the state of the GUI checking if a command was sent to the robot, and a response has
/// come back
fn update(
    mut ev_objs: EventWriter<ObjectData>,
    mut ev_cliffs: EventWriter<CliffEvent>,
    mut ev_path: EventWriter<PathEvent>,
    mut console: EventWriter<PrintConsoleLine>,
    mut state: ResMut<State>,
    mut socket: ResMut<Socket>,
    mut cybot: Query<&mut Transform, (With<Cybot>, Without<PreviousCybot>)>,
    mut prev: Query<&mut Transform, (With<PreviousCybot>, Without<Cybot>)>,
) {
    let mut cybot_pos = cybot.single_mut();
    let mut prev_pos = prev.single_mut();

    // If the state is non-normal then we sent a command, check for a response
    if *state != State::Normal {
        let response = read_response(&mut socket);
        if response.is_err() {
            console.send(PrintConsoleLine::new(response.unwrap_err().to_string().into()));
            *state = State::Normal;
            return;
        }
        let response = response.unwrap();
        match (*state, response) {
            (
                State::SentDrive { .. },
                Some(Response::DriveDone {
                    total_distance,
                    cliff_detected,
                    bump_detected,
                }),
            ) => {
                *prev_pos = cybot_pos.clone();
                let move_by =
                    cybot_pos
                        .rotation
                        .mul_vec3(Vec3::new(0., cm_to_unit(total_distance), 0.));
                cybot_pos.translation += move_by;

                console.send_batch([
                    PrintConsoleLine::new(format!("Drove: {total_distance:.2}cm").into()),
                    PrintConsoleLine::new(format!("\tcliff: {cliff_detected}").into()),
                    PrintConsoleLine::new(format!("\tbump: {bump_detected}").into()),
                ]);
                ev_path.send(PathEvent);

                if cliff_detected {
                    ev_cliffs.send(CliffEvent { color: Color::RED });
                }
                if bump_detected {
                    ev_cliffs.send(CliffEvent { color: Color::ORANGE });
                }
            }
            (State::SentTurn { .. }, Some(Response::TurnDone { total_angle })) => {
                *prev_pos = cybot_pos.clone();
                cybot_pos.rotate_z(total_angle.to_radians());
                console.send(PrintConsoleLine::new(format!("Turned: {total_angle:.2}°").into()));
            }
            (State::SentScan { .. }, Some(Response::ScanDone { data })) => {
                console.send(PrintConsoleLine::new(
                    format!("Scanned: {} objects", data.len()).into(),
                ));

                console.send_batch(data.iter().enumerate().map(|(i, obj)| {
                    PrintConsoleLine::new(
                        format!(
                            "\t{i}. angle: {} distance: {:.2} width: {:.2}",
                            obj.angle, obj.distance, obj.width
                        )
                        .into(),
                    )
                }));

                ev_objs.send_batch(data);
            }
            (_, None) => {
                return;
            }
            (cmd, resp) => {
                console.send(PrintConsoleLine::new(format!("Invalid response for command: {cmd:?} {resp:?}").into()));
            },
        }
        *state = State::Normal;
    }
}

/// The main function where the GUI is initialized
fn main() {
    // create the connection to the cybot
    let socket = Socket(TcpStream::connect("localhost:2888").unwrap());
    socket
        .0
        .set_nonblocking(true)
        .expect("cannot get non-blocking");

    // Start the GUI
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CliPlugin)
        .insert_resource(State::Normal)
        .insert_resource(socket)
        .add_event::<PathEvent>()
        .add_event::<ObjectData>()
        .add_event::<CliffEvent>()
        .add_startup_system(setup)
        .add_system(spawn_path)
        .add_system(spawn_object)
        .add_system(spawn_cliff)
        .add_system(cursor_drag)
        .add_system(update)
        .run();
}
