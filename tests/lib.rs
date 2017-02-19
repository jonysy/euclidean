#![feature(plugin)]
#![plugin(speculate)]

extern crate euclidean;

speculate! {

    describe "plane geometry" {

        describe "coordinates in plane geometry" {
            use euclidean::Point2D;

            before {
                let point = Point2D::from([1., 2.]);
            }

            it "has an `x` coordinate" {
                assert_eq!(point.x(), 1.);
            }

            it "has a `y` coordinate" {
                assert_eq!(point.y(), 2.);
            }
        }

        describe "dimensions in plane geometry" {
            use euclidean::Size2D;

            before {
                let dimensions = Size2D::from([1., 2.]);
            }

            it "has a width" {
                assert_eq!(dimensions.width(), 1.);
            }

            it "has a height" {
                assert_eq!(dimensions.height(), 2.);
            }
        }

        describe "a plane region" {
            use euclidean::{Point2D, Size2D, Region2D};

            before {
                let origin = Point2D::from([1., 2.]);
                let dimensions = Size2D::from([4., 5.]);
                let region = Region2D::new(origin, dimensions);
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
            use euclidean::{Point2D, Size2D, Region2D};

            it "iterates over a region" {
                let region = Region2D::new(Point2D::from([5, 7]), Size2D::from([9, 10]));
                let mut it = region.iter();

                for y in 7..10 {
                    for x in 5..9 {
                        assert_eq!(Point2D::from([x, y]), it.next().unwrap());
                    }
                }

                assert!(it.next().is_none());
            }

            it "iterates over a 10x10 region" {
                let region = Region2D::new(Point2D::from([0, 0]), Size2D::from([10, 10]));
                let mut it = region.iter();

                for y in 0..10 {
                    for x in 0..10 {
                        assert_eq!(Point2D::from([x, y]), it.next().unwrap());
                    }
                }

                assert!(it.next().is_none());
            }
        }
    }

    describe "solid geometry" {

        describe "coordinates solid geometry" {
            use euclidean::Point3D;

            before {
                let coordinates = Point3D::from([1., 2., 3.]);
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
            use euclidean::Size3D;

            before {
                let dimensions = Size3D::from([1., 2., 3.]);
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
            use euclidean::{Point3D, Size3D, Region3D};

            before {
                let origin = Point3D::from([1., 2., 3.]);
                let dimensions = Size3D::from([4., 5., 6.]);
                let region = Region3D::new(origin, dimensions);
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