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

impl std::ops::Add for long4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long4::broadcast(other)) };
  }
}

impl std::ops::Add<long4> for i64 {
  type Output = long4;

  #[inline]
  fn add(self, other: long4) -> long4 {
    return unsafe { simd_add(long4::broadcast(self), other) };
  }
}

impl std::ops::Sub for long4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long4::broadcast(other)) };
  }
}

impl std::ops::Sub<long4> for i64 {
  type Output = long4;

  #[inline]
  fn sub(self, other: long4) -> long4 {
    return unsafe { simd_sub(long4::broadcast(self), other) };
  }
}

impl std::ops::Mul for long4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long4::broadcast(other)) };
  }
}

impl std::ops::Mul<long4> for i64 {
  type Output = long4;

  #[inline]
  fn mul(self, other: long4) -> long4 {
    return unsafe { simd_mul(long4::broadcast(self), other) };
  }
}

impl std::ops::Div for long4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long4::broadcast(other)) };
  }
}

impl std::ops::Div<long4> for i64 {
  type Output = long4;

  #[inline]
  fn div(self, other: long4) -> long4 {
    return unsafe { simd_div(long4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitand(self, other: long4) -> long4 {
    return unsafe { simd_and(long4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitOr<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitor(self, other: long4) -> long4 {
    return unsafe { simd_or(long4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long4::broadcast(other)) };
  }
}

impl std::ops::BitXor<long4> for i64 {
  type Output = long4;

  #[inline]
  fn bitxor(self, other: long4) -> long4 {
    return unsafe { simd_xor(long4::broadcast(self), other) };
  }
}

impl std::ops::Shl<long4> for long4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long4::broadcast(other)) };
  }
}

impl std::ops::Shl<long4> for i64 {
  type Output = long4;

  #[inline]
  fn shl(self, other: long4) -> long4 {
    return unsafe { simd_shl(long4::broadcast(self), other) };
  }
}

impl std::ops::Shr<long4> for long4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long4::broadcast(other)) };
  }
}

impl std::ops::Shr<long4> for i64 {
  type Output = long4;

  #[inline]
  fn shr(self, other: long4) -> long4 {
    return unsafe { simd_shr(long4::broadcast(self), other) };
  }
}

impl std::ops::Not for long4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for long4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for long4 {
  type Scalar = i64;
  type Boolean = long4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 63;
    return (self ^ mask) - mask;
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
  fn to_char_sat(self) -> char4 {
    return long4::to_char(simd::clamp(self, long4::broadcast(std::i8::MIN as i64), long4::broadcast(std::i8::MAX as i64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return long4::to_uchar(simd::clamp(self, long4::broadcast(std::u8::MIN as i64), long4::broadcast(std::u8::MAX as i64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return long4::to_short(simd::clamp(self, long4::broadcast(std::i16::MIN as i64), long4::broadcast(std::i16::MAX as i64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return long4::to_ushort(simd::clamp(self, long4::broadcast(std::u16::MIN as i64), long4::broadcast(std::u16::MAX as i64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return long4::to_int(simd::clamp(self, long4::broadcast(std::i32::MIN as i64), long4::broadcast(std::i32::MAX as i64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return long4::to_uint(simd::clamp(self, long4::broadcast(std::u32::MIN as i64), long4::broadcast(std::u32::MAX as i64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return self;
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return long4::to_ulong(simd::max(self, long4::broadcast(0)));
  }
}

impl simd::Dot for long4 {
  type DotProduct = i64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for long4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & std::i64::MIN != 0;
  }
}

impl simd::Reduce for long4 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return simd::reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return simd::reduce_min(simd::min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return simd::reduce_max(simd::max(self.lo(), self.hi()));
  }
}

impl simd::Select<long4> for long4 {
  #[inline(always)]
  fn select(self, a: long4, b: long4) -> long4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long4, b: long4) -> long4 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ulong4> for long4 {
  #[inline(always)]
  fn select(self, a: ulong4, b: ulong4) -> ulong4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong4, b: ulong4) -> ulong4 {
    return ulong4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl simd::Select<double4> for long4 {
  #[inline(always)]
  fn select(self, a: double4, b: double4) -> double4 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double4, b: double4) -> double4 {
    return double4::bitcast(self.bitselect(long4::bitcast(a), long4::bitcast(b)));
  }
}

impl long4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> Self {
    return long4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: long4, y: long4, z: long4) -> long4 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> long2 {
    return long2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
