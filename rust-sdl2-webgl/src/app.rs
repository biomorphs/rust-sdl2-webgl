use glow::HasContext;  
use crate::render::camera::Camera;
use crate::render::immediate_render::ImmediateRender;
use crate::render::grid_render::*;
use crate::top_down_camera::*;
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

    fn tick(particles: &mut Vec<SimpleParticle>, delta_time: f64, im_render: &mut ImmediateRender)
    {
        const GRAVITY: f64 = -9.8;
        for particle in particles {
            particle.velocity.y = particle.velocity.y + (GRAVITY * delta_time) as f32;
            particle.position = particle.position + particle.velocity * delta_time as f32;
            particle.size = 0.05 + particle.position.y * 0.02;
            if particle.position.y < 0.0 {
                *particle = SimpleParticle::new();
            }

            let p0 = particle.position + Vector3::new(0.0,particle.size,0.0);
            let p1 = particle.position + Vector3::new(-particle.size,-particle.size,0.0);
            let p2 = particle.position + Vector3::new(particle.size,-particle.size,0.0);
            im_render.add_triangle(&p0, &particle.colour, &p1, &particle.colour, &p2, &particle.colour);
        }
    }
}

pub struct ApplicationState {
    pub im_render_3d: ImmediateRender,
    pub im_render_2d: ImmediateRender,
    particles: Vec<SimpleParticle>,
    camera: TopDownCamera,
    render_camera_3d:Camera
}

// main init fn called once on start
pub fn init(gl : &glow::Context) -> ApplicationState
{
    let mut particles = Vec::new();
    for _ in 1..4000 {
        particles.push( SimpleParticle::new() );
    }

    let cam_start_pos = Point3::new(0.0, 25.0, -10.0);
    let cam_look_target = cam_start_pos + Vector3::new(0.0, -20.0, 5.0);

    ApplicationState {
        im_render_3d: ImmediateRender::new(gl, 1024 * 32),
        im_render_2d: ImmediateRender::new(gl, 1024 * 4),
        particles: particles,
        camera: TopDownCamera::new(
            cam_start_pos, 
            cam_look_target - cam_start_pos),
        render_camera_3d: Camera::make_projection(0.1, 100.0, 1.0, 90.0)
    }
}

// main tick/update entry point
pub fn tick(state: &mut ApplicationState, input: &crate::input::InputState, delta_time: f64, viewport_width: u32, viewport_height: u32)
{
    state.im_render_3d.clear();
    state.im_render_2d.clear();

    SimpleParticle::tick(&mut state.particles, delta_time, &mut state.im_render_3d);

    // top-down camera input update
    if input.mouse_state.left_btn_down 
    {
        let mouse_x = input.mouse_state.position_x as f32;
        let mouse_y = input.mouse_state.position_y as f32;
        let mouse_delta_x = (viewport_width as f32 / 2.0) - mouse_x;
        let mouse_delta_z = (viewport_height as f32 / 2.0) - mouse_y;

        // camera movement, if clicked outside center of screen, move in that direction
        const MIN_SCREEN_DISTANCE: f32 = 0.4;   // min distance away from screen center before camera moves
        const CAM_MOVE_DISTANCE_MUL: f32 = 0.06;    // how far to move in world, multiplied by mouse distance to center
        const CAM_MOVE_SPEED_MOUSE_MUL: f32 = 0.1;  // speed = mouse dist to center * this
        const CAM_MAX_MOVE_SPEED: f32 = 28.0;       // world space units/s
        let screen_edge_distance = viewport_width.min(viewport_height) as f32 * 0.5;    // distance to edge of screen
        let screen_center_to_mouse = Vector3::new(mouse_delta_x, 0.0, mouse_delta_z);
        let cam_move_magnitude = (screen_center_to_mouse.magnitude() - screen_edge_distance * MIN_SCREEN_DISTANCE).max(0.0);  
        if cam_move_magnitude > 0.0  
        {
            // move camera based on click direction from center
            let cam_move_target = screen_center_to_mouse.normalize() * cam_move_magnitude * CAM_MOVE_DISTANCE_MUL;

            // scale camera move speed as a proportion of distance of mouse to center of window
            state.camera.move_speed_multi = (cam_move_magnitude * CAM_MOVE_SPEED_MOUSE_MUL).min(CAM_MAX_MOVE_SPEED);
            state.camera.set_target(state.camera.current_position + cam_move_target);
        }
    }
    state.camera.tick(delta_time);

    // update render camera
    let aspect: f32 = viewport_width as f32 / viewport_height as f32;
    state.render_camera_3d = Camera::make_projection(0.1, 100.0, aspect, 90.0);
    state.camera.apply_to_render_camera(&mut state.render_camera_3d);

    draw_grid_xz(&mut state.im_render_3d, 
        &Point3::new(-128.0, 0.0, -128.0), 
        &Point3::new(256.0,0.0,256.0), 
        2.0, 
        &Point4::new(0.7,0.7,0.7,1.0));
}

// main update/drawing entry point
pub fn draw_gl(gl : &glow::Context, state: &ApplicationState,viewport_width: u32, viewport_height: u32)
{
    unsafe {
        gl.viewport(0, 0, viewport_width as i32, viewport_height as i32);
        gl.clear_color(0.3, 0.3, 0.35, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        gl.enable(glow::DEPTH_TEST);
        gl.depth_func(glow::LESS);
    }
    state.im_render_3d.draw(gl, &state.render_camera_3d);

    // 2d stuff always uses ortho projection matching viewport size
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
    state.im_render_2d.cleanup(gl);
}