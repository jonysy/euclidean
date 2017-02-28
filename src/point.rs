use array::{Array, ArrayLength as Length, FixedSizeArray};
use std::{clone, cmp, convert, fmt, marker, ops};
use typenum::{consts, Cmp, Greater};

/// Point in a 1-dimensional coordinate system (simply a single coordinate).
pub type Point1D<T = f64> = Point<T, consts::U1>;

/// Point in a 2-dimensional coordinate system.
pub type Point2D<T = f64> = Point<T, consts::U2>;

/// Point in a 3-dimensional coordinate system.
pub type Point3D<T = f64> = Point<T, consts::U3>;

/// Point in an `n`-dimensional coordinate system.
pub struct Point<T, N>(Array<T, N>) where N: Length<T>;

impl<T, N> Point<T, N> where N: Length<T> {

    pub fn coordinates(&self) -> Array<T, N> where Array<T, N>: Copy {
        self.0
    }
}

impl<T, N> Point<T, N> where N: Cmp<consts::U0, Output = Greater> + Length<T> {

    /// Returns the x-coordinate.
    #[inline]
    pub fn x(&self) -> T where T: Copy {
        self[0]
    }

    /// Returns a mutable reference to the x-coordinate.
    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }
}

impl<T, N> Point<T, N> where N: Cmp<consts::U1, Output = Greater> + Length<T> {

    /// Returns the y-coordinate.
    #[inline]
    pub fn y(&self) -> T where T: Copy {
        self[1]
    }

    /// Returns a mutable reference to the y-coordinate.
    pub fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }
}

impl<T, N> Point<T, N> where N: Cmp<consts::U2, Output = Greater> + Length<T> {

    /// Returns the z-coordinate
    #[inline]
    pub fn z(&self) -> T where T: Copy {
        self[2]
    }

    /// Returns a mutable reference to the z-coordinate.
    pub fn z_mut(&mut self) -> &mut T {

        &mut self[2]
    }
}

impl<T, N> clone::Clone for Point<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Point(self.0.clone())
    }
}

impl<T, N> cmp::Eq for Point<T, N> where N: Length<T>, Array<T, N>: Eq { }

impl<T, N> cmp::PartialEq for Point<T, N> where N: Length<T>, Array<T, N>: PartialEq {

    fn eq(&self, other: &Self) -> bool {

        self.0 == other.0
    }
}

impl<T> convert::From<[T; 1]> for Point<T, consts::U1> {

    fn from(array: [T; 1]) -> Self {

        Point(array)
    }
}

impl<T> convert::From<[T; 2]> for Point<T, consts::U2> {

    fn from(array: [T; 2]) -> Self {

        Point(array)
    }
}

impl<T> convert::From<[T; 3]> for Point<T, consts::U3> {

    fn from(array: [T; 3]) -> Self {

        Point(array)
    }
}

impl<T> convert::From<[T; 4]> for Point<T, consts::U4> {

    fn from(array: [T; 4]) -> Self {

        Point(array)
    }
}

impl<T> convert::Into<[T; 2]> for Point<T, consts::U2> {

    fn into(self) -> [T; 2] {
        self.0
    }
}

impl<T, N> fmt::Debug for Point<T, N> where N: Length<T>, Array<T, N>: fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{:?}", self)
    }
}

impl<T> fmt::Display for Point<T, consts::U2> where T: Copy + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl<T, N> marker::Copy for Point<T, N> where N: Length<T>, Array<T, N>: Copy { }

impl<T, N> ops::Index<usize> for Point<T, N> where N: Length<T> {

    type Output = T;

    fn index(&self, index: usize) -> &T {

        &self.0.as_slice()[index]
    }
}

impl<T, N> ops::IndexMut<usize> for Point<T, N> where N: Length<T> {

    fn index_mut(&mut self, index: usize) -> &mut T {

        &mut self.0.as_mut_slice()[index]
    }
}