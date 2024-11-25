use nalgebra::{Point3, Vector3};

use super::{intersection::Intersection, primatives::sphere::Intersect};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }

    pub fn intersect<T: Intersect<T>>(&self, shape: &T) -> Vec<Intersection<T>> {
        shape.intersect(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let origin = Point3::new(2.0, 3.0, 4.0); // Example origin
        let direction = Vector3::new(1.0, 0.0, 0.0); // Example direction
        let ray = Ray::new(origin, direction);

        // Test position at t = 0
        let t = 0.0;
        let expected_position = Point3::new(2.0, 3.0, 4.0);
        assert_eq!(ray.position(t), expected_position);

        // Test position at t = 1
        let t = 1.0;
        let expected_position = Point3::new(3.0, 3.0, 4.0);
        assert_eq!(ray.position(t), expected_position);

        // Test position at t = -1
        let t = -1.0;
        let expected_position = Point3::new(1.0, 3.0, 4.0);
        assert_eq!(ray.position(t), expected_position);

        // Test position at t = 2.5
        let t = 2.5;
        let expected_position = Point3::new(4.5, 3.0, 4.0);
        assert_eq!(ray.position(t), expected_position);
    }

}