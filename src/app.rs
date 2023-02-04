use piston::{
    CloseArgs, CloseEvent, Event, EventSettings, Events, Input, RenderArgs, RenderEvent,
    UpdateArgs, UpdateEvent, Window,
};

pub trait App<'a, W: Window> {
    /// Responsible for things like saving app data
    fn close(self: Box<Self>, args: &CloseArgs);

    fn render(&mut self, args: &RenderArgs);

    fn update(&mut self, args: &UpdateArgs);

    fn window(&mut self) -> &mut W;

    fn input_event(&mut self, input: Input);

    fn events(&self) -> Events {
        Events::new(EventSettings::new())
    }

    /// Run the app until it is exited
    fn run(mut self: Box<Self>) {
        let mut events = self.events();
        while let Some(e) = events.next(self.window()) {
            if let Some(args) = e.close_args() {
                self.close(&args);
                break;
            }

            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }

            match e {
                Event::Input(input, _i) => {
                    self.input_event(input);
                }
                _ => {}
            }
        }
    }
}
