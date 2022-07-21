use chrono::Duration;
use libmpv::Mpv;
use std::cmp::{max, min};

const TILE_SIZE: usize = 20;
const LINGER: i64 = 4;

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
    fn get_media_title(&self) -> String;
    fn get_duration(&self) -> f64;
    fn get_time(&self) -> f64;
    fn get_volume(&self) -> f64;
}

impl StreamState for Mpv {
    fn get_display(&self) -> String {
        return format!(
            "\n Title: {:size$} \n Volume: {} \n {} / {} \n ",
            self.get_title(),
            self.get_volume(),
            Duration::seconds(self.get_time() as i64).clock(),
            Duration::seconds(self.get_duration() as i64).clock(),
            size = TILE_SIZE as usize,
        );
    }

    fn get_title(&self) -> String {
        let title = self.get_media_title();
        if title == " - " {
            return "---".to_string();
        }
        match title.len() {
            0 => "---".to_string(),
            1..=TILE_SIZE => title,
            title_size => {
                let title_size_int = title_size as i64;
                let tile_size_int = TILE_SIZE as i64;
                let mut offset = ((2. * self.get_time()) as i64
                    % (title_size_int - tile_size_int + 2 * LINGER))
                    - LINGER;
                offset = max(min(offset, title_size as i64 - tile_size_int), 0);
                let start = offset as usize;
                let end = (tile_size_int + offset) as usize;
                title[start..end].to_string()
            }
        }
    }

    fn get_media_title(&self) -> String {
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
