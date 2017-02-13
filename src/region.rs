use Coordinates;
use Coordinates2d;
use Dimensions;
use array::{Array, ArrayLength as Length};
use num::{One, Zero};
use std::ops::{AddAssign, Mul};
use typenum::{consts, Cmp, Greater};

pub type Region1d<T = f64> = Region<T, consts::U1>;
pub type Region2d<T = f64> = Region<T, consts::U2>;
pub type Region3d<T = f64> = Region<T, consts::U3>;
pub type Region4d<T = f64> = Region<T, consts::U4>;

pub struct Region<T, N> where N: Length<T> {
    pub origin: Coordinates<T, N>,
    pub dimensions: Dimensions<T, N>,
}

impl<T, N> Region<T, N> where N: Length<T> {

    pub fn new(origin: Coordinates<T, N>, dimensions: Dimensions<T, N>) -> Self {

        Region { origin: origin, dimensions: dimensions }
    }
}

impl<T, N> Region<T, N> 
    where T: Copy, 
          N: Cmp<consts::U0, Output = Greater>,
          N: Length<T> {

    #[inline]
    pub fn x(&self) -> T {
        self.origin.x()
    }

    #[inline]
    pub fn width(&self) -> T {
        self.dimensions.width()
    }
}

impl<T, N> Region<T, N> 
    where T: Copy, 
          N: Cmp<consts::U0, Output = Greater>,
          N: Cmp<consts::U1, Output = Greater>,
          N: Length<T> {

    #[inline]
    pub fn y(&self) -> T {
        self.origin.y()
    }

    #[inline]
    pub fn height(&self) -> T {
        self.dimensions.height()
    }
}

impl<T> Region<T, consts::U2> {

    #[inline]
    pub fn area(&self) -> <T as Mul<T>>::Output where T: Copy + Mul {
        self.width() * self.height()
    }

    pub fn iter(&self) -> Iter<T> where T: Copy + PartialOrd, Array<T, consts::U2>: Clone {

        assert!(self.origin.x() < self.dimensions.width() && self.origin.y() < self.dimensions.height());

        Iter {
            position: self.origin.clone(),
            region: self,
        }
    }
}

impl<T, N> Region<T, N> 
    where T: Copy, 
          N: Cmp<consts::U0, Output = Greater>,
          N: Cmp<consts::U1, Output = Greater>,
          N: Cmp<consts::U2, Output = Greater>,
          N: Length<T> {

    #[inline]
    pub fn z(&self) -> T {
        self.origin.z()
    }

    #[inline]
    pub fn depth(&self) -> T {
        self.dimensions.depth()
    }
}

impl<T> Region<T, consts::U3> {

    #[inline]
    pub fn volume(&self) -> <<T as Mul<T>>::Output as Mul<T>>::Output 
        where T: Copy + Mul, 
              <T as Mul<T>>::Output: Mul<T>,
    {
        self.width() * self.height() * self.depth()
    }
}

impl<T, N> Clone for Region<T, N> where N: Length<T>, Array<T, N>: Clone {

    fn clone(&self) -> Self {
        Region { origin: self.origin.clone(), dimensions: self.dimensions.clone() }
    }
}

impl<T, N> Copy for Region<T, N> where N: Length<T>, Array<T, N>: Copy { }

pub struct Iter<'r, T: 'r> { position: Coordinates2d<T>, region: &'r Region2d<T> }

impl<'r, T> Iterator for Iter<'r, T> 
    where T: AddAssign + Copy + One + PartialOrd + Zero
{

    type Item = Coordinates2d<T>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.position.x() >= self.region.width() {

            *self.position.x_mut() = self.region.origin.x();
            *self.position.y_mut() += T::one();
        }

        if self.position.y() >= self.region.height() {

            return None;
        }

        let x_coordinate = self.position.x();
        let y_coordinate = self.position.y();

        *self.position.x_mut() += T::one();

        Some(Coordinates::from([x_coordinate, y_coordinate]))
    }
}