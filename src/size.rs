use array::{Array, ArrayLength as Length, FixedSizeArray};
use std::ops::Index;
use typenum::{consts, Cmp, Greater};

pub type Size1d<T = f64> = Size<T, consts::U1>;
pub type Size2d<T = f64> = Size<T, consts::U2>;
pub type Size3d<T = f64> = Size<T, consts::U3>;
pub type Size4d<T = f64> = Size<T, consts::U4>;

pub struct Size<T, N>(Array<T, N>) where N: Length<T>;

impl<T, N> Size<T, N> where N: Length<T> {

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

impl<T, N> Clone for Size<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Size(self.0.clone())
    }
}

impl<T, N> Copy for Size<T, N> where N: Length<T>, Array<T, N>: Copy { }

impl<T> From<[T; 1]> for Size<T, consts::U1> {

    fn from(array: [T; 1]) -> Self {

        Size(array)
    }
}

impl<T> From<[T; 2]> for Size<T, consts::U2> {

    fn from(array: [T; 2]) -> Self {

        Size(array)
    }
}

impl<T> From<[T; 3]> for Size<T, consts::U3> {

    fn from(array: [T; 3]) -> Self {

        Size(array)
    }
}

impl<T, N> Index<usize> for Size<T, N> where N: Length<T> {

    type Output = T;

    fn index(&self, index: usize) -> &T {

        &self.0.as_slice()[index]
    }
}