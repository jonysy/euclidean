use array::{Array, ArrayLength as Length, FixedSizeArray};
use std::ops::Index;
use typenum::{consts, Cmp, Greater};

pub type Dimensions1d<T = f64> = Dimensions<T, consts::U1>;
pub type Dimensions2d<T = f64> = Dimensions<T, consts::U2>;
pub type Dimensions3d<T = f64> = Dimensions<T, consts::U3>;
pub type Dimensions4d<T = f64> = Dimensions<T, consts::U4>;

pub struct Dimensions<T, N>(Array<T, N>) where N: Length<T>;

impl<T, N> Dimensions<T, N> where N: Length<T> {

    #[inline]
    pub fn width(&self) -> T where T: Copy, N: Cmp<consts::U0, Output = Greater> {
        self.0.as_slice()[0]
    }

    #[inline]
    pub fn height(&self) -> T where T: Copy, N: Cmp<consts::U1, Output = Greater> {
        self.0.as_slice()[1]
    }

    #[inline]
    pub fn depth(&self) -> T where T: Copy, N: Cmp<consts::U2, Output = Greater> {
        self.0.as_slice()[2]
    }
}

impl<T, N> Clone for Dimensions<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Dimensions(self.0.clone())
    }
}

impl<T, N> Copy for Dimensions<T, N> where N: Length<T>, Array<T, N>: Copy { }

impl<T> From<[T; 1]> for Dimensions<T, consts::U1> {

    fn from(array: [T; 1]) -> Self {

        Dimensions(array)
    }
}

impl<T> From<[T; 2]> for Dimensions<T, consts::U2> {

    fn from(array: [T; 2]) -> Self {

        Dimensions(array)
    }
}

impl<T> From<[T; 3]> for Dimensions<T, consts::U3> {

    fn from(array: [T; 3]) -> Self {

        Dimensions(array)
    }
}

impl<T, N> Index<usize> for Dimensions<T, N> where N: Length<T> {

    type Output = T;

    fn index(&self, index: usize) -> &T {

        &self.0.as_slice()[index]
    }
}