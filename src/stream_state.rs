use chrono::Duration;
use libmpv::Mpv;
use std::cmp::{max, min};

const TILE_SIZE: i64 = 20;

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

pub trait StreamState {
    fn get_display(&self) -> String;
    fn get_title(&self) -> String;
    fn get_duration(&self) -> f64;
    fn get_time(&self) -> f64;
    fn get_volume(&self) -> f64;
}

impl StreamState for Mpv {
    fn get_display(&self) -> String {
        let mut title = self.get_title().to_string();
        let title_size = title.len() as i64;
        if title_size > TILE_SIZE {
            let linger = 4;
            let mut offset =
                ((2. * self.get_time()) as i64 % (title_size - TILE_SIZE + 2 * linger)) - linger;
            offset = max(min(offset, title_size - TILE_SIZE), 0);
            let start = offset as usize;
            let end = (TILE_SIZE + offset) as usize;
            title = self.get_title()[start..end].to_string();
        }
        return format!(
            "\n Title: {:size$} \n Volume: {} \n {} / {} \n ",
            title,
            self.get_volume(),
            Duration::seconds(self.get_time() as i64).clock(),
            Duration::seconds(self.get_duration() as i64).clock(),
            size = TILE_SIZE as usize,
        );
    }

    fn get_title(&self) -> String {
        self.get_property("media-title")
            .unwrap_or("Station".to_string())
    }

    fn get_duration(&self) -> f64 {
        self.get_property("duration").unwrap_or(0.)
    }

    fn get_time(&self) -> f64 {
        self.get_property("time-pos").unwrap_or(0.)
    }

    fn get_volume(&self) -> f64 {
        self.get_property("volume").unwrap_or(0.)
    }
}
