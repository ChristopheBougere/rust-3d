/*
Copyright 2018 Martin Buck

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

//! Polygon3D, a polygon within 3D space

use std::fmt;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Polygon3D, a polygon within 3D space
pub struct Polygon3D<P>
where
    P: IsBuildable3D,
{
    pc: PointCloud3D<P>,
}

impl<P> IsPolygon<P> for Polygon3D<P>
where
    P: IsBuildable3D + Clone,
{
    fn num_segments(&self) -> usize {
        self.pc.len()
    }

    fn segment_vertex_ids(&self, segmentid: SId) -> Option<(VId, VId)> {
        if segmentid.0 >= self.pc.len() {
            None
        } else if segmentid.0 == self.pc.len() - 1 {
            Some((VId(segmentid.0), VId(0)))
        } else {
            Some((VId(segmentid.0), VId(segmentid.0 + 1)))
        }
    }

    fn segment_vertices(&self, segmentid: SId) -> Option<(P, P)> {
        let (vid1, vid2) = self.segment_vertex_ids(segmentid)?;
        Some((self.pc[vid1.0].clone(), self.pc[vid2.0].clone()))
    }

    fn vertex(&self, vertexid: VId) -> Option<P> {
        self.pc.get_d(vertexid.0).clone()
    }
}

impl<P> IsEditablePolygon<P> for Polygon3D<P>
where
    P: IsBuildable3D + Clone,
{
    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.data.push(vertex);
        VId(self.pc.len() - 1)
    }

    fn change_vertex(&mut self, vertexid: VId, vertex: P) -> Result<()> {
        if vertexid.0 >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }

        self.pc[vertexid.0] = vertex;
        Ok(())
    }
}

impl<P> IsMovable3D for Polygon3D<P>
where
    P: IsBuildable3D + IsMovable3D,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.pc.move_by(x, y, z)
    }
}

impl<P> HasBoundingBox3DMaybe for Polygon3D<P>
where
    P: IsBuildable3D,
{
    fn bounding_box_maybe(&self) -> Option<BoundingBox3D> {
        self.pc.bounding_box_maybe()
    }
}

impl<P> HasCenterOfGravity3D for Polygon3D<P>
where
    P: IsBuildable3D,
{
    fn center_of_gravity(&self) -> Option<Point3D> {
        self.pc.center_of_gravity()
    }
}

impl<P> HasLength for Polygon3D<P>
where
    P: IsBuildable3D,
{
    fn length(&self) -> f64 {
        let mut length = self.pc.length();

        if self.pc.data.len() > 0 {
            length += dist_3d(&self.pc.data[self.pc.data.len() - 1], &self.pc.data[0]);
        }

        length
    }
}

impl<P> IsScalable for Polygon3D<P>
where
    P: IsEditable3D + IsBuildable3D,
{
    fn scale(&mut self, factor: Positive) {
        self.pc.scale(factor)
    }
}

impl<P> IsMatrix4Transformable for Polygon3D<P>
where
    P: IsBuildable3D + IsMatrix4Transformable + Clone,
{
    fn transformed(&self, m: &Matrix4) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix4) {
        self.pc.transform(m);
    }
}

impl<P> Default for Polygon3D<P>
where
    //https://github.com/rust-lang/rust/issues/26925
    P: IsBuildable3D,
{
    fn default() -> Self {
        let pc = PointCloud3D::default();
        Polygon3D { pc }
    }
}

impl<P> fmt::Display for Polygon3D<P>
where
    P: IsBuildable3D + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pc.fmt(f)
    }
}

impl<P> From<PointCloud3D<P>> for Polygon3D<P>
where
    P: IsBuildable3D,
{
    fn from(pc: PointCloud3D<P>) -> Self {
        Polygon3D { pc }
    }
}
