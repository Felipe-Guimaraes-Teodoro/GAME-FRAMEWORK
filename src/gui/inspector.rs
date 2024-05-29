use imgui::Ui;

use crate::Renderer;

pub fn renderer_inspector(renderer: &mut Renderer, frame: &mut Ui) {
    let inspector_window = frame.window("Inspector");

    inspector_window.build(|| {
        for mesh in &renderer.meshes {
            let txt = format!("Mesh: {:?}; P: {:.1}; R: {:.1}; S: {:.1}", mesh.0, mesh.1.position, mesh.1.rotation, mesh.1.scale);
            frame.text(txt);
        }

        for light in &renderer.lights {
            let txt = format!("Light: {:?}; P: {:.1}; C: {:.1}", light.0, light.1.position, light.1.color);
            frame.text(txt);
        }

    });
}