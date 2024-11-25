#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection<T> {
    pub t: f64,
    pub object: T
}

impl<T> Intersection<T> {
    pub fn new(t:f64, object:T) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &T  {
        &self.object
    }
}

#[derive(Debug)]
pub struct Intersections<T> {
    pub intersections: Vec<Intersection<T>>
}

impl<T> Intersections<T>{
    pub fn new() -> Self {
        Self { intersections: Vec::<Intersection<T>>::with_capacity(16) }
    }

    pub fn with_intersection(mut self, intersection: Intersection<T>) -> Self {
        self.intersections.push(intersection);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::rtc::primatives::sphere::Sphere;

    use super::*;

    #[test]
    fn test_intersection() {
        let sphere = Sphere{};
        let intersection = Intersection::new(3.5, sphere);
        assert_eq!(intersection.t(), 3.5);
        assert_eq!(*intersection.object(), sphere);
    }

    #[test]
    fn test_intersections() {
        let sphere = Sphere{};

        let intersections = Intersections::new()
            .with_intersection(Intersection::new(1.0, sphere))
            .with_intersection(Intersection::new(2.0, sphere));

        assert_eq!(intersections.intersections.len(), 2);
        assert_eq!(intersections.intersections[0].t, 1.0);
        assert_eq!(intersections.intersections[1].t, 2.0);
    }
}