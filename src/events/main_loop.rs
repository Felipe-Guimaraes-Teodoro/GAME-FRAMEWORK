extern crate glfw;   

use glfw::{fail_on_errors, Glfw, GlfwReceiver, PWindow, WindowEvent};
use glfw::{Action, Context, Key};

use super::EventHandler;

pub struct EventLoop {
    pub event_handler: EventHandler,
    pub window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl EventLoop {
    pub fn new() -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    
        let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    
        window.make_current();
        window.set_key_polling(true);
    
        let mut event_handler = EventHandler::new();

        Self {
            event_handler,
            window,
            glfw,
            events,
        }
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
    
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", &self.event_handler.keys_pressed);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                glfw::WindowEvent::Key(key, _, Action::Press, _ ) => {
                    self.event_handler.on_key_press(key);
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _ ) => {
                    self.event_handler.on_key_release(key);
                }
                _ => {},
            }
        }
    }

    pub fn is_key_down(&mut self, key: Key) -> bool {
        if self.window.get_key(key) ==Action::Press {
            true
        } else { 
            false 
        }
    }
}

