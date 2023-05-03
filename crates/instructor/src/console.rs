use std::num::NonZeroU16;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;
use cyproto_core::Command;

use crate::{Socket, State};


/// Drive the cybot
///
/// This command will drive the robot forwards or backwards
/// a positive distance value will drive forwards and a negative
/// distance value will drive backwards
#[derive(Parser, ConsoleCommand)]
#[command(name = "drive")]
pub struct DriveCli {
    #[arg(allow_negative_numbers = true)]
    pub distance: f32,
    #[arg(default_value_t = NonZeroU16::new(200).unwrap())]
    pub speed: NonZeroU16,
}


/// Turn the cybot
///
/// This command will turn the cybot counter-clockwise or clockwise
/// a positive value will turn it counter-clockwise (left) and
/// a negative value will turn it clockwise (right).
#[derive(Parser, ConsoleCommand)]
#[command(name = "turn")]
pub struct TurnCli {
    #[arg(allow_negative_numbers = true)]
    pub angle: f32,
    #[arg(default_value_t = NonZeroU16::new(100).unwrap())]
    pub speed: NonZeroU16,
}

/// Scan the field
///
/// This command tells the robot to scan the field for objects
#[derive(Parser, ConsoleCommand)]
#[command(name = "scan")]
pub struct ScanCli {
    #[arg(default_value_t = 0)]
    pub start: u8,
    #[arg(default_value_t = 180)]
    pub end: u8,
}


/// Send the drive command to the robot
fn do_drive(
    mut cli: ConsoleCommand<DriveCli>,
    mut socket: ResMut<Socket>,
    mut state: ResMut<State>,
) {
    let DriveCli { distance, speed } = match cli.take() {
        Some(Ok(cmd)) => cmd,
        _ => return,
    };

    if !matches!(*state, State::Normal) {
        cli.reply_failed("Unable to run command while another command is being processed");
        return;
    }

    crate::com::send_command(
        &mut socket,
        Command::Drive {
            distance,
            speed: speed.into(),
        },
    )
    .unwrap();
    *state = State::SentDrive { distance };
}

/// Send the turn command to the robot
fn do_turn(mut cli: ConsoleCommand<TurnCli>, mut socket: ResMut<Socket>, mut state: ResMut<State>) {
    let TurnCli { angle, speed } = match cli.take() {
        Some(Ok(cmd)) => cmd,
        _ => return,
    };

    if !matches!(*state, State::Normal) {
        cli.reply_failed("Unable to run command while another command is being processed");
        return;
    }

    crate::com::send_command(
        &mut socket,
        Command::Turn {
            angle,
            speed: speed.into(),
        },
    )
    .unwrap();
    *state = State::SentTurn { angle };
}

/// Send the scan command to the robot
fn do_scan(mut cli: ConsoleCommand<ScanCli>, mut socket: ResMut<Socket>, mut state: ResMut<State>) {
    let ScanCli { start, end } = match cli.take() {
        Some(Ok(cmd)) => cmd,
        _ => return,
    };

    if !matches!(*state, State::Normal) {
        cli.reply_failed("Unable to run command while another command is being processed");
        return;
    }

    crate::com::send_command(&mut socket, Command::Scan { start, end }).unwrap();
    *state = State::SentScan { start, end };
}

/// The plugin for adding the commands to the GUI
pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ConsolePlugin)
            .add_console_command::<DriveCli, _>(do_drive)
            .add_console_command::<TurnCli, _>(do_turn)
            .add_console_command::<ScanCli, _>(do_scan)
            .insert_resource(ConsoleConfiguration {
                left_pos: 0.,
                top_pos: 0.,
                width: 200.,
                height: 500.,
                ..Default::default()
            });
    }
}
