use piston::{
    CloseArgs, CloseEvent, Event, EventSettings, Events, Input, RenderArgs, RenderEvent,
    ResizeArgs, ResizeEvent, UpdateArgs, UpdateEvent, Window,
};

/// A trait to simplify some of the boilerplate in running an app
pub trait App<W: Window> {
    /// Responsible for things like saving app data
    fn close(self: Box<Self>, args: &CloseArgs);

    /// Runs the render loop
    fn render(&mut self, args: &RenderArgs);

    /// Runs the fixed time update loop
    fn update(&mut self, args: &UpdateArgs);

    /// Controls the window resize event
    fn resize(&mut self, args: &ResizeArgs);

    /// A mutable reference to the app's window
    fn window(&mut self) -> &mut W;

    /// Controls input events
    fn input_event(&mut self, input: Input);

    /// The [Events] to be used for running the app. Override to set custom event settings
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

            if let Some(args) = e.resize_args() {
                self.resize(&args);
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
