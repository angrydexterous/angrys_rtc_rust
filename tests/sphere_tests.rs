use nalgebra::{Point3, Vector3};
use rtc::rtc::{primatives::sphere::Sphere, ray::Ray};

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
