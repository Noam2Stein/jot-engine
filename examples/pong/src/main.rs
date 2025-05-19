use jot::window::*;

fn main() {
    run::<Pong>();
}

struct Pong {
    fs_switch: FullscreenSwitch,
}

impl App for Pong {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default().with_title("Pong")
    }

    fn new(_ctx: &AppContext) -> Self {
        Self {
            fs_switch: FullscreenSwitch::new(),
        }
    }

    fn event(&mut self, event: &Event, ctx: &AppContext) -> AppFlow {
        self.fs_switch.event(event, ctx);

        event.into()
    }
}
