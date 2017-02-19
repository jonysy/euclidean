use array::{Array, ArrayLength as Length, FixedSizeArray};
use std::fmt;
use std::ops::{Index, IndexMut};
use typenum::{consts, Cmp, Greater};

pub type Size1D<T = f64> = Size<T, consts::U1>;
pub type Size2D<T = f64> = Size<T, consts::U2>;
pub type Size3D<T = f64> = Size<T, consts::U3>;

pub struct Size<T, N>(Array<T, N>) where N: Length<T>;

impl<T, N> Size<T, N> where N: Length<T> {

    #[inline]
    pub fn width(&self) -> T where T: Copy, N: Cmp<consts::U0, Output = Greater> {
        self[0]
    }

    #[inline]
    pub fn height(&self) -> T where T: Copy, N: Cmp<consts::U1, Output = Greater> {
        self[1]
    }

    pub fn dimensions(&self) -> Array<T, N> where Array<T, N>: Copy {
        self.0
    }

    #[inline]
    pub fn depth(&self) -> T where T: Copy, N: Cmp<consts::U2, Output = Greater> {
        self[2]
    }
}

impl<T, N> Clone for Size<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Size(self.0.clone())
    }
}

impl<T, N> Copy for Size<T, N> where N: Length<T>, Array<T, N>: Copy { }

impl<T, N> fmt::Debug for Size<T, N> where N: Length<T>, Array<T, N>: fmt::Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{:?}", self)
    }
}

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

impl<T, N> IndexMut<usize> for Size<T, N> where N: Length<T> {

    fn index_mut(&mut self, index: usize) -> &mut T {

        &mut self.0.as_mut_slice()[index]
    }
}