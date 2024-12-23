/// Represents a point on a 2D grid.
///
/// ### Note
/// The coordinates of `Point` use matrix conventions *(row, col)*
/// rather than cartesian *(x,y)*, so increasing the second coordinate is
/// generally considered as moving downwards.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Point {
    pub row: i64,
    pub col: i64,
}

impl Point {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

impl From<(i64, i64)> for Point {
    fn from(coords: (i64, i64)) -> Self {
        Point {
            row: coords.0,
            col: coords.1,
        }
    }
}

// makes it quicker/cleaner to define arithmetic operation impls
macro_rules! impl_op {
    ($trait_name:ident, $fn_name:ident, $op:tt, $lhs:ty, $rhs:ty) => {
        impl std::ops::$trait_name<$rhs> for $lhs {
            type Output = Point;

            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                Point {
                    row: self.row $op rhs.row,
                    col: self.col $op rhs.col,
                }
            }
        }
    };
}

impl_op!(Add, add, +, Point, Point);
impl_op!(Add, add, +, &Point, &Point);
impl_op!(Add, add, +, Point, &Point);
impl_op!(Add, add, +, &Point, Point);
impl_op!(Sub, sub, -, Point, Point);
impl_op!(Sub, sub, -, &Point, &Point);
impl_op!(Sub, sub, -, &Point, Point);
impl_op!(Sub, sub, -, Point, &Point);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_points() {
        let p = Point::from((1, 3));
        let q = Point::from((2, -7));

        assert_eq!(p + q, Point::from((3, -4)));
    }

    #[test]
    fn should_subtract_points() {
        let p = Point::from((1, 3));
        let q = Point::from((2, -7));

        assert_eq!(p - q, Point::from((-1, 10)));
    }
}
