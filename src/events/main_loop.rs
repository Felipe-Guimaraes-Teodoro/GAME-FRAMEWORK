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
    pub fn new(w: u32, h: u32) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    
        let (mut window, events) = glfw.create_window(w, h, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    
        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|s| window.get_proc_address(s) );
    
        let mut event_handler = EventHandler::new();
        event_handler.on_window_resize(w as i32, h as i32);

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
            // println!("{:?}", &self.event_handler.keys_pressed);
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
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.event_handler.on_mouse_move(x, y);
                }
                glfw::WindowEvent::FramebufferSize(w, h) => {
                    self.event_handler.on_window_resize(w, h);
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

