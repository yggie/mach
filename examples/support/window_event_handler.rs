extern crate glium;

use self::glium::glutin;

pub enum EventResponse {
    Bubble,
    Consumed,
}

pub trait WindowEventHandler {
    fn handle_event(&mut self, event: &glutin::Event) -> EventResponse;
}
