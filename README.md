# Rust Game Framework

## Will eventually contain:
  * Utility for rendering (todo)
  * ECS (todo)
  * Rapier physics engine integration?
  * This list most likely update frequently as soon as i'm still active on this project 

As of now, the crate aims to prioritize simplicity.

Example usage:

```rust 
use tiny_game_framework::{quad_indices, quad_vertices, EventLoop, Renderer};

fn main() {
    let mut el = EventLoop::new();
    let mut renderer = Renderer::new();

    renderer.add_mesh("my mesh", quad_vertices(0.5, 0.5), quad_indices());

    while !el.window.should_close() {
        el.update();
        renderer.update();

        unsafe {
            renderer.draw();
        }
    }
}
```
