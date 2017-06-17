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

//! rust-3d trait implementations for the standard VecDeque

use std::collections::VecDeque;

use result::*;
use view::*;
use traits::is_random_accessible::*;
use traits::is_random_insertible::*;
use traits::is_view_buildable::*;

impl<T> IsRandomAccessible<T> for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> IsRandomInsertible<T> for VecDeque<T> {
    fn push(&mut self, x: T) {
        self.push_back(x)
    }

    fn insert(&mut self, index: usize, x: T) -> Result<()> {
        if index >= self.len() {
            return Err(ErrorKind::IndexOutOfBounds);
        }
        self.insert(index, x);
        Ok(())
    }
}

impl<T> IsViewBuildable for VecDeque<T> where
    T: Clone {

    fn apply_view(&mut self, view: &View) -> Result<()> {
        match view {
            &View::Full => { Ok(()) }
            &View::Restricted(ref indices) => {
                let n = self.len();
                if indices.iter().any(|x| x >= &n) {
                    return Err(ErrorKind::IndexOutOfBounds);
                }
                let mut new_data = VecDeque::new();
                for (i, p) in self.iter().enumerate() {
                    if indices.contains(&i) {
                        new_data.push(p.clone());
                    }
                }
                *self = new_data;
                Ok(())
            }
        }
    }

    fn from_view(&self, view: &View) -> Result<Box<Self>> {
        let mut cloned = self.clone();
        cloned.apply_view(view)?;
        Ok(Box::new(cloned))
    }
}
