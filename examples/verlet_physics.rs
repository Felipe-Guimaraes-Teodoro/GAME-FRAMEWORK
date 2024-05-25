use tiny_game_framework::{lerp, rand_betw, EventLoop, Renderer};
use tiny_game_framework::Circle;
use tiny_game_framework::glam::*;
use tiny_game_framework::gl;

static GRAVITY: Vec3 = Vec3 {
    x: 0.0,
    y: -5.0,
    z: 0.0,
};

fn main() {
    let mut el = EventLoop::new(500, 500);
    let mut renderer = Renderer::new();

    let mut world = World::new(&mut renderer);

    let mut dt = 0.0;
    while !el.window.should_close() {
        let now = std::time::Instant::now();
        el.update();

        world.update(&mut renderer, dt, &el);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            renderer.draw(&el);
        }

        dt = now.elapsed().as_secs_f32();
    }
}

struct World {
    particles: Vec<Particle>,
}

impl World {
    pub fn new(renderer: &mut Renderer) -> Self {
        let mut particles = vec![];
        for i in 0..63 {
            let mut p1 = Particle::new(Vec3::ZERO, &i.to_string(), renderer, 10.0);
            p1.p = vec3(rand_betw(-250.0, 250.0), rand_betw(-250.0, 250.0), 0.0);
            p1.op = p1.p - vec3(rand_betw(-1.0, 1.0), rand_betw(-1.0, 1.0), 0.0);

            particles.push(p1);
        }

        Self {
            particles,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer, dt: f32, el: &EventLoop) {
        let mut collision_pairs: Vec<[usize; 2]> = vec![];
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if Self::detect_collision(&self.particles[i], &self.particles[j]) {
                    collision_pairs.push([i, j]);
                }
            }
        }

        for pair in &collision_pairs {
            Self::resolve_collision(&mut self.particles, pair[0], pair[1]);
        }
    
        for particle in self.particles.iter_mut() {
            particle.update(dt);
            particle.update_mesh(renderer);
            particle.constrain(el);
        }
    }

    pub fn detect_collision(collider: &Particle, collide: &Particle) -> bool {
        collider.check_with(collide)
    }

    pub fn resolve_collision(particles: &mut Vec<Particle>, i: usize, j: usize) {
        if i == j {
            return;
        }

        let (left, right) = if i < j {
            particles.split_at_mut(j)
        } else {
            particles.split_at_mut(i)
        };

        let (p1, p2) = if i < j {
            (&mut left[i], &mut right[0])
        } else {
            (&mut right[0], &mut left[j])
        };

        let delta = p2.p - p1.p;
        let distance = delta.length();

        let overlap = (p1.r + p2.r) - distance;

        if overlap > 0.0 {
            let delta_norm = delta / distance;

            let adjustment = delta_norm * (overlap / 2.0);
            p1.p -= adjustment;
            p2.p += adjustment;

            std::mem::swap(&mut p1.v, &mut p2.v);
        }
    }
}

struct Particle {
    op: Vec3,  
    p: Vec3,   
    v: Vec3,   

    r: f32,

    name: String,
}

impl Particle {
    pub fn new(p: Vec3, name: &str, renderer: &mut Renderer, radius: f32) -> Self {
        let n = Vec3::ZERO;

        let circle = Circle::new(7, radius / 250.0, vec4(rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), 1.0));
        renderer.add_mesh(name, circle.mesh()).unwrap();
        renderer.get_mesh_mut(name).unwrap().rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rand_betw(0.0, 2.0*std::f32::consts::PI));

        Self {
            p,
            op: p,
            v: n,

            r: radius,

            name: name.to_string(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.v = self.p - self.op;
        self.op = self.p;
        self.p += self.v + GRAVITY * dt;
    }

    pub fn check_with(&self, particle: &Particle) -> bool {
        if self.p.distance(particle.p) < self.r * 2.0 {
            true
        } else {
            false
        }
    }

    pub fn constrain(&mut self, el: &EventLoop) {
        let (w, h) = el.window.get_framebuffer_size();
        let (hw, hh) = (w as f32 / 2.0, h as f32 / 2.0);

        if self.p.x >= hw - self.r || self.p.x <= -hw + self.r {
            self.p.x = self.p.x.clamp(-hw + self.r, hw - self.r);
            self.v.x = -self.v.x * 0.8;
            self.op.x = self.p.x - self.v.x;
        }

        if self.p.y >= hh - self.r || self.p.y <= -hh + self.r {
            self.p.y = self.p.y.clamp(-hh + self.r, hh - self.r);
            self.v.y = -self.v.y * 0.8;
            self.op.y = self.p.y - self.v.y;
        }
    }

    pub fn update_mesh(&self, renderer: &mut Renderer) {
        if let Some(mesh) = renderer.get_mesh_mut(&self.name) {
            mesh.position.x = lerp(mesh.position.x, self.p.x, 0.5);
            mesh.position.y = lerp(mesh.position.y, self.p.y, 0.5);
        }
    }
}
