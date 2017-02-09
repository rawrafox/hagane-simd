use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct short4(pub i16, pub i16, pub i16, pub i16);
pub type vector_short4 = short4;

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

impl std::ops::Index<u32> for short4 {
  type Output = i16;

  #[inline]
  fn index(&self, index: u32) -> &i16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for short4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short4::broadcast(other)) };
  }
}

impl std::ops::Add<short4> for i16 {
  type Output = short4;

  #[inline]
  fn add(self, other: short4) -> short4 {
    return unsafe { simd_add(short4::broadcast(self), other) };
  }
}

impl std::ops::Sub for short4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short4::broadcast(other)) };
  }
}

impl std::ops::Sub<short4> for i16 {
  type Output = short4;

  #[inline]
  fn sub(self, other: short4) -> short4 {
    return unsafe { simd_sub(short4::broadcast(self), other) };
  }
}

impl std::ops::Mul for short4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short4::broadcast(other)) };
  }
}

impl std::ops::Mul<short4> for i16 {
  type Output = short4;

  #[inline]
  fn mul(self, other: short4) -> short4 {
    return unsafe { simd_mul(short4::broadcast(self), other) };
  }
}

impl std::ops::Div for short4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short4::broadcast(other)) };
  }
}

impl std::ops::Div<short4> for i16 {
  type Output = short4;

  #[inline]
  fn div(self, other: short4) -> short4 {
    return unsafe { simd_div(short4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitand(self, other: short4) -> short4 {
    return unsafe { simd_and(short4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitOr<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitor(self, other: short4) -> short4 {
    return unsafe { simd_or(short4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitXor<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitxor(self, other: short4) -> short4 {
    return unsafe { simd_xor(short4::broadcast(self), other) };
  }
}

impl std::ops::Shl<short4> for short4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short4::broadcast(other)) };
  }
}

impl std::ops::Shl<short4> for i16 {
  type Output = short4;

  #[inline]
  fn shl(self, other: short4) -> short4 {
    return unsafe { simd_shl(short4::broadcast(self), other) };
  }
}

impl std::ops::Shr<short4> for short4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short4::broadcast(other)) };
  }
}

impl std::ops::Shr<short4> for i16 {
  type Output = short4;

  #[inline]
  fn shr(self, other: short4) -> short4 {
    return unsafe { simd_shr(short4::broadcast(self), other) };
  }
}

impl std::ops::Not for short4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ short4::broadcast(-1);
  }
}

impl PartialEq for short4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short4::all(short4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short4::all(short4::ne(*self, *other));
  }
}

impl Dot for short4 {
  type Output = i16;

  #[inline]
  fn dot(self, other: short4) -> i16 {
    return short4::reduce_add(self * other);
  }
}

impl short4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> short4 {
    return short4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i16) -> short4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: short4, y: short4) -> short4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: short4, y: short4) -> short4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: short4, y: short4) -> short4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: short4, y: short4) -> short4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: short4, y: short4) -> short4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: short4, y: short4) -> short4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: short4) -> short4 {
    return short4(x.0.abs(), x.1.abs(), x.2.abs(), x.3.abs());
  }

  #[inline]
  pub fn max(x: short4, y: short4) -> short4 {
    return short4(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1), std::cmp::max(x.2, y.2), std::cmp::max(x.3, y.3));
  }

  #[inline]
  pub fn min(x: short4, y: short4) -> short4 {
    return short4(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1), std::cmp::min(x.2, y.2), std::cmp::min(x.3, y.3));
  }

  #[inline]
  pub fn clamp(x: short4, min: short4, max: short4) -> short4 {
    return short4::min(short4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: short4) -> i16 {
    return short2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: short4) -> i16 {
    return short2::reduce_min(short2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: short4) -> i16 {
    return short2::reduce_max(short2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: short4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn any(x: short4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & std::i16::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: short4, y: short4, z: short4) -> short4 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, self.3);
  }
}
