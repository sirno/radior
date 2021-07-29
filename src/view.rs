use super::mpv::Mpv;
use super::radio::Radio;
use cursive::event::{Event, EventResult};
use cursive::view::{View, ViewWrapper};
use cursive::views::{TextContent, TextView};

pub struct PlayerView<T: View> {
    view: T,
    content: TextContent,
    mpv: Mpv,
}

impl PlayerView<TextView> {
    pub fn new() -> Self {
        let mut mpv = Mpv::new();
        let content = TextContent::new(mpv.get_streamstate().get_display());
        let view = TextView::new_with_content(content.clone());
        return Self {
            view: view,
            content: content,
            mpv: mpv,
        };
    }

    pub fn new_with_url(url: String) -> Self {
        let mut pv = Self::new();
        pv.mpv.loadfile(url.as_str());
        return pv;
    }
}

pub struct RadioView<T: View> {
    player_view: T,
    radio: Radio,
}

impl RadioView<PlayerView<TextView>> {
    pub fn new(radio: Radio) -> Self {
        let mut player_view = PlayerView::new();
        player_view.mpv.loadfile(radio.get_url().as_str());

        return Self {
            player_view: player_view,
            radio: radio,
        };
    }
}

impl<T: View> ViewWrapper for PlayerView<T> {
    cursive::wrap_impl!(self.view: T);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('=') => {
                self.mpv.add_property("volume", 2);
                return EventResult::Consumed(None);
            }
            Event::Char('-') => {
                self.mpv.add_property("volume", -2);
                return EventResult::Consumed(None);
            }
            Event::Char('n') => {
                self.mpv.playlist_next();
                return EventResult::Consumed(None);
            }
            Event::Char('p') => {
                self.mpv.playlist_prev();
                return EventResult::Consumed(None);
            }
            Event::Refresh => {
                self.content
                    .set_content(self.mpv.get_streamstate().get_display());
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
                self.player_view.mpv.loadfile(self.radio.get_url().as_str());
                return EventResult::Consumed(None);
            }
            Event::Char(',') => {
                self.radio.prev();
                self.player_view.mpv.loadfile(self.radio.get_url().as_str());
                return EventResult::Consumed(None);
            }
            _ => self.player_view.on_event(event),
        }
    }
}