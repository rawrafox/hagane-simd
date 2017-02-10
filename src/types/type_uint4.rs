use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct uint4(pub u32, pub u32, pub u32, pub u32);
pub type vector_uint4 = uint4;

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

impl std::ops::Index<u32> for uint4 {
  type Output = u32;

  #[inline]
  fn index(&self, index: u32) -> &u32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for uint4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u32) -> Self {
    return unsafe { simd_add(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Add<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn add(self, other: uint4) -> uint4 {
    return unsafe { simd_add(uint4::broadcast(self), other) };
  }
}

impl std::ops::Sub for uint4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u32) -> Self {
    return unsafe { simd_sub(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Sub<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn sub(self, other: uint4) -> uint4 {
    return unsafe { simd_sub(uint4::broadcast(self), other) };
  }
}

impl std::ops::Mul for uint4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u32) -> Self {
    return unsafe { simd_mul(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Mul<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn mul(self, other: uint4) -> uint4 {
    return unsafe { simd_mul(uint4::broadcast(self), other) };
  }
}

impl std::ops::Div for uint4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u32) -> Self {
    return unsafe { simd_div(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Div<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn div(self, other: uint4) -> uint4 {
    return unsafe { simd_div(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uint4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u32) -> Self {
    return unsafe { simd_and(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitand(self, other: uint4) -> uint4 {
    return unsafe { simd_and(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uint4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u32) -> Self {
    return unsafe { simd_or(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitOr<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitor(self, other: uint4) -> uint4 {
    return unsafe { simd_or(uint4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uint4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u32) -> Self {
    return unsafe { simd_xor(self, uint4::broadcast(other)) };
  }
}

impl std::ops::BitXor<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn bitxor(self, other: uint4) -> uint4 {
    return unsafe { simd_xor(uint4::broadcast(self), other) };
  }
}

impl std::ops::Shl<uint4> for uint4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u32) -> Self {
    return unsafe { simd_shl(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Shl<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn shl(self, other: uint4) -> uint4 {
    return unsafe { simd_shl(uint4::broadcast(self), other) };
  }
}

impl std::ops::Shr<uint4> for uint4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u32> for uint4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u32) -> Self {
    return unsafe { simd_shr(self, uint4::broadcast(other)) };
  }
}

impl std::ops::Shr<uint4> for u32 {
  type Output = uint4;

  #[inline]
  fn shr(self, other: uint4) -> uint4 {
    return unsafe { simd_shr(uint4::broadcast(self), other) };
  }
}

impl std::ops::Not for uint4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u32::MAX;
  }
}

impl PartialEq for uint4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int4::all(uint4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int4::all(uint4::ne(*self, *other));
  }
}

impl Dot for uint4 {
  type Output = u32;

  #[inline]
  fn dot(self, other: uint4) -> u32 {
    return uint4::reduce_add(self * other);
  }
}

impl uint4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> uint4 {
    return uint4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u32) -> uint4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: uint4, y: uint4) -> int4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn max(x: uint4, y: uint4) -> uint4 {
    return uint4::bitselect(x, y, uint4::gt(y, x));
  }

  #[inline]
  pub fn min(x: uint4, y: uint4) -> uint4 {
    return uint4::bitselect(x, y, uint4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: uint4, min: uint4, max: uint4) -> uint4 {
    return uint4::min(uint4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: uint4) -> u32 {
    return uint2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: uint4) -> u32 {
    return uint2::reduce_min(uint2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: uint4) -> u32 {
    return uint2::reduce_max(uint2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: uint4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & 0x80000000 != 0;
  }

  #[inline]
  pub fn any(x: uint4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & 0x80000000 != 0;
  }

  #[inline]
  pub fn bitselect(x: uint4, y: uint4, z: int4) -> uint4 {
    return uint4::bitcast(int4::bitselect(int4::bitcast(x), int4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, self.3);
  }
}
