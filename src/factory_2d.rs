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

use std::f64::consts::PI;

use positive::*;
use point_2d::*;
use point_cloud_2d::*;
use traits::is_buildable_2d::*;

///@todo entire file has to be added to tests
///@todo add some type level checks like diameter > 0 etc., or return Option types (similar to flaggedT?)
///@todo define trait for pc2d factories, later for 3d as well
///@todo remove center as param and create all around origin
///@todo correct reserving
///@todo order parameters (e.g. center and n_points always first)

pub fn origin() -> Box<Point2D> {
    Point2D::build(0.0, 0.0)
}

pub fn rectangle<P>(center: &P, width: Positive, height: Positive) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let w = width.get();
    let h = height.get();
    pc.push(*P::build(center.x() - w / 2.0, center.y() - h / 2.0));
    pc.push(*P::build(center.x() + w / 2.0, center.y() - h / 2.0));
    pc.push(*P::build(center.x() + w / 2.0, center.y() + h / 2.0));
    pc.push(*P::build(center.x() - w / 2.0, center.y() + h / 2.0));
    Box::new(pc)
}

pub fn involut_circle<P>(center: &P, diameter: Positive, n_points: usize, radians_start: f64, radians_end: f64) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    //@todo reserve
    let mut pc = PointCloud2D::new();
    let d = diameter.get();
    let p_dist = (radians_end - radians_start).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let current = (i as f64) * p_dist;
        pc.push(*P::build(center.x() + d/2.0 * (current.cos() + current * current.sin()),
                          center.y() + d/2.0 * (current.sin() - current * current.cos())));
    }
    Box::new(pc)
}

pub fn arc<P>(center: &P, diameter: Positive, n_points: usize, radians_start: f64, radians_end: f64) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let d = diameter.get();
    let p_dist = (radians_end - radians_start).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let radians = radians_start + (i as f64) * p_dist;
        pc.push(*P::build(center.x() + d/2.0 * radians.cos(),
                          center.y() + d/2.0 * radians.sin()));
    }
    Box::new(pc)
}

pub fn ellipse<P>(center: &P, ap: Positive, bp: Positive, n_points: usize) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let p_dist = PI / (n_points - 1) as f64;
    let a = ap.get();
    let b = bp.get();
    let angle: f64 = 0.0; //@todo as parameter? or just drop from formulas?

    for i in 0..n_points {
        let radians = (i as f64) * p_dist;
        pc.push(*P::build(center.x() + a * radians.cos() * angle.cos() - b * radians.sin() * angle.sin(),
                          center.y() + a * radians.cos() * angle.sin() + b * radians.sin() * angle.cos()));
    }
    Box::new(pc)
}
