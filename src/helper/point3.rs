#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3 { x, y, z }
    }
    pub fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::ops::Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Point3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl std::ops::Mul<i64> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: i64) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<Point3> for i64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl std::ops::MulAssign<i64> for Point3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl std::ops::Div<i64> for Point3 {
    type Output = Point3;

    fn div(self, rhs: i64) -> Self::Output {
        Point3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::DivAssign<i64> for Point3 {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl std::ops::Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl From<(i64, i64, i64)> for Point3 {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Point3 { x, y, z }
    }
}
impl Into<(i64, i64, i64)> for Point3 {
    fn into(self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}
