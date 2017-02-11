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

impl std::ops::Add for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Add<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn add(self, other: ulong2) -> ulong2 {
    return unsafe { simd_add(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn sub(self, other: ulong2) -> ulong2 {
    return unsafe { simd_sub(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn mul(self, other: ulong2) -> ulong2 {
    return unsafe { simd_mul(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Div<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn div(self, other: ulong2) -> ulong2 {
    return unsafe { simd_div(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitand(self, other: ulong2) -> ulong2 {
    return unsafe { simd_and(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_or(ulong2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn bitxor(self, other: ulong2) -> ulong2 {
    return unsafe { simd_xor(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shl(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shl(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong2> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong2::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong2> for u64 {
  type Output = ulong2;

  #[inline]
  fn shr(self, other: ulong2) -> ulong2 {
    return unsafe { simd_shr(ulong2::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ulong2 {
  type Scalar = u64;
  type Boolean = long2;

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
    return ulong2::to_char(simd::min(self, ulong2::broadcast(std::i8::MAX as u64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return ulong2::to_uchar(simd::min(self, ulong2::broadcast(std::u8::MAX as u64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return ulong2::to_short(simd::min(self, ulong2::broadcast(std::i16::MAX as u64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return ulong2::to_ushort(simd::min(self, ulong2::broadcast(std::u16::MAX as u64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return ulong2::to_int(simd::min(self, ulong2::broadcast(std::i32::MAX as u64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return ulong2::to_uint(simd::min(self, ulong2::broadcast(std::u32::MAX as u64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return ulong2::to_long(simd::min(self, ulong2::broadcast(std::i64::MAX as u64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return self;
  }
}

impl simd::Dot for ulong2 {
  type DotProduct = u64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for ulong2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & 0x8000000000000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & 0x8000000000000000 != 0;
  }
}

impl simd::Reduce for ulong2 {
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

impl ulong2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> Self {
    return ulong2(x, x);
  }

  #[inline]
  pub fn madd(x: ulong2, y: ulong2, z: ulong2) -> ulong2 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> u64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> u64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> u64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> u64 {
    return self.0;
  }
}
