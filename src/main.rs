#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod mpv;
mod radio;
mod view;

use clap::{AppSettings, Clap};
use cursive::{Cursive, CursiveExt};
use radio::Radio;
use shellexpand;
use toml::Value;
use view::MpvView;

#[derive(Clap)]
#[clap(version = "0.1", author = "Nicolas Ochsner <nicolasochsner@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    input: Option<String>,
    #[clap(long, default_value = "~/.config/radior/config.toml")]
    config: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let input = opts.input.unwrap_or_else(|| "nts".to_string());

    let expanded_path = shellexpand::tilde(opts.config.as_str());
    let config_path = std::path::Path::new(expanded_path.as_ref());

    if !config_path.exists() {
        println!("Unable to finde config list at {}.", expanded_path);
        std::process::exit(1);
    }

    let config: Value = std::fs::read_to_string(config_path)
        .expect("Unable to read config.")
        .parse()
        .unwrap();

    let radios = &config["radios"].as_table().unwrap();

    let stations: Vec<String> = radios.keys().cloned().collect();
    let station_urls: Vec<&Value> = radios.values().collect();

    let mut station_index = 0;
    let mut idx = 0;
    for station in stations.iter() {
        if *station == input {
            station_index = idx;
            break;
        }
        idx += 1;
    }

    let mut siv = Cursive::new();

    let radio = Radio::new_with_index(
        stations,
        station_urls
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect(),
        station_index,
    );

    let mpv_view = MpvView::new(radio);

    siv.add_layer(mpv_view);

    siv.add_global_callback('q', move |s| {
        s.quit();
    });

    siv.set_fps(10);
    siv.run();
}
