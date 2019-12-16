#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub v: [i64; 3],
}

impl Point3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3 { v: [x, y, z] }
    }
    pub fn manhattan(&self) -> i64 {
        self.v[0].abs() + self.v[1].abs() + self.v[2].abs()
    }
    pub fn x(&self) -> i64 {
        self.v[0]
    }
    pub fn y(&self) -> i64 {
        self.v[1]
    }
    pub fn z(&self) -> i64 {
        self.v[2]
    }
    pub fn x_mut(&mut self) -> &mut i64 {
        &mut self.v[0]
    }
    pub fn y_mut(&mut self) -> &mut i64 {
        &mut self.v[1]
    }
    pub fn z_mut(&mut self) -> &mut i64 {
        &mut self.v[2]
    }
}

impl std::ops::Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3 {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
            ],
        }
    }
}
impl std::ops::AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Point3) {
        self.v[0] += rhs.v[0];
        self.v[1] += rhs.v[1];
        self.v[2] += rhs.v[2];
    }
}
impl std::ops::Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Point3 {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
            ],
        }
    }
}
impl std::ops::SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Point3) {
        self.v[0] -= rhs.v[0];
        self.v[1] -= rhs.v[1];
        self.v[2] -= rhs.v[2];
    }
}
impl std::ops::Mul<i64> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: i64) -> Self::Output {
        Point3 {
            v: [self.v[0] * rhs, self.v[1] * rhs, self.v[2] * rhs],
        }
    }
}
impl std::ops::Mul<Point3> for i64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3 {
            v: [self * rhs.v[0], self * rhs.v[1], self * rhs.v[2]],
        }
    }
}
impl std::ops::MulAssign<i64> for Point3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.v[0] *= rhs;
        self.v[1] *= rhs;
        self.v[2] *= rhs;
    }
}
impl std::ops::Div<i64> for Point3 {
    type Output = Point3;

    fn div(self, rhs: i64) -> Self::Output {
        Point3 {
            v: [self.v[0] / rhs, self.v[1] / rhs, self.v[2] / rhs],
        }
    }
}
impl std::ops::DivAssign<i64> for Point3 {
    fn div_assign(&mut self, rhs: i64) {
        self.v[0] /= rhs;
        self.v[1] /= rhs;
        self.v[2] /= rhs;
    }
}
impl std::ops::Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        Point3 {
            v: [-self.v[0], -self.v[1], -self.v[2]],
        }
    }
}
impl From<(i64, i64, i64)> for Point3 {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Point3 { v: [x, y, z] }
    }
}
impl Into<(i64, i64, i64)> for Point3 {
    fn into(self) -> (i64, i64, i64) {
        (self.v[0], self.v[1], self.v[2])
    }
}
impl std::ops::Index<usize> for Point3 {
    type Output = i64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

impl std::ops::IndexMut<usize> for Point3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx]
    }
}
