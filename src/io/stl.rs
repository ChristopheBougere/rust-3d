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

//! Module for IO operations of the stl file format

use crate::prelude::*;

use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};

/// Saves an IsMesh3D in the ASCII .stl file format
pub fn save_stl_ascii<M, P>(mesh: &M, filepath: &str) -> Result<()>
where
    M: IsMesh3D<P>,
    P: IsBuildable3D,
{
    let mut f = BufWriter::new(File::create(filepath).map_err(|e| e.to_error_kind())?);

    f.write_all(b"solid STL generated by rust-3d\n")
        .map_err(|e| e.to_error_kind())?;

    for i in 0..mesh.num_faces() {
        let [v1, v2, v3] = mesh.face_vertices(FId { val: i })?;
        let n = mesh.face_normal(FId { val: i })?;
        let buffer = "facet normal ".to_string()
            + &str_exp(&n)
            + "\n"
            + "    outer loop\n"
            + "        vertex "
            + &str_exp(&v1)
            + "\n"
            + "        vertex "
            + &str_exp(&v2)
            + "\n"
            + "        vertex "
            + &str_exp(&v3)
            + "\n"
            + "    endloop\n"
            + "endfacet\n";
        f.write_all(buffer.as_bytes())
            .map_err(|e| e.to_error_kind())?;
    }
    f.write_all(b"endsolid STL generated by rust-3d\n")
        .map_err(|e| e.to_error_kind())
}

fn str_exp<P>(p: &P) -> String
where
    P: Is3D,
{
    format!("{:e} {:e} {:e}", p.x(), p.y(), p.z()).to_string()
}
