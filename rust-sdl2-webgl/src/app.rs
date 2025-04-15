use glow::HasContext;  
use crate::render::camera::Camera;
use crate::render::immediate_render::ImmediateRender;
use nalgebra::{Point3, Point4, Vector3};

struct SimpleParticle
{
    position: Point3<f32>,
    size: f32,
    velocity: Vector3<f32>,
    colour: Point4<f32>
}

impl SimpleParticle {
    fn new() -> Self
    {
        let position = Point3::new(0.0,0.0,0.0);
        let velocity = Vector3::new(
            rand::random_range(-2.0..2.0),
            rand::random_range(1.0..18.0),
            rand::random_range(-2.0..2.0)
        );
        let colour = Point4::new(
            rand::random_range(0.0..1.0),
            rand::random_range(0.0..1.0),
            rand::random_range(0.0..1.0),
            1.0
        );
        let size = rand::random_range(0.05..0.2);
        Self { position, size, velocity, colour }
    }
}

pub struct ApplicationState {
    pub im_render: ImmediateRender,
    pub bg: f32,
    pub time_elapsed: f64,
    particles: Vec<SimpleParticle> 
}

// main init fn called once on start
pub fn init(gl : &glow::Context) -> ApplicationState
{
    let mut particles = Vec::new();
    for _ in 1..4000 {
        particles.push( SimpleParticle::new() );
    }

    ApplicationState {
        im_render: ImmediateRender::new(gl, 1024 * 16),
        bg: 0.0, 
        time_elapsed: 0.0,
        particles: particles
    }
}

// main tick/update entry point
pub fn tick(state: &mut ApplicationState, delta_time: f64)
{
    state.im_render.clear();

    const GRAVITY: f64 = -9.8;
    for particle in &mut state.particles {
        particle.velocity.y = particle.velocity.y + (GRAVITY * delta_time) as f32;
        particle.position = particle.position + particle.velocity * delta_time as f32;
        particle.size = 0.05 + particle.position.y * 0.02;
        if particle.position.y < 0.0 {
            *particle = SimpleParticle::new();
        }

        let p0 = particle.position + Vector3::new(0.0,particle.size,0.0);
        let p1 = particle.position + Vector3::new(-particle.size,-particle.size,0.0);
        let p2 = particle.position + Vector3::new(particle.size,-particle.size,0.0);
        state.im_render.add_triangle(&p0, &particle.colour, &p1, &particle.colour, &p2, &particle.colour);
    }

    state.im_render.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(1.0,0.0,0.0,1.0),
        &Point3::new(1.0, 0.0, 0.0), &Point4::new(1.0,0.0,0.0,1.0),
    );
    state.im_render.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(0.0,1.0,0.0,1.0),
        &Point3::new(0.0, 1.0, 0.0), &Point4::new(0.0,1.0,0.0,1.0),
    );
    state.im_render.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(0.0,0.0,1.0,1.0),
        &Point3::new(0.0, 0.0, 1.0), &Point4::new(0.0,0.0,1.0,1.0),
    );

    state.time_elapsed += delta_time;
    state.bg = 0.2 + (0.5 + (state.time_elapsed.sin() as f32) * 0.5) * 0.4;
}

// main update/drawing entry point
pub fn draw_gl(gl : &glow::Context, state: &ApplicationState,viewport_width: u32, viewport_height: u32)
{
    let aspect: f32 = viewport_width as f32 / viewport_height as f32;
    let mut render_camera = Camera::make_projection(0.1, 100.0, aspect, 90.0 );
    render_camera.look_at(Point3::new(1.0,10.0,10.0), Point3::new(0.0,9.5,0.0), Vector3::y());

    unsafe {
        gl.viewport(0, 0, viewport_width as i32, viewport_height as i32);
        gl.clear_color(state.bg, state.bg, state.bg, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
    state.im_render.draw(gl, &render_camera);
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
pub fn cleanup_gl_resources(gl : &glow::Context, state: &mut ApplicationState)
{
    state.im_render.cleanup(gl);
}