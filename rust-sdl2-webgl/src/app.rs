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
    pub im_render_3d: ImmediateRender,
    pub im_render_2d: ImmediateRender,
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
        im_render_3d: ImmediateRender::new(gl, 1024 * 16),
        im_render_2d: ImmediateRender::new(gl, 1024 * 4),
        bg: 0.0, 
        time_elapsed: 0.0,
        particles: particles
    }
}

// main tick/update entry point
pub fn tick(state: &mut ApplicationState, input: &crate::input::InputState, delta_time: f64)
{
    state.im_render_3d.clear();
    state.im_render_2d.clear();

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
        state.im_render_3d.add_triangle(&p0, &particle.colour, &p1, &particle.colour, &p2, &particle.colour);
    }

    state.im_render_3d.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(1.0,0.0,0.0,1.0),
        &Point3::new(1.0, 0.0, 0.0), &Point4::new(1.0,0.0,0.0,1.0),
    );
    state.im_render_3d.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(0.0,1.0,0.0,1.0),
        &Point3::new(0.0, 1.0, 0.0), &Point4::new(0.0,1.0,0.0,1.0),
    );
    state.im_render_3d.add_line(
        &Point3::new(0.0, 0.0, 0.0), &Point4::new(0.0,0.0,1.0,1.0),
        &Point3::new(0.0, 0.0, 1.0), &Point4::new(0.0,0.0,1.0,1.0),
    );

    //if input.mouse_state.left_btn_down 
    {
        let mouse_x = input.mouse_state.position_x as f32;
        let mouse_y = input.mouse_state.position_y as f32;
        let p0 = Point3::new(mouse_x, mouse_y + 16.0, 0.1);
        let p1 = Point3::new(mouse_x - 16.0, mouse_y - 16.0, 0.1);
        let p2 = Point3::new(mouse_x + 16.0, mouse_y - 16.0, 0.1);
        let mut colour =  Point4::new(0.0,0.0,0.0,1.0);
        if input.mouse_state.left_btn_down {
            colour.x = 1.0;
        }
        if input.mouse_state.middle_btn_down {
            colour.y = 1.0;
        }
        if input.mouse_state.right_btn_down {
            colour.z = 1.0;
        }
        state.im_render_2d.add_triangle(&p0, &colour, &p1, &colour, &p2, &colour);
    }

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
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        gl.enable(glow::DEPTH_TEST);
        gl.depth_func(glow::LESS);
    }
    state.im_render_3d.draw(gl, &render_camera);

    let render_camera_2d = Camera::make_orthographic(0.0, viewport_width as f32, viewport_height as f32, 0.0, -1.0, 1.0);
    unsafe {
        gl.disable(glow::DEPTH_TEST);
    }
    state.im_render_2d.draw(gl, &render_camera_2d);
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
pub fn cleanup_gl_resources(gl : &glow::Context, state: &mut ApplicationState)
{
    state.im_render_3d.cleanup(gl);
}