use sfml::window::{Window, Event, Style};

struct GameWindow {
    window: Window
}

impl GameWindow {
    fn new() -> GameWindow {
        let mut window = Window::new((800, 600),
        "SFML window",
        Style::CLOSE,
        &Default::default());

        window.set_framerate_limit(60);

        GameWindow {
            window
        }
    }

    fn run(mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed {
                    self.window.close();
                }
            }
        
            self.window.set_active(true);
        
            // OpenGL drawing commands go here...
        
            self.window.display();
        }
    }
}