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

//! IsRandomAccessible trait used for collections of elements which are random accessible

use std::ops::Index;
use std::ops::IndexMut;

/// IsRandomAccessible is a trait used for collections of elements which are random accessible
pub trait IsRandomAccessible<T> : Index<usize, Output=T> + IndexMut<usize> {
    /// Should return the number of elements within the collection
    fn len(&self) -> usize;
}