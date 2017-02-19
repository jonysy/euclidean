#![feature(plugin, test)]
#![plugin(speculate)]

extern crate euclidean;
extern crate test;

speculate! {
    use euclidean::{Point2D, Size2D, Region2D};

    before {
        let region = {

            let c = Point2D::from([0., 0.]);
            let d = Size2D::from([10., 10.]);

            Region2D::new(c, d)
        };
    }

    bench "iterator 10x10 (f64)" |b| {

        b.iter(|| {
            // Consumes the iterator, counting the number of iterations and returning it.
            let _ = region.iter().count();
        });
    }
}