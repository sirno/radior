#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use chrono::Duration;
use cursive::event::Event;
use cursive::views::{TextContent, TextView};
use cursive::{Cursive, CursiveExt};
use std::cmp::{max, min};
use std::env;
use std::ffi::CStr;
use std::fs;
// use std::time::SystemTime;
use toml::Value;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const TILE_SIZE: i32 = 20;

macro_rules! cstr {
    ($s:expr) => {{
        std::ffi::CString::new($s)
            .expect("CString::new failed.")
            .as_ptr()
    }};
}

pub trait clock {
    fn get_seconds(&self) -> i64;

    fn clock(&self) -> String {
        let s = self.get_seconds();
        let ds = s % 60;
        let dm = s / 60 % 60;
        let dh = s / 3600;
        return format!("{:02}:{:02}:{:02}", dh, dm, ds);
    }
}

impl clock for Duration {
    fn get_seconds(&self) -> i64 {
        return self.num_seconds();
    }
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

unsafe fn readcstrf(s: *const ::std::os::raw::c_char) -> f32 {
    if s.is_null() {
        return 0.0;
    }
    return CStr::from_ptr(s)
        .to_string_lossy()
        .into_owned()
        .parse()
        .unwrap();
}

unsafe fn get_display(ctx: *mut mpv_handle) -> String {
    let mut title: String = readcstr(mpv_get_property_string(ctx, cstr!("media-title")));
    let vol: f32 = readcstrf(mpv_get_property_string(ctx, cstr!("volume")));
    let time: f32 = readcstrf(mpv_get_property_string(ctx, cstr!("time-pos")));
    let duration: f32 = readcstrf(mpv_get_property_string(ctx, cstr!("duration")));
    let title_size = title.chars().count() as i32;
    if title_size > TILE_SIZE {
        let linger = 4;
        let mut offset = ((2. * time) as i32 % (title_size - TILE_SIZE + 2 * linger)) - linger;
        offset = max(min(offset, title_size - TILE_SIZE), 0);
        let start = offset as usize;
        let end = (TILE_SIZE + offset) as usize;
        title = title[start..end].to_string();
    }
    return format!(
        "\n Title: {:size$} \n Volume: {} \n {} / {} \n ",
        title,
        vol,
        Duration::seconds(time as i64).clock(),
        Duration::seconds(duration as i64).clock(),
        size = TILE_SIZE as usize,
    );
}

// fn get_dancer(t: SystemTime) -> String {
//     let s = t.elapsed().unwrap().as_secs() as u32;
//     if s % 2 == 0 {
//         return " (._.)/\n<(  (  \n  /  \\ \n".to_string();
//     }
//     return "\\(._.) \n  )  )>\n  /  \\ \n".to_string();
// }

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

        let content = TextContent::new(get_display(ctx));
        let central_text = TextView::new_with_content(content.clone());

        siv.add_layer(central_text);

        // let t = SystemTime::now();
        // let dancer = TextContent::new(get_dancer(t));
        // let dancer_view = TextView::new_with_content(dancer.clone());

        // let layout = FixedLayout::new()
        //     .child(Rect::from_size((0, 0), (1, 1)), TextView::new("/"))
        //     .child(Rect::from_size((20, 0), (1, 1)), TextView::new(r"\"))
        //     .child(Rect::from_size((0, 4), (1, 1)), TextView::new(r"\"))
        //     .child(Rect::from_size((20, 4), (1, 1)), TextView::new("/"))
        //     .child(Rect::from_size((7, 1), (7, 3)), dancer_view);

        // siv.add_fullscreen_layer(layout);

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
            content_vol_up_clone.set_content(get_display(ctx));
        });

        let content_vol_down_clone = content.clone();
        siv.add_global_callback('-', move |_s| {
            mpv_command(
                ctx,
                [cstr!("add"), cstr!("volume"), cstr!("-2"), std::ptr::null()].as_mut_ptr(),
            );
            content_vol_down_clone.set_content(get_display(ctx));
        });

        let content_refresh = content.clone();
        // let dancer_refresh = dancer.clone();
        siv.add_global_callback(Event::Refresh, move |_s| {
            content_refresh.set_content(get_display(ctx));
            // dancer_refresh.set_content(get_dancer(t))
        });

        siv.set_fps(10);
        siv.run();

        mpv_terminate_destroy(ctx);
    }
}
