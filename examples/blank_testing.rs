use tiny_game_framework::{gl::*, glam::{vec2, Vec3, Vec4, vec3, vec4, Quat}, Cuboid, EventLoop, Light, Renderer, ShaderType, Sphere};

static GRAVITY: Vec3 = vec3(0., 0., 0.);

fn main() {
    let resolution = vec2(600., 600.);

    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();
    
    el.window.set_cursor_mode(tiny_game_framework::glfw::CursorMode::Disabled);

    unsafe {
        Enable(DEPTH_TEST);
        DepthFunc(LESS);
        Enable(BLEND);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    //let texture = tiny_game_framework::Texture::Path("src/images/grass.jpeg".into()); // not &¨str. i want String Ü
    
    let light = Light { position: vec3(10.0, 2.0, 5.0), color: Vec3::ONE*5.};
    renderer.add_light("l", light);

    let size = 10;
    let gap = 100;
    let mut counter = 0;
    for x in 0..size
    {
        for y in 0..size{
            counter += 1;

            let mut plane = Cuboid::new(Vec3::ONE*gap as f32, Vec4::ONE).mesh();
            plane.set_position(vec3(x as f32*gap as f32, 0., y as f32*gap as f32));
            //plane.set_texture(texture.clone());
            plane.set_shader_type(&ShaderType::Full);

            plane.setup_mesh();
            renderer.add_mesh(&format!("floor - {}", counter), plane).unwrap();
        }
    }

    while !el.window.should_close() {
        el.update();
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.update(renderer.camera.pos - GRAVITY*el.dt);
        
        //oooooooooouá la ui interface que morrestes nmutio brutalmente por goud deaudausl da sivla 🛹

        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            ClearColor(0.6, 0.6, 1., 1.);

            renderer.draw(&el);
        }
    }
}
