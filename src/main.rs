#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod mpv;
mod radio;
mod view;

use clap::Parser;
use cursive::view::View;
use cursive::views::Dialog;
use cursive::{Cursive, CursiveExt};
use radio::Radio;
use shellexpand;
use std::fs;
use std::io::ErrorKind;
use toml::Value;
use url::Url;
use view::{playerhelp, radiohelp};
use view::{PlayerView, RadioView};

#[derive(Parser)]
#[clap(version = "0.1", author = "Nicolas Ochsner <nicolasochsner@gmail.com>")]
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

    let config: Value = match fs::read_to_string(config_path) {
        Ok(content) => match content.parse::<Value>() {
            Ok(config) => config,
            Err(_) => {
                println!("The config file is invalid.");
                std::process::exit(1);
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let raw_config: &str = include_str!("config.toml");
                let config_path_directory = config_path.parent().unwrap();
                fs::create_dir_all(config_path_directory).expect("Unable to create directories.");
                fs::write(config_path, raw_config).expect("Unable to write config file.");
                raw_config
                    .to_string()
                    .parse()
                    .expect("Unable to parse default config.")
            }
            other => panic!("Problem opening config file: {:?}", other),
        },
    };

    let mut siv = Cursive::new();
    let gethelp: fn() -> Dialog;

    let boxed_view: Box<dyn View> = match Url::parse(input.as_str()) {
        Ok(url) => {
            gethelp = playerhelp;
            Box::new(PlayerView::new_with_url(url.to_string()))
        }
        Err(_) => {
            gethelp = radiohelp;
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

            let radio = Radio::new_with_index(
                stations,
                station_urls
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect(),
                station_index,
            );
            Box::new(RadioView::new(radio))
        }
    };

    siv.add_layer(boxed_view);

    siv.add_global_callback('q', move |s| {
        s.quit();
    });

    // Help / View keymappings
    siv.add_global_callback('?', move |s| {
        if let Some(pos) = s.screen_mut().find_layer_from_name("help_view") {
            s.screen_mut().remove_layer(pos);
        } else {
            s.add_layer(gethelp());
        }
    });

    siv.set_fps(10);
    siv.run();
}

// fn help() -> Dialog {
//     let bindings = r###"
// General<br>
// -: Decrease volume<br>
// =: Increase volume<br>
// n: Next track<br>
// p: Previous track<br><br>
// Radio<br>
// ,: Previous station<br>
// .: Next Station<br><br>
// Misc<br>
// q: Exit<br>
// ?: Toggle this help menu<br>
// "###;

//     return Dialog::around(cursive_markup::MarkupView::html(&bindings));
// }
