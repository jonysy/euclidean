//! Euclidean Geometry

#![deny(/*missing_docs,*/ unsafe_code, unused_import_braces, unused_qualifications)]
#![feature(field_init_shorthand, fixed_size_array, pub_restricted)]

extern crate array;
extern crate num;
extern crate typenum;

pub use point::{Point, Point1D, Point2D, Point3D};
pub use size::{Size, Size1D, Size2D, Size3D};
pub use region::{Iter, Region, Region1D, Region2D, Region3D};

mod point;
mod size;
mod region;