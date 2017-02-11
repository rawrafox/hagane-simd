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
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ushort2 {
  type Scalar = u16;
  type Boolean = short2;

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

impl simd::Dot for ushort2 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for ushort2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & 0x8000 != 0;
  }
}

impl simd::Reduce for ushort2 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }
}

impl ushort2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort2(x, x);
  }

  #[inline]
  pub fn madd(x: ushort2, y: ushort2, z: ushort2) -> ushort2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: ushort2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: ushort2) -> char2 {
    return ushort2::to_char(simd::min(x, ushort2::broadcast(std::i8::MAX as u16)));
  }

  #[inline]
  pub fn to_uchar(x: ushort2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: ushort2) -> uchar2 {
    return ushort2::to_uchar(simd::min(x, ushort2::broadcast(std::u8::MAX as u16)));
  }

  #[inline]
  pub fn to_short(x: ushort2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: ushort2) -> short2 {
    return ushort2::to_short(simd::min(x, ushort2::broadcast(std::i16::MAX as u16)));
  }

  #[inline]
  pub fn to_ushort(x: ushort2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: ushort2) -> ushort2 {
    return x;
  }

  #[inline]
  pub fn to_int(x: ushort2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: ushort2) -> int2 {
    return ushort2::to_int(simd::min(x, ushort2::broadcast(std::i32::MAX as u16)));
  }

  #[inline]
  pub fn to_uint(x: ushort2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: ushort2) -> uint2 {
    return ushort2::to_uint(x);
  }

  #[inline]
  pub fn to_float(x: ushort2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: ushort2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: ushort2) -> long2 {
    return ushort2::to_long(simd::min(x, ushort2::broadcast(std::i64::MAX as u16)));
  }

  #[inline]
  pub fn to_ulong(x: ushort2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: ushort2) -> ulong2 {
    return ushort2::to_ulong(x);
  }

  #[inline]
  pub fn to_double(x: ushort2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> u16 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u16 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u16 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u16 {
    return self.0;
  }
}
