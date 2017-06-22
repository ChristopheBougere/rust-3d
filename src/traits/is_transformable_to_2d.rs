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

//! IsTransFormableTo2D trait used for any type which can be transformed 2D space

use traits::is_buildable_2d::*;
use traits::is_3d::*;

/// IsTransFormableTo2D is a trait used for any type which can be transformed 2D space
pub trait IsTransFormableTo2D : Is3D {
    /// Should create representation of self within the 2D space
    fn transform_to_2d<P>(&self) -> P where
        P: IsBuildable2D;
}