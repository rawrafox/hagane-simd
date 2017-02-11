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
}

impl std::ops::Add for uchar2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u8) -> Self {
    return unsafe { simd_add(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Add<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn add(self, other: uchar2) -> uchar2 {
    return unsafe { simd_add(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Sub for uchar2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u8) -> Self {
    return unsafe { simd_sub(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Sub<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn sub(self, other: uchar2) -> uchar2 {
    return unsafe { simd_sub(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Mul for uchar2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u8) -> Self {
    return unsafe { simd_mul(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Mul<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn mul(self, other: uchar2) -> uchar2 {
    return unsafe { simd_mul(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Div for uchar2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u8) -> Self {
    return unsafe { simd_div(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Div<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn div(self, other: uchar2) -> uchar2 {
    return unsafe { simd_div(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uchar2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u8) -> Self {
    return unsafe { simd_and(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitand(self, other: uchar2) -> uchar2 {
    return unsafe { simd_and(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uchar2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u8) -> Self {
    return unsafe { simd_or(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitOr<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitor(self, other: uchar2) -> uchar2 {
    return unsafe { simd_or(uchar2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uchar2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u8) -> Self {
    return unsafe { simd_xor(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::BitXor<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn bitxor(self, other: uchar2) -> uchar2 {
    return unsafe { simd_xor(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Shl<uchar2> for uchar2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u8) -> Self {
    return unsafe { simd_shl(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Shl<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn shl(self, other: uchar2) -> uchar2 {
    return unsafe { simd_shl(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Shr<uchar2> for uchar2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u8> for uchar2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u8) -> Self {
    return unsafe { simd_shr(self, uchar2::broadcast(other)) };
  }
}

impl std::ops::Shr<uchar2> for u8 {
  type Output = uchar2;

  #[inline]
  fn shr(self, other: uchar2) -> uchar2 {
    return unsafe { simd_shr(uchar2::broadcast(self), other) };
  }
}

impl std::ops::Not for uchar2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u8::MAX;
  }
}

impl PartialEq for uchar2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uchar2 {
  type Scalar = u8;
  type Boolean = char2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

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

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return uchar2::to_char(simd::min(self, uchar2::broadcast(std::i8::MAX as u8)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return self;
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return uchar2::to_short(simd::min(self, uchar2::broadcast(std::i16::MAX as u8)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return uchar2::to_ushort(self);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return uchar2::to_int(simd::min(self, uchar2::broadcast(std::i32::MAX as u8)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return uchar2::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return uchar2::to_long(simd::min(self, uchar2::broadcast(std::i64::MAX as u8)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return uchar2::to_ulong(self);
  }
}

impl simd::Dot for uchar2 {
  type DotProduct = u8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for uchar2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & 0x80 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & 0x80 != 0;
  }
}

impl simd::Reduce for uchar2 {
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

impl uchar2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> Self {
    return uchar2(x, x);
  }

  #[inline]
  pub fn madd(x: uchar2, y: uchar2, z: uchar2) -> uchar2 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> u8 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u8 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u8 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u8 {
    return self.0;
  }
}
