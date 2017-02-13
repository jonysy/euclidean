#![feature(plugin)]
#![plugin(speculate)]

extern crate euclidean;

speculate! {

    describe "plane geometry" {

        describe "coordinates in plane geometry" {
            use euclidean::Coordinates2d;

            before {
                let coordinates = Coordinates2d::from([1., 2.]);
            }

            it "has an `x` coordinate" {
                assert_eq!(coordinates.x(), 1.);
            }

            it "has a `y` coordinate" {
                assert_eq!(coordinates.y(), 2.);
            }
        }

        describe "dimensions in plane geometry" {
            use euclidean::Dimensions2d;

            before {
                let dimensions = Dimensions2d::from([1., 2.]);
            }

            it "has a width" {
                assert_eq!(dimensions.width(), 1.);
            }

            it "has a height" {
                assert_eq!(dimensions.height(), 2.);
            }
        }

        describe "a plane region" {
            use euclidean::{Coordinates2d, Dimensions2d, Region2d};

            before {
                let origin = Coordinates2d::from([1., 2.]);
                let dimensions = Dimensions2d::from([4., 5.]);
                let region = Region2d::new(origin, dimensions);
            }

            it "has an origin" {
                assert!(region.x() == 1. && region.y() == 2.);
            }

            it "has a width and a height" {
                assert!(region.width() == 4. && region.height() == 5.);
            }

            it "has an area" {
                assert_eq!(region.area(), 20.);
            }
        }

        describe "region iterator" {
            use euclidean::{Coordinates2d, Dimensions2d, Region2d};

            it "iterates over a region" {
                let region = Region2d::new(Coordinates2d::from([5, 7]), Dimensions2d::from([9, 10]));
                let mut it = region.iter();

                for y in 7..10 {
                    for x in 5..9 {
                        assert_eq!(Coordinates2d::from([x, y]), it.next().unwrap());
                    }
                }

                assert!(it.next().is_none());
            }

            it "iterates over a 10x10 region" {
                let region = Region2d::new(Coordinates2d::from([0, 0]), Dimensions2d::from([10, 10]));
                let mut it = region.iter();

                for y in 0..10 {
                    for x in 0..10 {
                        assert_eq!(Coordinates2d::from([x, y]), it.next().unwrap());
                    }
                }

                assert!(it.next().is_none());
            }
        }
    }

    describe "solid geometry" {

        describe "coordinates solid geometry" {
            use euclidean::Coordinates3d;

            before {
                let coordinates = Coordinates3d::from([1., 2., 3.]);
            }

            it "has an `x` coordinate" {
                assert_eq!(coordinates.x(), 1.);
            }

            it "has a `y` coordinate" {
                assert_eq!(coordinates.y(), 2.);
            }

            it "has a `z` coordinate" {
                assert_eq!(coordinates.z(), 3.);
            }
        }

        describe "dimensions in solid geometry" {
            use euclidean::Dimensions3d;

            before {
                let dimensions = Dimensions3d::from([1., 2., 3.]);
            }

            it "has a width" {
                assert_eq!(dimensions.width(), 1.);
            }

            it "has a height" {
                assert_eq!(dimensions.height(), 2.);
            }

            it "has depth" {
                assert_eq!(dimensions.depth(), 3.);
            }
        }

        describe "a solid region" {
            use euclidean::{Coordinates3d, Dimensions3d, Region3d};

            before {
                let origin = Coordinates3d::from([1., 2., 3.]);
                let dimensions = Dimensions3d::from([4., 5., 6.]);
                let region = Region3d::new(origin, dimensions);
            }

            it "has an origin" {
                assert!(region.x() == 1. && region.y() == 2. && region.z() == 3.);
            }

            it "has a width, height, and depth" {
                assert!(region.width() == 4. && region.height() == 5. && region.depth() == 6.);
            }

            it "has a volume" {
                assert_eq!(region.volume(), 120.);
            }
        }
    }
}