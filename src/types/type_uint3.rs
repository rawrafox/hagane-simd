use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct uint3(pub u32, pub u32, pub u32);
pub type vector_uint3 = uint3;

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

impl std::ops::Index<u32> for uint3 {
  type Output = u32;

  #[inline]
  fn index(&self, index: u32) -> &u32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for uint3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn add(self, other: u32) -> Self {
    return unsafe { simd_add(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Add<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn add(self, other: uint3) -> uint3 {
    return unsafe { simd_add(uint3::broadcast(self), other) };
  }
}

impl std::ops::Sub for uint3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u32) -> Self {
    return unsafe { simd_sub(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Sub<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn sub(self, other: uint3) -> uint3 {
    return unsafe { simd_sub(uint3::broadcast(self), other) };
  }
}

impl std::ops::Mul for uint3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u32) -> Self {
    return unsafe { simd_mul(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Mul<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn mul(self, other: uint3) -> uint3 {
    return unsafe { simd_mul(uint3::broadcast(self), other) };
  }
}

impl std::ops::Div for uint3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn div(self, other: u32) -> Self {
    return unsafe { simd_div(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Div<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn div(self, other: uint3) -> uint3 {
    return unsafe { simd_div(uint3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uint3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u32) -> Self {
    return unsafe { simd_and(self, uint3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn bitand(self, other: uint3) -> uint3 {
    return unsafe { simd_and(uint3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uint3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u32) -> Self {
    return unsafe { simd_or(self, uint3::broadcast(other)) };
  }
}

impl std::ops::BitOr<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn bitor(self, other: uint3) -> uint3 {
    return unsafe { simd_or(uint3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uint3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u32) -> Self {
    return unsafe { simd_xor(self, uint3::broadcast(other)) };
  }
}

impl std::ops::BitXor<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn bitxor(self, other: uint3) -> uint3 {
    return unsafe { simd_xor(uint3::broadcast(self), other) };
  }
}

impl std::ops::Shl<uint3> for uint3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u32) -> Self {
    return unsafe { simd_shl(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Shl<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn shl(self, other: uint3) -> uint3 {
    return unsafe { simd_shl(uint3::broadcast(self), other) };
  }
}

impl std::ops::Shr<uint3> for uint3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u32> for uint3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u32) -> Self {
    return unsafe { simd_shr(self, uint3::broadcast(other)) };
  }
}

impl std::ops::Shr<uint3> for u32 {
  type Output = uint3;

  #[inline]
  fn shr(self, other: uint3) -> uint3 {
    return unsafe { simd_shr(uint3::broadcast(self), other) };
  }
}

impl std::ops::Not for uint3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u32::MAX;
  }
}

impl PartialEq for uint3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int3::all(uint3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int3::all(uint3::ne(*self, *other));
  }
}

impl Dot for uint3 {
  type Output = u32;

  #[inline]
  fn dot(self, other: uint3) -> u32 {
    return uint3::reduce_add(self * other);
  }
}

impl uint3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> uint3 {
    return uint3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u32) -> uint3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: uint3, y: uint3) -> int3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: uint3, y: uint3, z: uint3) -> uint3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: uint3) -> uint3 {
    return x;
  }

  #[inline]
  pub fn max(x: uint3, y: uint3) -> uint3 {
    return uint3::bitselect(x, y, uint3::gt(y, x));
  }

  #[inline]
  pub fn min(x: uint3, y: uint3) -> uint3 {
    return uint3::bitselect(x, y, uint3::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: uint3, min: uint3, max: uint3) -> uint3 {
    return uint3::min(uint3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: uint3) -> u32 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: uint3) -> u32 {
    return std::cmp::min(uint2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: uint3) -> u32 {
    return std::cmp::max(uint2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn all(x: uint3) -> bool {
    return (x.0 & x.1 & x.2) & 0x80000000 != 0;
  }

  #[inline]
  pub fn any(x: uint3) -> bool {
    return (x.0 | x.1 | x.2) & 0x80000000 != 0;
  }

  #[inline]
  pub fn bitselect(x: uint3, y: uint3, z: int3) -> uint3 {
    return uint3::bitcast(int3::bitselect(int3::bitcast(x), int3::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> uint2 {
    return uint2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> uint2 {
    return uint2(self.0, self.2);
  }
}
