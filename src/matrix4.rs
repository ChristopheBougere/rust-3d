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

//! Module containing Matrix4, a matrix with 4 rows and columns

use result::*;
use point_3d::*;
use traits::is_3d::*;
use traits::is_buildable_nd::*;
use traits::is_buildable_3d::*;
use functions::cross;

/// Matrix4, a matrix with 4 rows and columns
pub struct Matrix4 {
    pub data: [[f64; 4]; 4]
}

impl Matrix4 {
    /// Creates a new matrix which does nothing when multiplying by it
    pub fn new() -> Matrix4 {
        Matrix4{
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    /// Creates a new matrix which contains only zeroes
    pub fn zeroes() -> Matrix4 {
        Matrix4{
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }

    /// Creates a new matrix which applies translation
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4{
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    /// Creates a new matrix which applies scaling
    pub fn scale(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4{
            data: [
                [x,   0.0, 0.0, 0.0],
                [0.0, y,   0.0, 0.0],
                [0.0, 0.0, z,   0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    /// Creates a new matrix which applies rotation
    pub fn rotation(rad_x: f64, rad_y: f64, rad_z: f64) -> Matrix4 {
        let (mut mx, mut my, mut mz) = (Matrix4::new(), Matrix4::new(), Matrix4::new());

        mx.data[0][0] = 1.0;     mx.data[0][1] = 0.0;           mx.data[0][2] = 0.0;            mx.data[0][3] = 0.0;
        mx.data[1][0] = 0.0;     mx.data[1][1] = rad_x.cos();    mx.data[1][2] = -rad_x.sin();    mx.data[1][3] = 0.0;
        mx.data[2][0] = 0.0;     mx.data[2][1] = rad_x.sin();    mx.data[2][2] = rad_x.cos();     mx.data[2][3] = 0.0;
        mx.data[3][0] = 0.0;     mx.data[3][1] = 0.0;           mx.data[3][2] = 0.0;            mx.data[3][3] = 1.0;

        my.data[0][0] = rad_y.cos();     my.data[0][1] = 0.0;      my.data[0][2] = rad_y.sin();   my.data[0][3] = 0.0;
        my.data[1][0] = 0.0;            my.data[1][1] = 1.0;      my.data[1][2] = 0.0;          my.data[1][3] = 0.0;
        my.data[2][0] = -rad_y.sin();    my.data[2][1] = 0.0;      my.data[2][2] = rad_y.cos();   my.data[2][3] = 0.0;
        my.data[3][0] = 0.0;            my.data[3][1] = 0.0;      my.data[3][2] = 0.0;          my.data[3][3] = 1.0;

        mz.data[0][0] = rad_z.cos(); mz.data[0][1] = -rad_z.sin();    mz.data[0][2] = 0.0;      mz.data[0][3] = 0.0;
        mz.data[1][0] = rad_z.sin(); mz.data[1][1] = rad_z.cos();     mz.data[1][2] = 0.0;      mz.data[1][3] = 0.0;
        mz.data[2][0] = 0.0;        mz.data[2][1] = 0.0;            mz.data[2][2] = 1.0;      mz.data[2][3] = 0.0;
        mz.data[3][0] = 0.0;        mz.data[3][1] = 0.0;            mz.data[3][2] = 0.0;      mz.data[3][3] = 1.0;

        mx.multiply_m(&my.multiply_m(&mz))
    }

    ///@todo wont have to be of type option once uvec implemented
    /// Creates a new matrix which applies rotation around an axis
    pub fn rotation_axis<P>(axis: &P, rad: f64) -> Result<Matrix4> where
        P: IsBuildable3D {

        let u = axis.clone().normalized()?;
        let mut result = Matrix4::new();
        //@todo needs testing!!!
        result.data[0][0] = rad.cos() + u.x()*u.x()*(1.0 - rad.cos());          result.data[0][1] = u.x()*u.y()*(1.0 -rad.cos()) - u.z()*rad.sin();     result.data[0][2] = u.x()*u.z()*(1.0 - rad.cos()) + u.y()*rad.sin();    result.data[0][3] = 0.0;
        result.data[1][0] = u.y()*u.x()*(1.0 - rad.cos()) + u.z()*rad.sin();    result.data[1][1] = rad.cos() + u.y()*u.y()*(1.0 - rad.cos());          result.data[1][2] = u.y()*u.z()*(1.0 - rad.cos()) - u.x()*rad.sin();    result.data[1][3] = 0.0;
        result.data[2][0] = u.z()*u.x()*(1.0 - rad.cos()) - u.y()*rad.sin();    result.data[2][1] = u.z()*u.y()*(1.0 - rad.cos()) + u.x()*rad.sin();    result.data[2][2] = rad.cos() + u.z()*u.z()*(1.0 - rad.cos());          result.data[2][3] = 0.0;
        result.data[3][0] = 0.0;                                                result.data[3][1] = 0.0;                                                result.data[3][2] = 0.0;                                                result.data[3][3] = 1.0;
        Ok(result)
    }

    /// Creates a new matrix which applies perspective transformation
    pub fn perspective(close: f64, away: f64, fov_rad: f64) -> Matrix4 {
        let range = close - away;
        let tan_fov_half = (fov_rad/2.0).tan();

        let mut result = Matrix4::new();
        result.data[0][0] = 1.0 / (tan_fov_half * away);  result.data[0][1] = 0.0;               result.data[0][2] = 0.0;                      result.data[0][3] = 0.0;
        result.data[1][0] = 0.0;                        result.data[1][1] = 1.0 / tan_fov_half;  result.data[1][2] = 0.0;                      result.data[1][3] = 0.0;
        result.data[2][0] = 0.0;                        result.data[2][1] = 0.0;               result.data[2][2] = (-close - away) / range;  result.data[2][3] = 2.0 * away * close / range;
        result.data[3][0] = 0.0;                        result.data[3][1] = 0.0;               result.data[3][2] = 1.0;                      result.data[3][3] = 1.0;
        result
    }

    //@todo require normalized vectors in these functions
    /// Creates a new matrix which applies a look at transformation
    pub fn look_at<P>(target: &P, up: &P) -> Result<Matrix4> where
        P: IsBuildable3D { //@todo wont have to be an option once unitvector is defined whis is always l > 0 ( l == 1)

        let n = target.clone().normalized()?;
        let u = up.clone().normalized().map(|x| {
            let mut result = *Point3D::new(); //@todo can be dropped?
            result.from(*(cross(&*x, target)));
            result
        })?;
        let v = cross(&*n, &u);

        let mut result = Matrix4::new();
        result.data[0][0] = u.x();  result.data[0][1] = u.y();  result.data[0][2] = u.z();  result.data[0][3] = 0.0;
        result.data[1][0] = v.x();  result.data[1][1] = v.y();  result.data[1][2] = v.z();  result.data[1][3] = 0.0;
        result.data[2][0] = n.x();  result.data[2][1] = n.y();  result.data[2][2] = n.z();  result.data[2][3] = 0.0;
        result.data[3][0] = 0.0;  result.data[3][1] = 0.0;  result.data[3][2] = 0.0;  result.data[3][3] = 1.0;
        Ok(result)
    }

    /// Multiplies this matrix by another
    pub fn multiply_m(&self, other: &Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] =
                    self.data[i][0] * other.data[0][j] +
				    self.data[i][1] * other.data[1][j] +
				    self.data[i][2] * other.data[2][j] +
				    self.data[i][3] * other.data[3][j];
            }
        }
        result
    }

    /// Multiplies all values of this matrix by the factor
    pub fn multiply_f(&self, other: f64) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}
