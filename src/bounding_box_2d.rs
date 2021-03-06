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

//! BoundingBox2D, an axis aligned bounding box within 2D space

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// BoundingBox2D, an axis aligned bounding box within 2D space
pub struct BoundingBox2D {
    min: Point2D,
    max: Point2D,
}

impl BoundingBox2D {
    /// Creates a new BoundingBox2D with the given min and max positions
    pub fn new<P1, P2>(min: &P1, max: &P2) -> Result<BoundingBox2D>
    where
        P1: Is2D,
        P2: Is2D,
    {
        if min.x() == max.x() || min.y() == max.y() {
            Err(ErrorKind::MinMaxEqual)
        } else if min.x() > max.x() || min.y() > max.y() {
            Err(ErrorKind::MinMaxSwapped)
        } else {
            Ok(BoundingBox2D {
                min: Point2D {
                    x: min.x(),
                    y: min.y(),
                },
                max: Point2D {
                    x: max.x(),
                    y: max.y(),
                },
            })
        }
    }
    /// Creates a new BoundingBox2D which contains all the given positions
    pub fn from_iterator<'a, It2D, P>(source: It2D) -> Result<BoundingBox2D>
    where
        It2D: IntoIterator<Item = &'a P>,
        P: 'a + Is2D + Sized,
    {
        let mut count = 0;

        let mut minx: f64 = 0.0;
        let mut miny: f64 = 0.0;
        let mut maxx: f64 = 0.0;
        let mut maxy: f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                maxx = p.x();
                maxy = p.y();
                count += 1;
                continue;
            }
            if p.x() < minx {
                minx = p.x();
            }
            if p.y() < miny {
                miny = p.y();
            }
            if p.x() > maxx {
                maxx = p.x();
            }
            if p.y() > maxy {
                maxy = p.y();
            }
            count += 1;
        }
        if count >= 2 {
            Self::new(&Point2D { x: minx, y: miny }, &Point2D { x: maxx, y: maxy })
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Creates a new BoundingBox2D which contains all the given positions
    pub fn from_into_iterator<It2D, P>(source: It2D) -> Result<BoundingBox2D>
    where
        It2D: IntoIterator<Item = P>,
        P: Is2D + Sized,
    {
        let mut count = 0;

        let mut minx: f64 = 0.0;
        let mut miny: f64 = 0.0;
        let mut maxx: f64 = 0.0;
        let mut maxy: f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                maxx = p.x();
                maxy = p.y();
                count += 1;
                continue;
            }
            if p.x() < minx {
                minx = p.x();
            }
            if p.y() < miny {
                miny = p.y();
            }
            if p.x() > maxx {
                maxx = p.x();
            }
            if p.y() > maxy {
                maxy = p.y();
            }
            count += 1;
        }
        if count >= 2 {
            Self::new(&Point2D { x: minx, y: miny }, &Point2D { x: maxx, y: maxy })
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Returns the minimum position of the bounding box
    pub fn min_p(&self) -> Point2D {
        self.min.clone()
    }
    /// Returns the maximum position of the bounding box
    pub fn max_p(&self) -> Point2D {
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
    /// Returns the sizes of the bounding box
    pub fn sizes(&self) -> [Positive; 2] {
        [self.size_x(), self.size_y()]
    }
    /// Returns the center of the bounding box
    pub fn center_bb(&self) -> Point2D {
        Point2D {
            x: self.min.x() + (self.max.x() - self.min.x()) / 2.0,
            y: self.min.y() + (self.max.y() - self.min.y()) / 2.0,
        }
    }
    /// Tests whether this bounding box is within the other
    pub fn is_inside(&self, other: &BoundingBox2D) -> bool {
        self.min.x() > other.min.x()
            && self.min.y() > other.min.y()
            && self.max.x() < other.max.x()
            && self.max.y() < other.max.y()
    }
    /// Tests whether this bounding box contains a position
    pub fn contains<P>(&self, other: &P) -> bool
    where
        Self: Sized,
        P: Is2D,
    {
        other.x() > self.min.x()
            && other.x() < self.max.x()
            && other.y() > self.min.y()
            && other.y() < self.max.y()
    }
    /// Tests whether this bounding box contains the other
    pub fn has_inside(&self, other: &BoundingBox2D) -> bool {
        self.min.x() < other.min.x()
            && self.min.y() < other.min.y()
            && self.max.x() > other.max.x()
            && self.max.y() > other.max.y()
    }
    /// Tests whether this bounding box and the other overlap in any way
    pub fn collides_with(&self, other: &BoundingBox2D) -> bool {
        2.0 * (self.center_bb().x - other.center_bb().x).abs() < *(self.size_x() + other.size_x())
            && 2.0 * (self.center_bb().y - other.center_bb().y).abs()
                < *(self.size_y() + other.size_y())
    }
    /// Tests whether this bounding box crosses a certain x value
    pub fn crossing_x_value(&self, x: f64) -> bool {
        self.min.x() < x && self.max.x() > x
    }
    /// Tests whether this bounding box crosses a certain y value
    pub fn crossing_y_value(&self, y: f64) -> bool {
        self.min.y() < y && self.max.y() > y
    }
    /// Returns the corner points of the bounding box
    pub fn corners(&self) -> [Point2D; 4] {
        [
            Point2D::new(self.min.x(), self.min.y()),
            Point2D::new(self.min.x(), self.max.y()),
            Point2D::new(self.max.x(), self.min.y()),
            Point2D::new(self.max.x(), self.max.y()),
        ]
    }
    /// Returns the distance to another Is2D
    pub fn distance<P>(&self, other: &P) -> NonNegative
    where
        P: Is2D,
    {
        let sqr_dist = *self.sqr_distance(other);
        NonNegative::new(sqr_dist.sqrt()).unwrap()
    }
    /// Returns the square distance to another Is2D
    pub fn sqr_distance<P>(&self, other: &P) -> NonNegative
    where
        P: Is2D,
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
        NonNegative::new(dx * dx + dy * dy).unwrap()
    }
}

//------------------------------------------------------------------------------

impl Default for BoundingBox2D {
    fn default() -> Self {
        BoundingBox2D {
            min: Point2D { x: -0.5, y: -0.5 },
            max: Point2D { x: 0.5, y: 0.5 },
        }
    }
}

impl IsND for BoundingBox2D {
    fn n_dimensions() -> usize {
        2
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        self.center_bb().position_nd(dimension)
    }
}

impl Is2D for BoundingBox2D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.center_bb().x()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.center_bb().y()
    }
}

impl IsMovable2D for BoundingBox2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.min.move_by(x, y);
        self.max.move_by(x, y);
    }
}

impl HasBoundingBox2D for BoundingBox2D {
    fn bounding_box(&self) -> BoundingBox2D {
        BoundingBox2D::new(&self.min, &self.max).unwrap() // safe
    }
}

impl HasBoundingBox2DMaybe for BoundingBox2D {
    fn bounding_box_maybe(&self) -> Option<BoundingBox2D> {
        Some(self.bounding_box())
    }
}

impl HasDistanceTo<BoundingBox2D> for BoundingBox2D {
    fn sqr_distance(&self, other: &BoundingBox2D) -> NonNegative {
        let mut dx = 0.0;
        let mut dy = 0.0;

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

        NonNegative::new(dx * dx + dy * dy).unwrap()
    }
}

impl IsScalable for BoundingBox2D {
    fn scale(&mut self, factor: Positive) {
        let c = self.center_bb();
        let min_x = c.x - (0.5 * *factor * *self.size_x());
        let max_x = c.x + (0.5 * *factor * *self.size_x());
        let min_y = c.y - (0.5 * *factor * *self.size_y());
        let max_y = c.y + (0.5 * *factor * *self.size_y());

        self.min.set_xy(min_x, min_y);
        self.max.set_xy(max_x, max_y);
    }
}

impl IsMergeable for BoundingBox2D {
    fn consume(&mut self, other: Self) {
        let (mut min_x, mut min_y) = (self.min.x(), self.min.y());
        let (mut max_x, mut max_y) = (self.max.x(), self.max.y());

        if other.min.x() < min_x {
            min_x = other.min.x()
        }
        if other.min.y() < min_y {
            min_y = other.min.y()
        }

        if other.max.x() > max_x {
            max_x = other.max.x()
        }
        if other.max.y() > max_y {
            max_y = other.max.y()
        }

        self.min.set_xy(min_x, min_y);
        self.max.set_xy(max_x, max_y);
    }

    fn combine(&self, other: &Self) -> Self {
        let (mut min_x, mut min_y) = (self.min.x(), self.min.y());
        let (mut max_x, mut max_y) = (self.max.x(), self.max.y());

        if other.min.x() < min_x {
            min_x = other.min.x()
        }
        if other.min.y() < min_y {
            min_y = other.min.y()
        }

        if other.max.x() > max_x {
            max_x = other.max.x()
        }
        if other.max.y() > max_y {
            max_y = other.max.y()
        }

        let min = Point2D::new(min_x, min_y);
        let max = Point2D::new(max_x, max_y);

        BoundingBox2D { min, max }
    }
}
