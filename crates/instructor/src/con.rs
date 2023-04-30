use std::num::NonZeroU16;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;
use cyproto_core::Command;

use crate::{Socket, State};

#[derive(Parser, ConsoleCommand)]
#[command(name = "drive", about = "Drive the cybot")]
pub struct DriveCli {
    #[arg(allow_negative_numbers = true)]
    pub distance: f32,
    #[arg(default_value_t = NonZeroU16::new(200).unwrap())]
    pub speed: NonZeroU16,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "turn", about = "Turn the cybot")]
pub struct TurnCli {
    #[arg(allow_negative_numbers = true)]
    pub angle: f32,
    #[arg(default_value_t = NonZeroU16::new(200).unwrap())]
    pub speed: NonZeroU16,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "scan", about = "Scan for objects on the field")]
pub struct ScanCli {
    #[arg(default_value_t = 0)]
    pub start: u8,
    #[arg(default_value_t = 180)]
    pub end: u8,
}

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
                height: 1080.,
                ..Default::default()
            });
    }
}
