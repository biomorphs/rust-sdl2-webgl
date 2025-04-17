use nalgebra::{Perspective3, Orthographic3, Isometry3, Point3, Vector3, Matrix4};

enum ProjectionTransform {
    Perspective { transform: Perspective3<f32> },
    Orthographic { transform: Orthographic3<f32> }
}

// A camera used for rendering
pub struct Camera
{
    // camera eye position/target/up
    position: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,

    // view transform caculate from values above
    view_transform: Isometry3<f32>,

    // perspective / ortho projection
    projection_transform: ProjectionTransform
}

impl Camera {

    fn rebuild_view_matrix(&mut self)
    {
        self.view_transform = Isometry3::look_at_rh(&self.position, &self.target, &self.up);
    }

    pub fn look_at(&mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>)
    {
        self.position = eye;
        self.target = target;
        self.up = up;
        self.rebuild_view_matrix();
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4<f32>
    {
        let view_matrix = self.view_transform.to_homogeneous(); // convert to 4x4
        match self.projection_transform {
            ProjectionTransform::Perspective { transform, .. } => {
                transform.as_matrix() * view_matrix 
            },
            ProjectionTransform::Orthographic { transform,.. } => {
                transform.as_matrix() * view_matrix
            }
        }
    }

    pub fn make_projection(near: f32, far: f32, aspect: f32, fov: f32) -> Self
    {
        let position = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0,0.0,-1.0);
        let up = Vector3::y();
        Camera {
            position: position,
            target: target,
            up: up,
            view_transform: Isometry3::look_at_rh(&position, &target, &up),
            projection_transform: ProjectionTransform::Perspective {
                transform: Perspective3::new(aspect, fov, near, far) 
            }
        }
    }

    pub fn make_orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self
    {
        let position = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0,0.0,-1.0);
        let up = Vector3::y();
        Camera {
            position: position,
            target: target,
            up: up,
            view_transform: Isometry3::look_at_rh(&position, &target, &up),
            projection_transform: ProjectionTransform::Orthographic {
                transform: Orthographic3::new(left, right, bottom, top, near, far)
            }
        }
    }

    pub fn get_view_transform(&self) -> Isometry3<f32>
    {
        self.view_transform
    }

}