use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut};
use std::ops::{BitAnd, BitAndAssign};
use std::ops::{BitOr, BitOrAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Rem, RemAssign};
use std::ops::{Shl, ShlAssign};
use std::ops::{Shr, ShrAssign};
use std::ops::{Sub, SubAssign};
use std::num::Wrapping;

use serde::Deserialize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
/// Packed color integer.
pub struct ColorInt(pub i32);

impl fmt::Debug for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Binary for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Octal for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::LowerHex for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::UpperHex for ColorInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add for ColorInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_add(rhs.0))
    }
}

impl Add<i32> for ColorInt {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self + ColorInt(rhs)
    }
}

impl AddAssign for ColorInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i32> for ColorInt {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + ColorInt(rhs);
    }
}

impl Sub for ColorInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<i32> for ColorInt {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self - ColorInt(rhs)
    }
}

impl SubAssign for ColorInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i32> for ColorInt {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - ColorInt(rhs);
    }
}

impl Mul for ColorInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul<i32> for ColorInt {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * ColorInt(rhs)
    }
}

impl MulAssign for ColorInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<i32> for ColorInt {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * ColorInt(rhs);
    }
}

impl Div for ColorInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_div(rhs.0))
    }
}

impl Div<i32> for ColorInt {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / ColorInt(rhs)
    }
}

impl DivAssign for ColorInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<i32> for ColorInt {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / ColorInt(rhs);
    }
}

impl Rem for ColorInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ColorInt(self.0.wrapping_rem(rhs.0))
    }
}

impl Rem<i32> for ColorInt {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self::Output {
        self % ColorInt(rhs)
    }
}

impl RemAssign for ColorInt {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl RemAssign<i32> for ColorInt {
    fn rem_assign(&mut self, rhs: i32) {
        *self = *self % ColorInt(rhs);
    }
}

impl BitOr for ColorInt {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let num = Wrapping(self.0) | Wrapping(rhs.0);
        Self(num.0)
    }
}

impl BitOr<i32> for ColorInt {
    type Output = Self;

    fn bitor(self, rhs: i32) -> Self::Output {
        self | ColorInt(rhs)
    }
}

impl BitOrAssign for ColorInt {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitOrAssign<i32> for ColorInt {
    fn bitor_assign(&mut self, rhs: i32) {
        *self = *self | ColorInt(rhs);
    }
}

impl BitAnd for ColorInt {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let num = Wrapping(self.0) & Wrapping(rhs.0);
        Self(num.0)
    }
}

impl BitAnd<i32> for ColorInt {
    type Output = Self;

    fn bitand(self, rhs: i32) -> Self::Output {
        self & ColorInt(rhs)
    }
}

impl BitAndAssign for ColorInt {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitAndAssign<i32> for ColorInt {
    fn bitand_assign(&mut self, rhs: i32) {
        *self = *self & ColorInt(rhs);
    }
}

impl Shl<usize> for ColorInt {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let num = Wrapping(self.0) << rhs;
        Self(num.0)
    }
}

impl ShlAssign<usize> for ColorInt {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl Shr<usize> for ColorInt {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let num = Wrapping(self.0) >> rhs;
        Self(num.0)
    }
}

impl ShrAssign<usize> for ColorInt {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl From<i32> for ColorInt {
    fn from(value: i32) -> Self {
        ColorInt(value)
    }
}

impl Deref for ColorInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ColorInt {
    pub fn alpha(&self) -> u8 {
        ((*self >> 24) & 0xff).0 as u8
    }

    pub fn red(&self) -> u8 {
        ((*self >> 16) & 0xFF).0 as u8
    }

    pub fn green(&self) -> u8 {
        ((*self >> 8) & 0xFF).0 as u8
    }

    pub fn blue(&self) -> u8 {
        (*self & 0xFF).0 as u8
    }
}