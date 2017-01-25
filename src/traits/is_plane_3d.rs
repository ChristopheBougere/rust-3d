/*
Copyright 2016 Martin Buck
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

//! Module containing the IsPlane3D trait used for planes within 3D space

use traits::is_3d::*;
use traits::is_normalized_3d::*;


pub trait IsPlane3D<P,N> where
    P: Is3D,
    N: IsNormalized3D {

    fn new() -> Box<Self>;

    fn build(origin: P, u: N, v: N) -> Box<Self>;

    fn origin(&self) -> P;

    fn u(&self) -> N;

    fn v(&self) -> N;
}
