use std::{fmt, ops::{Add, Div, Mul, Neg, Sub}};


use wide::*;

#[derive(Copy, Clone, PartialEq)]
struct Tuple {
    data: f64x4,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple {
            data: f64x4::new([x, y, z, w]),
        }
    }

    pub fn is_vector(&self) -> bool {
        self.w() == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }
    
    pub fn magnitude(&self) -> f64 {
        (self.data * self.data).reduce_add().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple {
            data: self.data / f64x4::splat(magnitude),
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        (self.data * other.data).reduce_add()
    }

    pub fn cross(&self, other: Self) -> Self{
        // Create vectors that represent the required terms in the cross product formula
        let a_yzx = f64x4::new([self.y(), self.z(), self.x(), 0.0]);
        let b_yzx = f64x4::new([other.y(), other.z(), other.x(), 0.0]);
        
        let a_zxy = f64x4::new([self.z(), self.x(), self.y(), 0.0]);
        let b_zxy = f64x4::new([other.z(), other.x(), other.y(), 0.0]);
        
        // Calculate cross product terms using element-wise multiplication and subtraction
        let mut result = a_yzx * b_zxy - a_zxy * b_yzx;
        result.as_array_mut()[3] = 0.0;
        Tuple {
            data: result
        }
    }

    pub fn approx_eq(&self, other: Self) -> bool {
        let epsilon = 0.00001;
        (self.data - other.data).abs().cmp_lt(f64x4::splat(epsilon)).all()
    }

    // Getters
    pub fn x(&self) -> f64 {
        self.data.as_array_ref()[0]
    }

    pub fn y(&self) -> f64 {
        self.data.as_array_ref()[1]
    }

    pub fn z(&self) -> f64 {
        self.data.as_array_ref()[2]
    }

    pub fn w(&self) -> f64 {
        self.data.as_array_ref()[3]
    }

}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple {
            data: self.data + other.data,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple {
            data: self.data - other.data,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Tuple {
            data: self.data * f64x4::splat(scalar),
        }
    }
}

impl Tuple {
    fn mul_tuple(self, other: Self) -> Self {
        Tuple {
            data: self.data * other.data,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Tuple {
            data: self.data / f64x4::splat(scalar),
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            data: self.data.neg(),
        }
    }
}


impl fmt::Debug for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.as_array_ref();
        f.debug_struct("Tuple")
            .field("x", &data[0])
            .field("y", &data[1])
            .field("z", &data[2])
            .field("w", &data[3])
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point() {
        let p = Tuple::new_point(1.0, 2.0, 3.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn test_is_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(v.is_vector());
        assert!(!v.is_point());
    }

    #[test]
    fn test_add() {
        let p = Tuple::new_point(3.0, -2.0, 5.0);
        let v = Tuple::new_vector(-2.0, 3.0, 1.0);
        assert!(p + v == Tuple::new_point(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_subtract() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        assert!(p1 - p2 == Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_negate() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert!(-v == Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert!(v * 3.5 == Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_scalar_division() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert!(v / 2.0 == Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_magnitude() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::new_vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());

        let v = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        assert!(v.normalize() == Tuple::new_vector(1.0, 0.0, 0.0));

        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(v.normalize().approx_eq(Tuple::new_vector(0.26726, 0.53452, 0.80178)));

        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        assert!(v.normalize().magnitude() == 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!(a.dot(b) == 20.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert!(a.cross(b).approx_eq(Tuple::new_vector(-1.0, 2.0, -1.0)));
        assert!(b.cross(a).approx_eq(Tuple::new_vector(1.0, -2.0, 1.0)));
    }
}