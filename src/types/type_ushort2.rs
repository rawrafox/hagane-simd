use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ushort2(pub u16, pub u16);
pub type vector_ushort2 = ushort2;

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

impl std::ops::Index<u32> for ushort2 {
  type Output = u16;

  #[inline]
  fn index(&self, index: u32) -> &u16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ushort2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u16) -> Self {
    return unsafe { simd_add(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Add<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn add(self, other: ushort2) -> ushort2 {
    return unsafe { simd_add(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Sub for ushort2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u16) -> Self {
    return unsafe { simd_sub(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Sub<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn sub(self, other: ushort2) -> ushort2 {
    return unsafe { simd_sub(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Mul for ushort2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u16) -> Self {
    return unsafe { simd_mul(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Mul<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn mul(self, other: ushort2) -> ushort2 {
    return unsafe { simd_mul(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Div for ushort2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u16) -> Self {
    return unsafe { simd_div(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Div<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn div(self, other: ushort2) -> ushort2 {
    return unsafe { simd_div(ushort2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ushort2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u16) -> Self {
    return unsafe { simd_and(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn bitand(self, other: ushort2) -> ushort2 {
    return unsafe { simd_and(ushort2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ushort2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u16) -> Self {
    return unsafe { simd_or(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::BitOr<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn bitor(self, other: ushort2) -> ushort2 {
    return unsafe { simd_or(ushort2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ushort2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u16) -> Self {
    return unsafe { simd_xor(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::BitXor<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn bitxor(self, other: ushort2) -> ushort2 {
    return unsafe { simd_xor(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Shl<ushort2> for ushort2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn shl(self, other: ushort2) -> ushort2 {
    return unsafe { simd_shl(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort2> for ushort2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort2::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort2> for u16 {
  type Output = ushort2;

  #[inline]
  fn shr(self, other: ushort2) -> ushort2 {
    return unsafe { simd_shr(ushort2::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short2::all(ushort2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short2::all(ushort2::ne(*self, *other));
  }
}

impl Dot for ushort2 {
  type Output = u16;

  #[inline]
  fn dot(self, other: ushort2) -> u16 {
    return ushort2::reduce_add(self * other);
  }
}

impl ushort2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> ushort2 {
    return ushort2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u16) -> ushort2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ushort2, y: ushort2) -> short2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn max(x: ushort2, y: ushort2) -> ushort2 {
    return ushort2::bitselect(x, y, ushort2::gt(y, x));
  }

  #[inline]
  pub fn min(x: ushort2, y: ushort2) -> ushort2 {
    return ushort2::bitselect(x, y, ushort2::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ushort2, min: ushort2, max: ushort2) -> ushort2 {
    return ushort2::min(ushort2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ushort2) -> u16 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: ushort2) -> u16 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: ushort2) -> u16 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: ushort2) -> bool {
    return (x.0 & x.1) & 0x8000 != 0;
  }

  #[inline]
  pub fn any(x: ushort2) -> bool {
    return (x.0 | x.1) & 0x8000 != 0;
  }

  #[inline]
  pub fn bitselect(x: ushort2, y: ushort2, z: short2) -> ushort2 {
    return ushort2::bitcast(short2::bitselect(short2::bitcast(x), short2::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> ushort1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> ushort1 {
    return self.1;
  }
}
