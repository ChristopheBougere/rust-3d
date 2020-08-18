/*
Copyright 2020 Martin Buck

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

//! Module for IO of the off file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load points from a .off file
pub struct OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    i_line: usize,
    line_buffer: Vec<u8>,
    off_seen: bool,
    n_vertices: Option<usize>,
    n_added: usize,
    phantom_p: PhantomData<P>,
}

impl<P, R> OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            i_line: 0,
            line_buffer: Vec::new(),
            off_seen: false,
            n_vertices: None,
            n_added: 0,
            phantom_p: PhantomData,
        }
    }
}

impl<P, R> Iterator for OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = OffResult<ReserveOrData<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if !self.off_seen && line.starts_with(b"OFF") {
                self.off_seen = true;
                continue;
            }

            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }

            if self.n_vertices.is_none() {
                let mut words = to_words_skip_empty(line);
                match words
                    .next()
                    .and_then(|word| from_ascii(word))
                    .ok_or(OffError::VertexCount)
                    .line(self.i_line, line)
                {
                    Ok(n) => {
                        self.n_vertices = Some(n);
                        return Some(Ok(ReserveOrData::Reserve(self.n_vertices.unwrap())));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }

            // safe since checked above
            if self.n_added < self.n_vertices.unwrap() {
                self.n_added += 1;
                return Some(fetch_vertex(line, self.i_line).map(|x| ReserveOrData::Data(x)));
            } else {
                return None;
            }
        }
        None
    }
}

impl<P, R> FusedIterator for OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load points from a .off file
pub struct OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    i_line: usize,
    line_buffer: Vec<u8>,
    off_seen: bool,
    counts: Option<[usize; 2]>,
    n_vertices_added: usize,
    phantom_p: PhantomData<P>,
}

impl<P, R> OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            i_line: 0,
            line_buffer: Vec::new(),
            off_seen: false,
            counts: None,
            n_vertices_added: 0,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_face(line: &[u8], i_line: usize) -> OffResult<[usize; 3]> {
        let mut words = to_words_skip_empty(line);

        let count_face = words
            .next()
            .ok_or(OffError::FaceVertexCount)
            .line(i_line, line)?;

        if count_face == b"3" {
            let a = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(OffError::Face)
                .line(i_line, line)?;

            let b = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(OffError::Face)
                .line(i_line, line)?;

            let c = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(OffError::Face)
                .line(i_line, line)?;

            Ok([a, b, c])
        } else {
            Err(OffError::FaceVertexCount).line(i_line, line)
        }
    }

    #[inline(always)]
    fn fetch_counts(line: &[u8], i_line: usize) -> OffResult<[usize; 2]> {
        let mut words = to_words_skip_empty(line);
        let n_vertices = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(OffError::VertexCount)
            .line(i_line, line)?;
        let n_faces = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(OffError::FaceCount)
            .line(i_line, line)?;

        Ok([n_vertices, n_faces])
    }
}

impl<P, R> Iterator for OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = OffResult<FaceOrData<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if !self.off_seen && line.starts_with(b"OFF") {
                self.off_seen = true;
                continue;
            }

            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }

            if self.counts.is_none() {
                match Self::fetch_counts(line, self.i_line) {
                    Ok(counts) => {
                        self.counts = Some(counts);
                        return Some(Ok(FaceOrData::ReserveDataFaces(counts[0], counts[1])));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }

            // safe since checked above
            if self.n_vertices_added < self.counts.unwrap()[0] {
                self.n_vertices_added += 1;
                return Some(fetch_vertex(line, self.i_line).map(|x| FaceOrData::Data(x)));
            } else {
                return Some(Self::fetch_face(line, self.i_line).map(|x| FaceOrData::Face(x)));
            }
        }

        None
    }
}

impl<P, R> FusedIterator for OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the off file format
pub fn load_off_mesh<EM, P, R>(read: R, mesh: &mut EM) -> OffResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let iterator = OffMeshIterator::<P, R>::new(read);

    for rd in iterator {
        match rd? {
            FaceOrData::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .map_err(|_| OffError::InvalidMeshIndices)
                    .simple()?;
            }
            FaceOrData::ReserveDataFaces(n_vertices, n_faces) => {
                mesh.reserve_vertices(n_vertices);
                mesh.reserve_faces(n_faces);
            }
            FaceOrData::Data(x) => {
                mesh.add_vertex(x);
            }
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .off file format
pub fn load_off_points<IP, P, R>(read: R, ip: &mut IP) -> OffResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = OffPointsIterator::new(read);

    for rd in iterator {
        match rd? {
            ReserveOrData::Reserve(x) => ip.reserve(x),
            ReserveOrData::Data(x) => ip.push(x),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .off file operations
pub enum OffError {
    AccessFile,
    InvalidMeshIndices,
    VertexCount,
    FaceCount,
    Vertex,
    Face,
    FaceVertexCount,
}

/// Result type for .off file operations
pub type OffResult<T> = IOResult<T, OffError>;

impl fmt::Debug for OffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::VertexCount => write!(f, "Unable to parse vertex count"),
            Self::FaceCount => write!(f, "Unable to parse face count"),
            Self::Vertex => write!(f, "Unable to parse vertex"),
            Self::Face => write!(f, "Unable to parse face"),
            Self::FaceVertexCount => write!(f, "Unable to parse vertex count of face"),
            Self::InvalidMeshIndices => write!(f, "File contains invalid mesh indices"),
        }
    }
}

impl fmt::Display for OffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ioError> for OffError {
    fn from(_error: ioError) -> Self {
        OffError::AccessFile
    }
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_vertex<P>(line: &[u8], i_line: usize) -> OffResult<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    let x = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(OffError::Vertex)
        .line(i_line, line)?;

    let y = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(OffError::Vertex)
        .line(i_line, line)?;

    let z = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(OffError::Vertex)
        .line(i_line, line)?;

    Ok(P::new(x, y, z))
}
