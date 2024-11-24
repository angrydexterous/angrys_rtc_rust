use nalgebra::Point3;
use crate::rtc::ray::Ray;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Point3::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_intersect_sphere_middle() {
        let origin = Point3::new(0.0, 0.0, -5.0); 
        let direction = Vector3::new(0.0, 0.0, 1.0); 
        let ray = Ray::new(origin, direction);

        let sphere = Sphere{};

        let expected_intersections = vec![4.0, 6.0];
        assert_eq!(ray.intersect(&sphere), expected_intersections);
    }

    #[test]
    fn test_intersect_sphere_edge() {
        let origin = Point3::new(0.0, 1.0, -5.0); 
        let direction = Vector3::new(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere{};

        let expected_intersections = vec![5.0, 5.0];
        assert_eq!(ray.intersect(&sphere), expected_intersections);
    }
}