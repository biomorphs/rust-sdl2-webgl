// Helpers to draw world-space grids

use nalgebra::{Point3,Point4,Vector3};
use crate::render::immediate_render::ImmediateRender;

pub fn draw_grid_xz(im_render: &mut ImmediateRender, origin: &Point3<f32>, dimensions: &Point3<f32>, step_size: f32, colour: &Point4<f32>)
{
    let steps_x = (dimensions.x / step_size).ceil() as i32;
    let steps_z = (dimensions.z / step_size).ceil() as i32;
    for z in 0..=steps_z 
    {
        let p0 = Point3::new(origin.x, origin.y, origin.z + step_size * z as f32);
        let p1 = p0 + Vector3::new(dimensions.x, 0.0, 0.0);
        im_render.add_line(&p0, &colour, &p1, &colour);
    }
    for x in 0..=steps_x 
    {
        let p0 = Point3::new(origin.x + step_size * x as f32, origin.y, origin.z);
        let p1 = p0 + Vector3::new(0.0, 0.0, dimensions.z);
        im_render.add_line(&p0, &colour, &p1, &colour);
    }
}