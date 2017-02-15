use std;
use std::ops::*;

pub trait Scalar : Sized + Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + PartialEq {
  fn max(x: Self, y: Self) -> Self;
  fn min(x: Self, y: Self) -> Self;
}

pub trait FloatScalar : Scalar {
  fn sqrt(x: Self) -> Self;

  fn fract(x: Self) -> Self;
  fn ceil(x: Self) -> Self;
  fn floor(x: Self) -> Self;
  fn trunc(x: Self) -> Self;

  fn sin(x: Self) -> Self;
  fn cos(x: Self) -> Self;
}

pub trait IntegerScalar : Scalar + BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self> + PartialEq {
  const ZERO: Self;
}

impl_scalar!(i8, signed);
impl_scalar!(i16, signed);
impl_scalar!(i32, signed);
impl_scalar!(i64, signed);

impl_scalar!(u8, unsigned);
impl_scalar!(u16, unsigned);
impl_scalar!(u32, unsigned);
impl_scalar!(u64, unsigned);

// impl_scalar!(f16, float);
impl_scalar!(f32, float);
impl_scalar!(f64, float);
