#![feature(plugin, test)]
#![plugin(speculate)]

extern crate euclidean;
extern crate test;

speculate! {
    use euclidean::{Coordinates2d, Size2d, Region2d};

    before {
        let region: Region2d<f64> = {
            let c = Coordinates2d::from([0., 0.]);
            let d = Size2d::from([10., 10.]);
            Region2d::new(c, d)
        };
    }

    bench "iterator 10x10 (f64)" |b| {

        b.iter(|| {
            // Consumes the iterator, counting the number of iterations and returning it.
            let _ = region.iter().count();
        });
    }
}