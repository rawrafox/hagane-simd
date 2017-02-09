use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct short3(pub i16, pub i16, pub i16);
pub type vector_short3 = short3;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for short3 {
  type Output = i16;

  #[inline]
  fn index(&self, index: u32) -> &i16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for short3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short3::broadcast(other)) };
  }
}

impl std::ops::Add<short3> for i16 {
  type Output = short3;

  #[inline]
  fn add(self, other: short3) -> short3 {
    return unsafe { simd_add(short3::broadcast(self), other) };
  }
}

impl std::ops::Sub for short3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short3::broadcast(other)) };
  }
}

impl std::ops::Sub<short3> for i16 {
  type Output = short3;

  #[inline]
  fn sub(self, other: short3) -> short3 {
    return unsafe { simd_sub(short3::broadcast(self), other) };
  }
}

impl std::ops::Mul for short3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short3::broadcast(other)) };
  }
}

impl std::ops::Mul<short3> for i16 {
  type Output = short3;

  #[inline]
  fn mul(self, other: short3) -> short3 {
    return unsafe { simd_mul(short3::broadcast(self), other) };
  }
}

impl std::ops::Div for short3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short3::broadcast(other)) };
  }
}

impl std::ops::Div<short3> for i16 {
  type Output = short3;

  #[inline]
  fn div(self, other: short3) -> short3 {
    return unsafe { simd_div(short3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitand(self, other: short3) -> short3 {
    return unsafe { simd_and(short3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitOr<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitor(self, other: short3) -> short3 {
    return unsafe { simd_or(short3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitXor<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitxor(self, other: short3) -> short3 {
    return unsafe { simd_xor(short3::broadcast(self), other) };
  }
}

impl std::ops::Shl<short3> for short3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short3::broadcast(other)) };
  }
}

impl std::ops::Shl<short3> for i16 {
  type Output = short3;

  #[inline]
  fn shl(self, other: short3) -> short3 {
    return unsafe { simd_shl(short3::broadcast(self), other) };
  }
}

impl std::ops::Shr<short3> for short3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short3::broadcast(other)) };
  }
}

impl std::ops::Shr<short3> for i16 {
  type Output = short3;

  #[inline]
  fn shr(self, other: short3) -> short3 {
    return unsafe { simd_shr(short3::broadcast(self), other) };
  }
}

impl std::ops::Not for short3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ short3::broadcast(-1);
  }
}

impl PartialEq for short3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short3::all(short3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short3::all(short3::ne(*self, *other));
  }
}

impl Dot for short3 {
  type Output = i16;

  #[inline]
  fn dot(self, other: short3) -> i16 {
    return short3::reduce_add(self * other);
  }
}

impl short3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> short3 {
    return short3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i16) -> short3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: short3, y: short3) -> short3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: short3, y: short3) -> short3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: short3, y: short3) -> short3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: short3, y: short3) -> short3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: short3, y: short3) -> short3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: short3, y: short3) -> short3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: short3) -> short3 {
    return short3(x.0.abs(), x.1.abs(), x.2.abs());
  }

  #[inline]
  pub fn max(x: short3, y: short3) -> short3 {
    return short3(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1), std::cmp::max(x.2, y.2));
  }

  #[inline]
  pub fn min(x: short3, y: short3) -> short3 {
    return short3(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1), std::cmp::min(x.2, y.2));
  }

  #[inline]
  pub fn clamp(x: short3, min: short3, max: short3) -> short3 {
    return short3::min(short3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: short3) -> i16 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: short3) -> i16 {
    return std::cmp::min(short2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: short3) -> i16 {
    return std::cmp::max(short2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn all(x: short3) -> bool {
    return (x.0 & x.1 & x.2) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn any(x: short3) -> bool {
    return (x.0 | x.1 | x.2) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: short3, y: short3, z: short3) -> short3 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, 0);
  }
}
