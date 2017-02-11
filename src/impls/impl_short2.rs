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

impl std::ops::Add for short2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short2::broadcast(other)) };
  }
}

impl std::ops::Add<short2> for i16 {
  type Output = short2;

  #[inline]
  fn add(self, other: short2) -> short2 {
    return unsafe { simd_add(short2::broadcast(self), other) };
  }
}

impl std::ops::Sub for short2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short2::broadcast(other)) };
  }
}

impl std::ops::Sub<short2> for i16 {
  type Output = short2;

  #[inline]
  fn sub(self, other: short2) -> short2 {
    return unsafe { simd_sub(short2::broadcast(self), other) };
  }
}

impl std::ops::Mul for short2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short2::broadcast(other)) };
  }
}

impl std::ops::Mul<short2> for i16 {
  type Output = short2;

  #[inline]
  fn mul(self, other: short2) -> short2 {
    return unsafe { simd_mul(short2::broadcast(self), other) };
  }
}

impl std::ops::Div for short2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short2::broadcast(other)) };
  }
}

impl std::ops::Div<short2> for i16 {
  type Output = short2;

  #[inline]
  fn div(self, other: short2) -> short2 {
    return unsafe { simd_div(short2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitand(self, other: short2) -> short2 {
    return unsafe { simd_and(short2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitOr<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitor(self, other: short2) -> short2 {
    return unsafe { simd_or(short2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short2::broadcast(other)) };
  }
}

impl std::ops::BitXor<short2> for i16 {
  type Output = short2;

  #[inline]
  fn bitxor(self, other: short2) -> short2 {
    return unsafe { simd_xor(short2::broadcast(self), other) };
  }
}

impl std::ops::Shl<short2> for short2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short2::broadcast(other)) };
  }
}

impl std::ops::Shl<short2> for i16 {
  type Output = short2;

  #[inline]
  fn shl(self, other: short2) -> short2 {
    return unsafe { simd_shl(short2::broadcast(self), other) };
  }
}

impl std::ops::Shr<short2> for short2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short2::broadcast(other)) };
  }
}

impl std::ops::Shr<short2> for i16 {
  type Output = short2;

  #[inline]
  fn shr(self, other: short2) -> short2 {
    return unsafe { simd_shr(short2::broadcast(self), other) };
  }
}

impl std::ops::Not for short2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for short2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for short2 {
  type Scalar = i16;
  type Boolean = short2;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;
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
}

impl simd::Dot for short2 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for short2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & std::i16::MIN != 0;
  }
}

impl simd::Reduce for short2 {
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

impl simd::Select<short2> for short2 {
  #[inline(always)]
  fn select(self, a: short2, b: short2) -> short2 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short2, b: short2) -> short2 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ushort2> for short2 {
  #[inline(always)]
  fn select(self, a: ushort2, b: ushort2) -> ushort2 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort2, b: ushort2) -> ushort2 {
    return ushort2::bitcast(self.bitselect(short2::bitcast(a), short2::bitcast(b)));
  }
}

impl short2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> Self {
    return short2(x, x);
  }

  #[inline]
  pub fn madd(x: short2, y: short2, z: short2) -> short2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: short2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: short2) -> char2 {
    return short2::to_char(simd::clamp(x, short2::broadcast(std::i8::MIN as i16), short2::broadcast(std::i8::MAX as i16)));
  }

  #[inline]
  pub fn to_uchar(x: short2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: short2) -> uchar2 {
    return short2::to_uchar(simd::clamp(x, short2::broadcast(std::u8::MIN as i16), short2::broadcast(std::u8::MAX as i16)));
  }

  #[inline]
  pub fn to_short(x: short2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: short2) -> short2 {
    return x;
  }

  #[inline]
  pub fn to_ushort(x: short2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: short2) -> ushort2 {
    return short2::to_ushort(simd::max(x, short2::broadcast(0)));
  }

  #[inline]
  pub fn to_int(x: short2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: short2) -> int2 {
    return short2::to_int(x);
  }

  #[inline]
  pub fn to_uint(x: short2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: short2) -> uint2 {
    return short2::to_uint(simd::max(x, short2::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: short2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: short2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: short2) -> long2 {
    return short2::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: short2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: short2) -> ulong2 {
    return short2::to_ulong(simd::max(x, short2::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: short2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> i16 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i16 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i16 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i16 {
    return self.0;
  }
}
