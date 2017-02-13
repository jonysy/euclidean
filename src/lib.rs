#![feature(fixed_size_array, pub_restricted)]

extern crate array;
extern crate num;
extern crate typenum;

pub use coordinates::{Coordinates, Coordinates1D, Coordinates2D, Coordinates3D, Coordinates4D};
pub use size::{Size, Size1D, Size2D, Size3D, Size4D};
pub use region::{Iter, Region, Region1D, Region2D, Region3D, Region4D};

mod coordinates;
mod size;
mod region;