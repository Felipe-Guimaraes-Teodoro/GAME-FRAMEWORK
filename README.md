# Rust Game Framework

## Will eventually contain:
  * Utility for rendering (todo)
  * ECS (todo)
  * Rapier physics engine integration(collisions, rigidbodies, etc)?
  * This list most likely update frequently as soon as i'm still active on this project 

As of now, the crate aims to prioritize simplicity.

## A simple 3D scene:

```rust 
use tiny_game_framework::{
    glam::{vec2, vec3, vec4, Vec4},
    gl::{Clear, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT},
    EventLoop,
    Renderer,
    Cuboid,
};

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let c = Cuboid::new(vec3(600., 600., 600.0), vec4(1.0, 1.0, 1.0, 1.0)).mesh();
    renderer.add_mesh("c", c).unwrap();

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);

    while !el.window.should_close() {
        el.update();
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.update(renderer.camera.pos);

        let frame = el.ui.frame(&mut el.window);
        frame.text("hello, world!");

        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            renderer.draw(&el);
            el.ui.draw();
        }
    }
}
```
