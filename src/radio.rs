#[derive(Clone)]
pub struct Radio {
    station_index: usize,
    station_list: Vec<String>,
    station_urls: Vec<String>,
}

impl Radio {
    pub fn new_with_index(stations: Vec<String>, station_urls: Vec<String>, idx: usize) -> Self {
        Self {
            station_index: idx,
            station_list: stations,
            station_urls,
        }
    }

    pub fn next(&mut self) {
        self.station_index = (self.station_index + 1) % self.station_list.len();
    }

    pub fn prev(&mut self) {
        if self.station_index == 0 {
            self.station_index = self.station_list.len() - 1
        } else {
            self.station_index -= 1;
        }
    }

    pub fn get_url(&self) -> String {
        self.station_urls[self.station_index].clone()
    }

    pub fn get_name(&self) -> String {
        self.station_list[self.station_index].clone()
    }
}
