# Rust Game Framework

## Will eventually contain:
  * Utility for rendering (todo)
  * ECS (todo)
  * Rapier physics engine integration?
  * This list most likely update frequently as soon as i'm still active on this project 

As of now, the crate aims to prioritize simplicity.

Example usage:

```rust 
use tiny_game_framework::{Circle, EventLoop, Renderer, Vector3D};

fn main() {
    let mut el = EventLoop::new();
    let mut renderer = Renderer::new();

    let c = Circle::new(16, Vector3D::ZERO, 0.1, Vector4D::new(0., 0., 0., 0.));
    c.add_to_renderer("my mesh", &mut renderer);

    while !el.window.should_close() {
        el.update();
        renderer.update();

        unsafe {
            renderer.draw();
        }
    }
}
```
