/*
Copyright 2017 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! BoundingBox3D, an axis aligned bounding box within 3D space

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// BoundingBox3D, an axis aligned bounding box within 3D space
pub struct BoundingBox3D {
    min: Point3D,
    max: Point3D,
}

impl BoundingBox3D {
    /// Creates a new BoundingBox3D with the given min and max positions
    pub fn new<P1, P2>(min: &P1, max: &P2) -> Result<BoundingBox3D>
    where
        P1: Is3D,
        P2: Is3D,
    {
        if min.x() == max.x() || min.y() == max.y() || min.z() == max.z() {
            Err(ErrorKind::MinMaxEqual)
        } else if min.x() > max.x() || min.y() > max.y() || min.z() > max.z() {
            Err(ErrorKind::MinMaxSwapped)
        } else {
            Ok(BoundingBox3D {
                min: Point3D {
                    x: min.x(),
                    y: min.y(),
                    z: min.z(),
                },
                max: Point3D {
                    x: max.x(),
                    y: max.y(),
                    z: max.z(),
                },
            })
        }
    }
    /// Creates a new BoundingBox3D which contains all the given positions
    pub fn from_iterator<'a, It3D, P>(source: It3D) -> Result<BoundingBox3D>
    where
        It3D: IntoIterator<Item = &'a P>,
        P: 'a + Is3D + Sized,
    {
        let mut count = 0;

        let mut minx: f64 = 0.0;
        let mut miny: f64 = 0.0;
        let mut minz: f64 = 0.0;
        let mut maxx: f64 = 0.0;
        let mut maxy: f64 = 0.0;
        let mut maxz: f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                minz = p.z();
                maxx = p.x();
                maxy = p.y();
                maxz = p.z();
                count += 1;
                continue;
            }
            if p.x() < minx {
                minx = p.x();
            }
            if p.y() < miny {
                miny = p.y();
            }
            if p.z() < minz {
                minz = p.z();
            }
            if p.x() > maxx {
                maxx = p.x();
            }
            if p.y() > maxy {
                maxy = p.y();
            }
            if p.z() > maxz {
                maxz = p.z();
            }
            count += 1;
        }
        if count >= 2 {
            Self::new(
                &Point3D {
                    x: minx,
                    y: miny,
                    z: minz,
                },
                &Point3D {
                    x: maxx,
                    y: maxy,
                    z: maxz,
                },
            )
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Creates a new BoundingBox3D which contains all the given positions
    pub fn from_into_iterator<It3D, P>(source: It3D) -> Result<BoundingBox3D>
    where
        It3D: IntoIterator<Item = P>,
        P: Is3D + Sized,
    {
        let mut count = 0;

        let mut minx: f64 = 0.0;
        let mut miny: f64 = 0.0;
        let mut minz: f64 = 0.0;
        let mut maxx: f64 = 0.0;
        let mut maxy: f64 = 0.0;
        let mut maxz: f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                minz = p.z();
                maxx = p.x();
                maxy = p.y();
                maxz = p.z();
                count += 1;
                continue;
            }
            if p.x() < minx {
                minx = p.x();
            }
            if p.y() < miny {
                miny = p.y();
            }
            if p.z() < minz {
                minz = p.z();
            }
            if p.x() > maxx {
                maxx = p.x();
            }
            if p.y() > maxy {
                maxy = p.y();
            }
            if p.z() > maxz {
                maxz = p.z();
            }
            count += 1;
        }
        if count >= 2 {
            Self::new(
                &Point3D {
                    x: minx,
                    y: miny,
                    z: minz,
                },
                &Point3D {
                    x: maxx,
                    y: maxy,
                    z: maxz,
                },
            )
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Returns the minimum position of the bounding box
    pub fn min_p(&self) -> Point3D {
        self.min.clone()
    }
    /// Returns the maximum position of the bounding box
    pub fn max_p(&self) -> Point3D {
        self.max.clone()
    }
    /// Returns the size the bounding box within the x-dimension
    pub fn size_x(&self) -> Positive {
        Positive::new((self.max.x() - self.min.x()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the size the bounding box within the y-dimension
    pub fn size_y(&self) -> Positive {
        Positive::new((self.max.y() - self.min.y()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the size the bounding box within the z-dimension
    pub fn size_z(&self) -> Positive {
        Positive::new((self.max.z() - self.min.z()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the sizes of the bounding box
    pub fn sizes(&self) -> [Positive; 3] {
        [self.size_x(), self.size_y(), self.size_z()]
    }
    /// Returns the center of the bounding box
    pub fn center_bb(&self) -> Point3D {
        Point3D {
            x: self.min.x() + (self.max.x() - self.min.x()) / 2.0,
            y: self.min.y() + (self.max.y() - self.min.y()) / 2.0,
            z: self.min.z() + (self.max.z() - self.min.z()) / 2.0,
        }
    }
    /// Tests whether this bounding box is within the other
    pub fn is_inside(&self, other: &BoundingBox3D) -> bool {
        self.min.x() > other.min.x()
            && self.min.y() > other.min.y()
            && self.min.z() > other.min.z()
            && self.max.x() < other.max.x()
            && self.max.y() < other.max.y()
            && self.max.z() < other.max.z()
    }
    /// Tests whether this bounding box contains a position
    pub fn contains<P>(&self, other: &P) -> bool
    where
        Self: Sized,
        P: Is3D,
    {
        other.x() > self.min.x()
            && other.x() < self.max.x()
            && other.y() > self.min.y()
            && other.y() < self.max.y()
            && other.z() > self.min.z()
            && other.z() < self.max.z()
    }
    /// Tests whether this bounding box contains the other
    pub fn has_inside(&self, other: &BoundingBox3D) -> bool {
        self.min.x() < other.min.x()
            && self.min.y() < other.min.y()
            && self.min.z() < other.min.z()
            && self.max.x() > other.max.x()
            && self.max.y() > other.max.y()
            && self.max.z() > other.max.z()
    }
    /// Tests whether this bounding box and the other overlap in any way
    pub fn collides_with(&self, other: &BoundingBox3D) -> bool {
        2.0 * (self.center_bb().x - other.center_bb().x).abs() < (*(self.size_x() + other.size_x()))
            && 2.0 * (self.center_bb().y - other.center_bb().y).abs()
                < (*(self.size_y() + other.size_y()))
            && 2.0 * (self.center_bb().z - other.center_bb().z).abs()
                < (*(self.size_z() + other.size_z()))
    }
    /// Tests whether this bounding box crosses a certain x value
    pub fn crossing_x_value(&self, x: f64) -> bool {
        self.min.x() < x && self.max.x() > x
    }
    /// Tests whether this bounding box crosses a certain y value
    pub fn crossing_y_value(&self, y: f64) -> bool {
        self.min.y() < y && self.max.y() > y
    }
    /// Tests whether this bounding box crosses a certain z value
    pub fn crossing_z_value(&self, z: f64) -> bool {
        self.min.z() < z && self.max.z() > z
    }
    /// Returns the corner points of the bounding box
    pub fn corners(&self) -> [Point3D; 8] {
        [
            Point3D::new(self.min.x(), self.min.y(), self.min.z()),
            Point3D::new(self.min.x(), self.min.y(), self.max.z()),
            Point3D::new(self.min.x(), self.max.y(), self.min.z()),
            Point3D::new(self.min.x(), self.max.y(), self.max.z()),
            Point3D::new(self.max.x(), self.min.y(), self.min.z()),
            Point3D::new(self.max.x(), self.min.y(), self.max.z()),
            Point3D::new(self.max.x(), self.max.y(), self.min.z()),
            Point3D::new(self.max.x(), self.max.y(), self.max.z()),
        ]
    }
    /// Returns the distance to another Is3D
    pub fn distance<P>(&self, other: &P) -> NonNegative
    where
        P: Is3D,
    {
        let sqr_dist = *self.sqr_distance(other);
        NonNegative::new(sqr_dist.sqrt()).unwrap()
    }
    /// Returns the square distance to another Is3D
    pub fn sqr_distance<P>(&self, other: &P) -> NonNegative
    where
        P: Is3D,
    {
        let dx = max_f64_3(
            self.min_p().x() - other.x(),
            0.0,
            other.x() - self.max_p().x(),
        );
        let dy = max_f64_3(
            self.min_p().y() - other.y(),
            0.0,
            other.y() - self.max_p().y(),
        );
        let dz = max_f64_3(
            self.min_p().z() - other.z(),
            0.0,
            other.z() - self.max_p().z(),
        );
        NonNegative::new(dx * dx + dy * dy + dz * dz).unwrap()
    }
}

//------------------------------------------------------------------------------

impl Default for BoundingBox3D {
    fn default() -> Self {
        BoundingBox3D {
            min: Point3D {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            max: Point3D {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        }
    }
}

impl IsND for BoundingBox3D {
    fn n_dimensions() -> usize {
        3
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        self.center_bb().position_nd(dimension)
    }
}

impl Is3D for BoundingBox3D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.center_bb().x()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.center_bb().y()
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        self.center_bb().z()
    }
}

impl IsMovable3D for BoundingBox3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.min.move_by(x, y, z);
        self.max.move_by(x, y, z);
    }
}

impl HasBoundingBox3D for BoundingBox3D {
    fn bounding_box(&self) -> BoundingBox3D {
        BoundingBox3D::new(&self.min, &self.max).unwrap() // safe
    }
}

impl HasBoundingBox3DMaybe for BoundingBox3D {
    fn bounding_box_maybe(&self) -> Option<BoundingBox3D> {
        Some(self.bounding_box())
    }
}

impl HasDistanceTo<BoundingBox3D> for BoundingBox3D {
    fn sqr_distance(&self, other: &BoundingBox3D) -> NonNegative {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;

        if other.max_p().x() < self.min_p().x() {
            dx = other.max_p().x() - self.min_p().x();
        } else if other.min_p().x() > self.max_p().x() {
            dx = other.min_p().x() - self.max_p().x();
        }

        if other.max_p().y() < self.min_p().y() {
            dy = other.max_p().y() - self.min_p().y();
        } else if other.min_p().y() > self.max_p().y() {
            dy = other.min_p().y() - self.max_p().y();
        }

        if other.max_p().z() < self.min_p().z() {
            dz = other.max_p().z() - self.min_p().z();
        } else if other.min_p().z() > self.max_p().z() {
            dz = other.min_p().z() - self.max_p().z();
        }

        NonNegative::new(dx * dx + dy * dy + dz * dz).unwrap()
    }
}

impl IsScalable for BoundingBox3D {
    fn scale(&mut self, factor: Positive) {
        let c = self.center_bb();
        let min_x = c.x - (0.5 * *factor * *self.size_x());
        let max_x = c.x + (0.5 * *factor * *self.size_x());
        let min_y = c.y - (0.5 * *factor * *self.size_y());
        let max_y = c.y + (0.5 * *factor * *self.size_y());
        let min_z = c.z - (0.5 * *factor * *self.size_z());
        let max_z = c.z + (0.5 * *factor * *self.size_z());

        self.min.set_xyz(min_x, min_y, min_z);
        self.max.set_xyz(max_x, max_y, max_z);
    }
}

impl IsMergeable for BoundingBox3D {
    fn consume(&mut self, other: Self) {
        let (mut min_x, mut min_y, mut min_z) = (self.min.x(), self.min.y(), self.min.z());
        let (mut max_x, mut max_y, mut max_z) = (self.max.x(), self.max.y(), self.max.z());

        if other.min.x() < min_x {
            min_x = other.min.x()
        }
        if other.min.y() < min_y {
            min_y = other.min.y()
        }
        if other.min.z() < min_z {
            min_z = other.min.z()
        }

        if other.max.x() > max_x {
            max_x = other.max.x()
        }
        if other.max.y() > max_y {
            max_y = other.max.y()
        }
        if other.max.z() > max_z {
            max_z = other.max.z()
        }

        self.min.set_xyz(min_x, min_y, min_z);
        self.max.set_xyz(max_x, max_y, max_z);
    }

    fn combine(&self, other: &Self) -> Self {
        let (mut min_x, mut min_y, mut min_z) = (self.min.x(), self.min.y(), self.min.z());
        let (mut max_x, mut max_y, mut max_z) = (self.max.x(), self.max.y(), self.max.z());

        if other.min.x() < min_x {
            min_x = other.min.x()
        }
        if other.min.y() < min_y {
            min_y = other.min.y()
        }
        if other.min.z() < min_z {
            min_z = other.min.z()
        }

        if other.max.x() > max_x {
            max_x = other.max.x()
        }
        if other.max.y() > max_y {
            max_y = other.max.y()
        }
        if other.max.z() > max_z {
            max_z = other.max.z()
        }

        let min = Point3D::new(min_x, min_y, min_z);
        let max = Point3D::new(max_x, max_y, max_z);

        BoundingBox3D { min, max }
    }
}

impl IsSATObject for BoundingBox3D {
    fn for_each_point<F>(&self, f: &mut F)
    where
        F: FnMut(&Point3D),
    {
        let (minx, miny, minz) = (self.min_p().x(), self.min_p().y(), self.min_p().z());
        let (maxx, maxy, maxz) = (self.max_p().x(), self.max_p().y(), self.max_p().z());

        f(&Point3D::new(minx, miny, minz));
        f(&Point3D::new(minx, miny, maxz));
        f(&Point3D::new(minx, maxy, minz));
        f(&Point3D::new(minx, maxy, maxz));
        f(&Point3D::new(maxx, miny, minz));
        f(&Point3D::new(maxx, miny, maxz));
        f(&Point3D::new(maxx, maxy, minz));
        f(&Point3D::new(maxx, maxy, maxz));
    }

    fn for_each_axis<F>(&self, f: &mut F)
    where
        F: FnMut(&Norm3D),
    {
        f(&Norm3D::norm_x());
        f(&Norm3D::norm_y());
        f(&Norm3D::norm_z());
    }
}
