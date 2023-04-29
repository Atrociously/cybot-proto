use std::{f32::consts::PI, net::TcpStream, io::Read};

use bevy::{
    prelude::{shape::Circle, *},
    sprite::MaterialMesh2dBundle,
};
use com::{send_command, read_response};
use cyproto_core::{Command, Response};

mod com;

const CYBOT_RADIUS_CM: f32 = 32.;

#[derive(Resource)]
pub struct Socket(TcpStream);

#[derive(Clone, Copy, Resource, PartialEq, Eq)]
pub enum State {
    Normal,
    SentDrive,
    SentTurn,
    SentScan,
}

#[derive(Component)]
pub struct Cybot;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Obstacle;

fn cm_to_unit(cm: f32) -> f32 {
    cm * 0.5
}

fn unit_to_cm(unit: f32) -> f32 {
    unit / 0.5
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(cm_to_unit(CYBOT_RADIUS_CM)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            Cybot,
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 15., 2.)),
                ..default()
            });
        });
}

fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
   // mut socket: ResMut<Socket>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cybot: Query<&mut Transform, With<Cybot>>,
) {
    let mut cybot_pos = cybot.single_mut();

    /*
    if *state != State::Normal {
        let response = read_response(&mut socket).unwrap();
        match *state {
            State::SentDrive if response.is_some() => {}
            State::SentTurn if response.is_some() => {}
            State::SentScan if response.is_some() => {}
            _ => unreachable!(),
        }
    }*/


    if keys.pressed(KeyCode::W) {
        let move_by = cybot_pos.rotation.mul_vec3(Vec3::new(0., 100. * time.delta_seconds(), 0.));
        cybot_pos.translation += move_by;
    }
    if keys.pressed(KeyCode::S) {
        let move_by = cybot_pos.rotation.mul_vec3(Vec3::new(0., 100. * time.delta_seconds(), 0.));
        cybot_pos.translation -= move_by;
    }
    if keys.pressed(KeyCode::A) {
        cybot_pos.rotate_z(PI / 32.);
    }
    if keys.pressed(KeyCode::D) {
        cybot_pos.rotate_z(-PI / 32.);
    }
}

fn main() {
    let mut socket = Socket(TcpStream::connect("192.168.1.1:288").unwrap());

    while let Some(res) = read_response(&mut socket).unwrap() {
        println!("{res:?}");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        //.insert_resource(Socket(socket))
        .add_startup_system(setup)
        .add_system(update)
        .run();
}
