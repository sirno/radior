#[derive(Clone)]
pub struct Radio {
    station_name: String,
    station_index: usize,
    station_url: String,
    station_list: Vec<String>,
    station_urls: Vec<String>,
}

impl Radio {
    pub fn new(stations: Vec<String>, station_urls: Vec<String>) -> Self {
        return Self::new_with_index(stations, station_urls, 0);
    }

    pub fn new_with_index(stations: Vec<String>, station_urls: Vec<String>, idx: usize) -> Self {
        return Self {
            station_name: stations[idx].clone(),
            station_index: idx,
            station_url: station_urls[idx].clone(),
            station_list: stations,
            station_urls: station_urls,
        };
    }
    fn update_station(&mut self) {
        self.station_name = self.station_list[self.station_index].clone();
        self.station_url = self.station_urls[self.station_index].clone();
    }

    pub fn next(&mut self) {
        self.station_index = (self.station_index + 1) % self.station_list.len();
        self.update_station()
    }

    pub fn prev(&mut self) {
        if self.station_index == 0 {
            self.station_index = self.station_list.len() - 1
        } else {
            self.station_index -= 1;
        }
        self.update_station();
    }

    pub fn get_url(&self) -> String {
        return self.station_url.clone();
    }
}
