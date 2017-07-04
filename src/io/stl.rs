/*
Copyright 2017 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Module for IO operations of the stl file format

use result::*;
use traits::is_3d::*;
use traits::is_buildable_3d::*;
use traits::is_mesh_3d::*;

use std::io::prelude::*;
use std::fs::File;

/// Saves an IsMesh3D in the ASCII .stl file format
pub fn save_stl_ascii<P>(mesh: &IsMesh3D<P>, filepath: &str) -> Result<()> where
    P: IsBuildable3D {

    let mut f = File::create(filepath).map_err(|e| e.to_error_kind())?;

    f.write_all(b"solid STL generated by rust-3d\n").map_err(|e| e.to_error_kind())?;

    for i in 0..mesh.num_faces() {
        let (v1, v2, v3) = mesh.face_vertices(i)?;
        let n = mesh.face_normal(i)?;
        let buffer = "facet normal ".to_string() + &str_exp(&n) + "\n"
                       + "    outer loop\n"
                       + "        vertex " + &str_exp(&v1) + "\n"
                       + "        vertex " + &str_exp(&v2) + "\n"
                       + "        vertex " + &str_exp(&v3) + "\n"
                       + "    endloop\n"
                       + "endfacet\n";
        f.write_all(buffer.as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    f.write_all(b"endsolid STL generated by rust-3d\n").map_err(|e| e.to_error_kind())
}

fn str_exp<P>(p: &P) -> String where
    P: Is3D {

    format!("{:e} {:e} {:e}", p.x(), p.y(), p.z()).to_string()
}
