// A camera that hovers above the world and has a fixed look vector
// Interpolates between current position and target position

use nalgebra::{Point3,Vector3};

pub struct TopDownCamera
{
    pub current_position: Point3<f32>,
    pub look_direction: Vector3<f32>,
    pub move_speed_multi: f32,
    target_position: Point3<f32>
}

impl TopDownCamera {
    pub fn new(position: Point3<f32>, look: Vector3<f32>) -> Self 
    {
        TopDownCamera { 
            current_position: position, 
            look_direction: look.normalize(), 
            target_position: position,
            move_speed_multi: 2.0
        }
    }

    pub fn tick(&mut self, time_delta: f64)
    {
        const SLOWDOWN_DISTANCE: f32 = 16.0;
        let movement = self.target_position - self.current_position;
        let movement_mag = movement.magnitude();
        if movement_mag > 0.0
        {
            let slowdown_factor = (movement_mag.min(SLOWDOWN_DISTANCE) / SLOWDOWN_DISTANCE).max(0.02);   // clamp so it never gets to zero before reaching target
            let actual_speed = self.move_speed_multi * slowdown_factor;
            self.current_position = self.current_position + movement.normalize() * actual_speed * time_delta as f32;
        }
    }

    pub fn set_target(&mut self, target: Point3<f32>)
    {
        self.target_position = target;
    }

    pub fn apply_to_render_camera(&self, render_cam: &mut crate::render::camera::Camera)
    {
        // up direction does not need to be perfect, but it cannot converge on look direction
        let mut up_direction: Vector3<f32> = Vector3::y();
        if self.look_direction.y >= -1.01 && self.look_direction.y <= -0.99    // pretty much looking straigt down
        {
            up_direction = Vector3::z();
        }
        render_cam.look_at(
            self.current_position, 
            self.current_position + self.look_direction, 
            up_direction
        );
    }
}