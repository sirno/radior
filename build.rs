extern crate pkg_config;

fn main() {
    match pkg_config::probe_library("mpv") {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
