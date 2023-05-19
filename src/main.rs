use colored::Colorize;
use ffmpeg_sidecar::{
    command::FfmpegCommand,
    event::{FfmpegEvent, LogLevel},
};
mod modules;
use modules::*;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use terminal_menu::{button, label, menu, mut_menu, run};

fn main() {
    let args = std::env::args().nth(1);
    if let None = args {
        let menu = menu(vec![
            label("Choose option:"),
            button("Record"),
            button("Create configs"),
            button("Exit"),
        ]);
        run(&menu);
        {
            let mm = mut_menu(&menu);
            match mm.selected_item_name() {
                "Record" => record(),
                "Create configs" => config::create_conf(),
                _ => {}
            }
        }
    } else {
        let args = args.unwrap();
        if args == "-n" {
            record()
        } else {
            println!("USAGE:\n\nsgrab -> launches the terminal ui to initialize the program\nsgrab -n -> starts recording instantly")
        }
    }
}

fn record() {
    FfmpegCommand::new()
        .args([
            "-video_size",
            format!("{}x{}", config::res_x(), config::res_y()).as_str(),
            "-framerate",
            config::fps().to_string().as_str(),
            "-f",
            "x11grab",
            "-i",
            format!(":0.0+{},{}", config::off_x(), config::off_y()).as_str(),
            "-f",
            "pulse",
            "-ac",
            "2",
            "-i",
            "alsa_output.pci-0000_00_1b.0.analog-stereo.monitor",
            format!(
                "{}sgrab-{}.mp4",
                config::dir(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
            .as_str(),
        ])
        .spawn()
        .unwrap()
        .iter()
        .unwrap()
        .for_each(|e| match e {
            FfmpegEvent::Log(LogLevel::Error, e) => println!("Error: {}", e),
            FfmpegEvent::Progress(progress) => {
                let output = format!(
                    "Recording started. Current speed: {}x Bandwidth: {}kbps",
                    progress.speed.to_string().magenta(),
                    progress.bitrate_kbps.to_string().red()
                );
                print!("\r{}    ", output);
                std::io::stdout().flush().expect("Could not flush output.");
            }
            _ => {}
        });
}
