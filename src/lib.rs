#![feature(fixed_size_array, pub_restricted)]

extern crate array;
extern crate num;
extern crate typenum;

pub use coordinates::{Coordinates, Coordinates1d, Coordinates2d, Coordinates3d, Coordinates4d};
pub use dimensions::{Dimensions, Dimensions1d, Dimensions2d, Dimensions3d, Dimensions4d};
pub use region::{Iter, Region, Region1d, Region2d, Region3d, Region4d};

mod coordinates;
mod dimensions;
mod region;