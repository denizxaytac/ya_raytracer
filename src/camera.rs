use crate::vec3::{Vec3, Point3, Color};
use crate::ray::{Ray};
use crate::sphere::{Sphere};
use crate::hittable::{HitRecord, Hittable, HittableList};

pub struct Camera{
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera{
    pub fn new() -> Camera{
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        let orig = Point3::new(0.0, 0.0, 0.0);
        let hor = Vec3::new(viewport_width, 0.0, 0.0);
        let ver = Vec3::new(0.0, viewport_height, 0.0);

        return Camera{
            origin: orig,
            horizontal: hor,
            vertical: ver,
            lower_left_corner: orig - hor / 2.0 - ver / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        };

    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        return Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin);
    }
}