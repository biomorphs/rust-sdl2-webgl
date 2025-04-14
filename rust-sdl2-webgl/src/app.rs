use glow::HasContext;  
use crate::render::camera::Camera;
use crate::render::immediate_render::ImmediateRender;
use nalgebra::{Point3, Point4, Vector3};

pub struct ApplicationState {
    pub im_render: ImmediateRender,
    pub bg: f32,
    pub time_elapsed: f64
}

// main init fn called once on start
pub fn init(gl : &glow::Context) -> ApplicationState
{
    ApplicationState {
        im_render: ImmediateRender::new(gl, 4096),
        bg: 0.0, 
        time_elapsed: 0.0
    }
}

// main tick/update entry point
pub fn tick(state: &mut ApplicationState, delta_time: f64)
{
    state.im_render.clear();

    state.im_render.add_triangle(
        &Point3::new(0.0, 3.0, 2.0), &Point4::new(1.0, 0.0, 0.0, 1.0),
        &Point3::new(1.0, 1.0, 2.0), &Point4::new(1.0, 0.0, 0.0, 1.0),
        &Point3::new(-1.0, 1.0, 2.0), &Point4::new(1.0, 0.0, 0.0, 1.0)
    );
    state.im_render.add_triangle(
        &Point3::new(2.0, 1.0, 1.0), &Point4::new(0.0, 1.0, 0.0, 1.0),
        &Point3::new(3.0, -1.0, 1.0), &Point4::new(0.0, 1.0, 0.0, 1.0),
        &Point3::new(1.0, -1.0, 1.0), &Point4::new(0.0, 1.0, 0.0, 1.0)
    );
    state.im_render.add_triangle(
        &Point3::new(4.0, 2.0, 0.0), &Point4::new(0.0, 0.0, 1.0, 1.0),
        &Point3::new(5.0, 0.0, 0.0), &Point4::new(0.0, 0.0, 1.0, 1.0),
        &Point3::new(3.0, 0.0, 0.0), &Point4::new(0.0, 0.0, 1.0, 1.0)
    );

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
    state.bg = 0.5 + (state.time_elapsed.sin() as f32) * 0.5;
}

// main update/drawing entry point
pub fn draw_gl(gl : &glow::Context, state: &ApplicationState,viewport_width: u32, viewport_height: u32)
{
    let aspect: f32 = viewport_width as f32 / viewport_height as f32;
    let mut render_camera = Camera::make_projection(0.1, 100.0, aspect, 90.0 );
    render_camera.look_at(Point3::new(2.0,1.0,5.0), Point3::new(0.0,0.0,0.0), Vector3::y());

    unsafe {
        gl.clear_color(state.bg, state.bg, state.bg, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);

        state.im_render.draw(gl, &render_camera);
    }
}

// cleanup function for desktop app
#[cfg(feature = "sdl2")]
pub fn cleanup_gl_resources(gl : &glow::Context, state: &mut ApplicationState)
{
    state.im_render.cleanup(gl);
}