#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod mpv;
mod radio;

use clap::{AppSettings, Clap};
use cursive::event::Event;
use cursive::views::{TextContent, TextView};
use cursive::{Cursive, CursiveExt};
use mpv::Mpv;
use radio::Radio;
use shellexpand;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use toml::Value;

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
    let input = opts.input.expect("No input provided.");

    let config_path = shellexpand::tilde(opts.config.as_str());

    let config: Value = fs::read_to_string(config_path.as_ref())
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

    let station_url = station_urls[station_index].as_str().unwrap();

    let radio_state = RefCell::new(Radio::new_with_index(
        stations,
        station_urls
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect(),
        station_index,
    ));

    let radio_state_ref = Rc::new(radio_state);

    unsafe {
        let ctx = Rc::new(RefCell::new(Mpv::new()));

        ctx.borrow_mut().loadfile(station_url);
        // let ctx_clone = mpv_create();

        // check_error(mpv_set_option_string(
        //     ctx,
        //     cstr!("input-default-bindings"),
        //     cstr!("yes"),
        // ));

        // check_error(mpv_set_option_string(
        //     ctx,
        //     cstr!("input-vo-keyboard"),
        //     cstr!("yes"),
        // ));

        // check_error(mpv_initialize(ctx));

        // check_error(mpv_command(
        //     ctx,
        //     [cstr!("loadfile"), cstr!(station_url), std::ptr::null()].as_mut_ptr(),
        // ));

        let mut siv = Cursive::new();

        let radio_display_state = ctx.borrow_mut().get_streamstate();
        let content = TextContent::new(radio_display_state.get_display());
        let central_text = TextView::new_with_content(content.clone());

        siv.add_layer(central_text);

        let ctx_clone = ctx.clone();
        siv.add_global_callback('q', move |s| {
            s.quit();
            ctx_clone.borrow_mut().quit();
        });

        let ctx_clone = ctx.clone();
        siv.add_global_callback('n', move |_s| {
            ctx_clone.borrow_mut().playlist_next();
        });

        let ctx_clone = ctx.clone();
        siv.add_global_callback('p', move |_s| {
            ctx_clone.borrow_mut().playlist_prev();
        });

        let ctx_clone = ctx.clone();
        let rs_ref = radio_state_ref.clone();
        siv.add_global_callback(',', move |_s| {
            rs_ref.borrow_mut().prev();
            ctx_clone
                .borrow_mut()
                .loadfile(rs_ref.borrow().get_url().as_str());
        });

        let ctx_clone = ctx.clone();
        let rs_ref = radio_state_ref.clone();
        siv.add_global_callback('.', move |_s| {
            rs_ref.borrow_mut().next();
            ctx_clone
                .borrow_mut()
                .loadfile(rs_ref.borrow().get_url().as_str());
        });

        let ctx_clone = ctx.clone();
        let content_clone = content.clone();
        siv.add_global_callback('=', move |_s| {
            ctx_clone.borrow_mut().add_property("volume", 2);
            content_clone.set_content(ctx_clone.borrow_mut().get_streamstate().get_display());
        });

        let ctx_clone = ctx.clone();
        let content_clone = content.clone();
        siv.add_global_callback('-', move |_s| {
            ctx_clone.borrow_mut().add_property("volume", -2);
            content_clone.set_content(ctx_clone.borrow_mut().get_streamstate().get_display());
        });

        let ctx_clone = ctx.clone();
        let content_clone = content.clone();
        siv.add_global_callback(Event::Refresh, move |_s| {
            content_clone.set_content(ctx_clone.borrow_mut().get_streamstate().get_display());
        });

        siv.set_fps(10);
        siv.run();

        ctx.borrow_mut().terminate();
    }
}
