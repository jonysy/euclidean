use array::{Array, ArrayLength as Length, FixedSizeArray};
use std::fmt;
use typenum::{consts, Cmp, Greater};

pub type Coordinates1d<T = f64> = Coordinates<T, consts::U1>;
pub type Coordinates2d<T = f64> = Coordinates<T, consts::U2>;
pub type Coordinates3d<T = f64> = Coordinates<T, consts::U3>;
pub type Coordinates4d<T = f64> = Coordinates<T, consts::U4>;

pub struct Coordinates<T, N>(Array<T, N>) where N: Length<T>;

impl<T, N> Coordinates<T, N> where N: Cmp<consts::U0, Output = Greater> + Length<T> {

    /// The x-coordinate.
    #[inline]
    pub fn x(&self) -> T where T: Copy {
        self.0.as_slice()[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0.as_mut_slice()[0]
    }
}

impl<T, N> Coordinates<T, N> where N: Cmp<consts::U1, Output = Greater> + Length<T> {

    /// The y-coordinate.
    #[inline]
    pub fn y(&self) -> T where T: Copy {
        self.0.as_slice()[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0.as_mut_slice()[1]
    }
}

impl<T, N> Coordinates<T, N> where N: Cmp<consts::U2, Output = Greater> + Length<T> {

    #[inline]
    pub fn z(&self) -> T where T: Copy {
        self.0.as_slice()[2]
    }
}

impl<T, N> Coordinates<T, N> where N: Cmp<consts::U3, Output = Greater> + Length<T> {

    #[inline]
    pub fn w(&self) -> T where T: Copy {
        self.0.as_slice()[3]
    }
}

impl<T, N> Clone for Coordinates<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Coordinates(self.0.clone())
    }
}

impl<T, N> Copy for Coordinates<T, N> where N: Length<T>, Array<T, N>: Copy { }

impl<T, N> fmt::Debug for Coordinates<T, N> where N: Length<T>, Array<T, N>: fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{:?}", self)
    }
}

impl<T, N> Eq for Coordinates<T, N> where N: Length<T>, Array<T, N>: Eq { }

impl<T> From<[T; 1]> for Coordinates<T, consts::U1> {

    fn from(array: [T; 1]) -> Self {

        Coordinates(array)
    }
}

impl<T> From<[T; 2]> for Coordinates<T, consts::U2> {

    fn from(array: [T; 2]) -> Self {

        Coordinates(array)
    }
}

impl<T> From<[T; 3]> for Coordinates<T, consts::U3> {

    fn from(array: [T; 3]) -> Self {

        Coordinates(array)
    }
}

impl<T> From<[T; 4]> for Coordinates<T, consts::U4> {

    fn from(array: [T; 4]) -> Self {

        Coordinates(array)
    }
}

impl<T, N> PartialEq for Coordinates<T, N> where N: Length<T>, Array<T, N>: PartialEq {

    fn eq(&self, other: &Self) -> bool {

        self.0 == other.0
    }
}