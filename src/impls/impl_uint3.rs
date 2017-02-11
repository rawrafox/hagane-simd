use std;
use ::*;
use ::simd::*;

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

  fn simd_cast<T, U>(x: T) -> U;
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
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uint3 {
  type Scalar = u32;
  type Boolean = int3;

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return simd::bitselect(simd::gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return simd::bitselect(simd::lt(other, self), self, other);
  }
}

impl simd::Dot for uint3 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for uint3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & 0x80000000 != 0;
  }
}

impl simd::Reduce for uint3 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(simd::reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(simd::reduce_max(self.lo()), self.2);
  }
}

impl uint3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u32) -> Self {
    return uint3(x, x, x);
  }

  #[inline]
  pub fn madd(x: uint3, y: uint3, z: uint3) -> uint3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: uint3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: uint3) -> char3 {
    return uint3::to_char(simd::min(x, uint3::broadcast(std::i8::MAX as u32)));
  }

  #[inline]
  pub fn to_uchar(x: uint3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: uint3) -> uchar3 {
    return uint3::to_uchar(simd::min(x, uint3::broadcast(std::u8::MAX as u32)));
  }

  #[inline]
  pub fn to_short(x: uint3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: uint3) -> short3 {
    return uint3::to_short(simd::min(x, uint3::broadcast(std::i16::MAX as u32)));
  }

  #[inline]
  pub fn to_ushort(x: uint3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: uint3) -> ushort3 {
    return uint3::to_ushort(simd::min(x, uint3::broadcast(std::u16::MAX as u32)));
  }

  #[inline]
  pub fn to_int(x: uint3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: uint3) -> int3 {
    return uint3::to_int(simd::min(x, uint3::broadcast(std::i32::MAX as u32)));
  }

  #[inline]
  pub fn to_uint(x: uint3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: uint3) -> uint3 {
    return x;
  }

  #[inline]
  pub fn to_float(x: uint3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: uint3) -> long3 {
    return long3(x.0 as i64, x.1 as i64, x.2 as i64);
  }

  #[inline]
  pub fn to_long_sat(x: uint3) -> long3 {
    return uint3::to_long(simd::min(x, uint3::broadcast(std::i64::MAX as u32)));
  }

  #[inline]
  pub fn to_ulong(x: uint3) -> ulong3 {
    return ulong3(x.0 as u64, x.1 as u64, x.2 as u64);
  }

  #[inline]
  pub fn to_ulong_sat(x: uint3) -> ulong3 {
    return uint3::to_ulong(x);
  }

  #[inline]
  pub fn to_double(x: uint3) -> double3 {
    return double3(x.0 as f64, x.1 as f64, x.2 as f64);
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
