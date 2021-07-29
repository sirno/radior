#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[path = "cutils.rs"]
#[macro_use]
mod cutils;

use chrono::Duration;
use cutils::{readcstr, readcstrf};
use std::cmp::{max, min};
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

#[derive(Default, Clone)]
pub struct StreamState {
    stream_title: String,
    stream_duration: f32,
    stream_time: f32,
    stream_volume: f32,
}

const TILE_SIZE: i32 = 20;

impl StreamState {
    pub fn get_display(&self) -> String {
        let title_size = self.stream_title.chars().count() as i32;
        let mut title = self.stream_title.to_string();
        if title_size > TILE_SIZE {
            let linger = 4;
            let mut offset =
                ((2. * self.stream_time) as i32 % (title_size - TILE_SIZE + 2 * linger)) - linger;
            offset = max(min(offset, title_size - TILE_SIZE), 0);
            let start = offset as usize;
            let end = (TILE_SIZE + offset) as usize;
            title = self.stream_title[start..end].to_string();
        }
        return format!(
            "\n Title: {:size$} \n Volume: {} \n {} / {} \n ",
            title,
            self.stream_volume,
            Duration::seconds(self.stream_time as i64).clock(),
            Duration::seconds(self.stream_duration as i64).clock(),
            size = TILE_SIZE as usize,
        );
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

#[derive(Debug, Copy, Clone)]
pub struct Mpv {
    handle: *mut mpv_handle,
}

impl Mpv {
    pub fn new() -> Self {
        unsafe {
            let handle = mpv_create();
            check_error(mpv_set_option_string(
                handle,
                cstr!("input-default-bindings"),
                cstr!("yes"),
            ));

            check_error(mpv_set_option_string(
                handle,
                cstr!("input-vo-keyboard"),
                cstr!("yes"),
            ));

            check_error(mpv_initialize(handle));
            return Self { handle: handle };
        }
    }

    pub fn loadfile(&mut self, filename: &str) {
        unsafe {
            check_error(mpv_command(
                self.handle,
                [cstr!("loadfile"), cstr!(filename), std::ptr::null()].as_mut_ptr(),
            ));
        }
    }

    pub fn playlist_next(&mut self) {
        unsafe {
            mpv_command(
                self.handle,
                [cstr!("playlist-next"), std::ptr::null()].as_mut_ptr(),
            );
        }
    }

    pub fn playlist_prev(&mut self) {
        unsafe {
            mpv_command(
                self.handle,
                [cstr!("playlist-prev"), std::ptr::null()].as_mut_ptr(),
            );
        }
    }

    pub fn add_property(&mut self, property: &str, value: i32) {
        unsafe {
            mpv_command(
                self.handle,
                [
                    cstr!("add"),
                    cstr!(property),
                    cstr!(value.to_string()),
                    std::ptr::null(),
                ]
                .as_mut_ptr(),
            );
        }
    }

    pub fn terminate(&mut self) {
        unsafe {
            mpv_terminate_destroy(self.handle);
        }
    }

    pub fn quit(&mut self) {
        unsafe {
            mpv_command(self.handle, [cstr!("quit"), std::ptr::null()].as_mut_ptr());
        }
    }

    pub fn get_streamstate(&mut self) -> StreamState {
        unsafe {
            return StreamState {
                stream_title: self.get_title(),
                stream_duration: self.get_duration(),
                stream_time: self.get_time(),
                stream_volume: self.get_volume(),
            };
        }
    }

    unsafe fn get_title(&mut self) -> String {
        return readcstr(mpv_get_property_string(self.handle, cstr!("media-title")));
    }

    unsafe fn get_duration(&mut self) -> f32 {
        return readcstrf(mpv_get_property_string(self.handle, cstr!("duration")));
    }

    unsafe fn get_time(&mut self) -> f32 {
        return readcstrf(mpv_get_property_string(self.handle, cstr!("time-pos")));
    }

    unsafe fn get_volume(&mut self) -> f32 {
        return readcstrf(mpv_get_property_string(self.handle, cstr!("volume")));
    }
}
