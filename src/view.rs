use super::radio::Radio;
use super::stream_state::StreamState;

use cursive::event::{Event, EventResult};
use cursive::traits::Nameable;
use cursive::view::{View, ViewWrapper};
use cursive::views::{Dialog, TextContent, TextView};
use libmpv::{FileState, Mpv};

pub struct PlayerView<T: View> {
    view: T,
    content: TextContent,
    mpv: Mpv,
}

impl PlayerView<TextView> {
    pub fn new() -> Result<Self, libmpv::Error> {
        let mpv = Mpv::new()?;
        let content = TextContent::new(mpv.get_display());
        let view = TextView::new_with_content(content.clone());
        Ok(Self {
            view: view,
            content: content,
            mpv: mpv,
        })
    }

    pub fn new_with_url(url: String) -> Result<Self, libmpv::Error> {
        match Self::new() {
            Ok(player) => {
                player
                    .mpv
                    .playlist_load_files(&[(url.as_str(), FileState::AppendPlay, None)])?;
                Ok(player)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct RadioView<T: View> {
    player_view: T,
    radio: Radio,
}

impl RadioView<PlayerView<TextView>> {
    pub fn new(radio: Radio) -> Result<Self, libmpv::Error> {
        match PlayerView::new_with_url(radio.get_url()) {
            Ok(player_view) => Ok(Self {
                player_view: player_view,
                radio: radio,
            }),
            Err(e) => Err(e),
        }
    }
}

impl<T: View> ViewWrapper for PlayerView<T> {
    cursive::wrap_impl!(self.view: T);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('=') => {
                self.mpv.add_property("volume", 2).unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char('-') => {
                self.mpv.add_property("volume", -2).unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char('<') => {
                self.mpv.playlist_previous_weak().unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char('>') => {
                self.mpv.playlist_next_weak().unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char('p') => {
                match self.mpv.get_property("pause") {
                    Ok(pause) => {
                        if pause {
                            self.mpv.unpause().unwrap();
                        } else {
                            self.mpv.pause().unwrap();
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
                return EventResult::Consumed(None);
            }
            Event::Refresh => {
                self.content.set_content(self.mpv.get_display());
                return self.view.on_event(event);
            }
            _ => self.view.on_event(event),
        }
    }
}

impl<T: View> ViewWrapper for RadioView<PlayerView<T>> {
    cursive::wrap_impl!(self.player_view: PlayerView<T>);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('.') => {
                self.radio.next();
                self.player_view
                    .mpv
                    .playlist_load_files(&[(
                        self.radio.get_url().as_str(),
                        FileState::Replace,
                        None,
                    )])
                    .unwrap();
                return EventResult::Consumed(None);
            }
            Event::Char(',') => {
                self.radio.prev();
                self.player_view
                    .mpv
                    .playlist_load_files(&[(
                        self.radio.get_url().as_str(),
                        FileState::Replace,
                        None,
                    )])
                    .unwrap();
                return EventResult::Consumed(None);
            }
            _ => self.player_view.on_event(event),
        }
    }
}

fn print_player_help() -> String {
    r###"
General
- Decrease volume
= Increase volume
> Next track
< Previous track
"###
    .to_string()
}

fn print_radio_help() -> String {
    let radio_bindings = r###"
Radio
, Previous station
. Next Station
"###;
    let mut bindings = print_player_help().to_owned();
    bindings.push_str(radio_bindings);
    return bindings;
}

fn print_general_help() -> String {
    r###"
Misc
q Exit
? Toggle this help menu
"###
    .to_string()
}

pub fn playerhelp() -> Dialog {
    let mut bindings = print_player_help().to_owned();
    bindings.push_str(print_general_help().as_str());
    Dialog::around(TextView::new(bindings.as_str()).with_name("help_view")).title("Help")
}

pub fn radiohelp() -> Dialog {
    let mut bindings = print_radio_help().to_owned();
    bindings.push_str(print_general_help().as_str());
    Dialog::around(TextView::new(bindings.as_str()).with_name("help_view")).title("Help")
}
