#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cursive::event::Event;
use cursive::views::TextContent;
use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};
use std::env;
use std::ffi::CStr;
use std::fs;
use toml::Value;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! cstr {
    ($s:expr) => {{
        std::ffi::CString::new($s)
            .expect("CString::new failed.")
            .as_ptr()
    }};
}

unsafe fn check_error(status: i32) {
    if status < 0 {
        let msg = CStr::from_ptr(mpv_error_string(status))
            .to_string_lossy()
            .into_owned();
        println!("Error: {}", msg);
        std::process::exit(1);
    }
}

unsafe fn readcstr(s: *const ::std::os::raw::c_char) -> String {
    if s.is_null() {
        return "".to_string();
    }
    return CStr::from_ptr(s).to_string_lossy().into_owned();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Value = fs::read_to_string("config.toml")
        .expect("Unable to read config.")
        .parse()
        .unwrap();

    println!("{:?}", config);

    unsafe {
        let ctx = mpv_create();

        check_error(mpv_set_option_string(
            ctx,
            cstr!("input-default-bindings"),
            cstr!("yes"),
        ));

        check_error(mpv_set_option_string(
            ctx,
            cstr!("input-vo-keyboard"),
            cstr!("yes"),
        ));

        check_error(mpv_initialize(ctx));

        check_error(mpv_command(
            ctx,
            [
                cstr!("loadfile"),
                // cstr!("http://stream-relay-geo.ntslive.net/stream"),
                cstr!(config["radios"][&args[1]].as_str().as_deref().unwrap()),
                std::ptr::null(),
            ]
            .as_mut_ptr(),
        ));

        let mut siv = Cursive::new();

        // let title: String = readcstr(mpv_get_property_string(ctx, cstr!("property-list")));
        let title: String = readcstr(mpv_get_property_string(ctx, cstr!("media-title")));

        let vol: f32 = readcstr(mpv_get_property_string(ctx, cstr!("volume")))
            .parse()
            .unwrap();

        let content = TextContent::new(format!(
            "Title: {}\nVolume: {}\nPress q to quit.",
            title, vol
        ));
        let central_text = TextView::new_with_content(content.clone());

        siv.add_layer(central_text);

        siv.add_global_callback('q', move |s| {
            s.quit();
            mpv_command(ctx, [cstr!("quit"), std::ptr::null()].as_mut_ptr());
        });

        siv.add_global_callback('n', move |_s| {
            mpv_command(ctx, [cstr!("playlist-next"), std::ptr::null()].as_mut_ptr());
        });

        siv.add_global_callback('p', move |_s| {
            mpv_command(ctx, [cstr!("playlist-prev"), std::ptr::null()].as_mut_ptr());
        });

        let content_vol_up_clone = content.clone();
        siv.add_global_callback('=', move |_s| {
            mpv_command(
                ctx,
                [cstr!("add"), cstr!("volume"), cstr!("2"), std::ptr::null()].as_mut_ptr(),
            );
            let vol: f32 = readcstr(mpv_get_property_string(ctx, cstr!("volume")))
                .parse()
                .unwrap();
            let title: String = readcstr(mpv_get_property_string(ctx, cstr!("media-title")));
            content_vol_up_clone.set_content(format!(
                "Title: {}\nVolume: {}\nPress q to quit.",
                title, vol
            ));
        });

        let content_vol_down_clone = content.clone();
        siv.add_global_callback('-', move |_s| {
            mpv_command(
                ctx,
                [cstr!("add"), cstr!("volume"), cstr!("-2"), std::ptr::null()].as_mut_ptr(),
            );
            let title: String = readcstr(mpv_get_property_string(ctx, cstr!("media-title")));
            let vol: f32 = readcstr(mpv_get_property_string(ctx, cstr!("volume")))
                .parse()
                .unwrap();
            content_vol_down_clone.set_content(format!(
                "Title: {}\nVolume: {}\nPress q to quit.",
                title, vol
            ));
        });

        let content_refresh = content.clone();
        siv.add_global_callback(Event::Refresh, move |_s| {
            let title: String = readcstr(mpv_get_property_string(ctx, cstr!("media-title")));
            let vol: f32 = readcstr(mpv_get_property_string(ctx, cstr!("volume")))
                .parse()
                .unwrap();
            content_refresh.set_content(format!(
                "Title: {}\nVolume: {}\nPress q to quit.",
                title, vol
            ));
        });

        siv.set_autorefresh(true);
        siv.run();

        mpv_terminate_destroy(ctx);
    }
}
