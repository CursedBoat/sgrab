use serde::Deserialize;
use std::{
    env, fs,
    process::{exit, Command},
};

#[derive(Deserialize)]
struct Config {
    fps: u64,
    res_x: u64,
    res_y: u64,
    off_x: u64,
    off_y: u64,
    dir: String,
}

pub fn create_conf() {
    let user = env::var("USER").expect("No user variable");
    let defaults = format!("#Sample config placed at ~/.config/sgrab\nfps = 30\nres_x = 1920\nres_y = 1080\noff_x = 0\noff_y = 0\ndir = \"/home/{}/Videos/\"", user);

    if let Err(e) = fs::create_dir(format!("/home/{}/.config/sgrab", user)) {
        if e.to_string() == "File exists (os error 17)" {
            Command::new("cat")
                .arg(format!("/home/{}/.config/sgrab/config", user))
                .spawn()
                .expect("Could not cat the config file.");
        }
        return;
    }
    fs::File::create(format!("/home/{}/.config/sgrab/config", user))
        .expect("Could not create sgrab config file");
    fs::write(format!("/home/{}/.config/sgrab/config", user), defaults)
        .expect("Could not write defaults");

    Command::new("cat")
        .arg(format!("/home/{}/.config/sgrab/config", user))
        .spawn()
        .expect("Could not launch default text editor.");
}

fn config() -> Config {
    let user = env::var("USER").expect("No user env variable");
    let contents = match fs::read_to_string(format!("/home/{}/.config/sgrab/config", user)) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Make sure that the configuration exists in ~/.config/sgrab/config and has these configured: \nfps, res_x, res_y, off_x, off_y");
            exit(1);
        }
    };

    let data: Config = toml::from_str(&contents).unwrap();
    return data;
}

pub fn fps() -> u64 {
    return config().fps;
}

pub fn res_x() -> u64 {
    return config().res_x;
}

pub fn res_y() -> u64 {
    return config().res_y;
}

pub fn off_x() -> u64 {
    return config().off_x;
}

pub fn off_y() -> u64 {
    return config().off_y;
}

pub fn dir() -> String {
    return config().dir;
}
