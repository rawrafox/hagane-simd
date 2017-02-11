use std;
use ::*;

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

impl std::ops::Add for long3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long3::broadcast(other)) };
  }
}

impl std::ops::Add<long3> for i64 {
  type Output = long3;

  #[inline]
  fn add(self, other: long3) -> long3 {
    return unsafe { simd_add(long3::broadcast(self), other) };
  }
}

impl std::ops::Sub for long3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long3::broadcast(other)) };
  }
}

impl std::ops::Sub<long3> for i64 {
  type Output = long3;

  #[inline]
  fn sub(self, other: long3) -> long3 {
    return unsafe { simd_sub(long3::broadcast(self), other) };
  }
}

impl std::ops::Mul for long3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long3::broadcast(other)) };
  }
}

impl std::ops::Mul<long3> for i64 {
  type Output = long3;

  #[inline]
  fn mul(self, other: long3) -> long3 {
    return unsafe { simd_mul(long3::broadcast(self), other) };
  }
}

impl std::ops::Div for long3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long3::broadcast(other)) };
  }
}

impl std::ops::Div<long3> for i64 {
  type Output = long3;

  #[inline]
  fn div(self, other: long3) -> long3 {
    return unsafe { simd_div(long3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitand(self, other: long3) -> long3 {
    return unsafe { simd_and(long3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitOr<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitor(self, other: long3) -> long3 {
    return unsafe { simd_or(long3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long3::broadcast(other)) };
  }
}

impl std::ops::BitXor<long3> for i64 {
  type Output = long3;

  #[inline]
  fn bitxor(self, other: long3) -> long3 {
    return unsafe { simd_xor(long3::broadcast(self), other) };
  }
}

impl std::ops::Shl<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shl<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shl(self, other: long3) -> long3 {
    return unsafe { simd_shl(long3::broadcast(self), other) };
  }
}

impl std::ops::Shr<long3> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long3::broadcast(other)) };
  }
}

impl std::ops::Shr<long3> for i64 {
  type Output = long3;

  #[inline]
  fn shr(self, other: long3) -> long3 {
    return unsafe { simd_shr(long3::broadcast(self), other) };
  }
}

impl std::ops::Not for long3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for long3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for long3 {
  type Scalar = i64;
  type Boolean = long3;

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
}

impl simd::Dot for long3 {
  type DotProduct = i64;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for long3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & std::i64::MIN != 0;
  }
}

impl simd::Reduce for long3 {
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

impl simd::Select<long3> for long3 {
  #[inline(always)]
  fn select(self, a: long3, b: long3) -> long3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long3, b: long3) -> long3 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ulong3> for long3 {
  #[inline(always)]
  fn select(self, a: ulong3, b: ulong3) -> ulong3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong3, b: ulong3) -> ulong3 {
    return ulong3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl simd::Select<double3> for long3 {
  #[inline(always)]
  fn select(self, a: double3, b: double3) -> double3 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double3, b: double3) -> double3 {
    return double3::bitcast(self.bitselect(long3::bitcast(a), long3::bitcast(b)));
  }
}

impl long3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> Self {
    return long3(x, x, x);
  }

  #[inline]
  pub fn madd(x: long3, y: long3, z: long3) -> long3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: long3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: long3) -> char3 {
    return long3::to_char(simd::clamp(x, long3::broadcast(std::i8::MIN as i64), long3::broadcast(std::i8::MAX as i64)));
  }

  #[inline]
  pub fn to_uchar(x: long3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: long3) -> uchar3 {
    return long3::to_uchar(simd::clamp(x, long3::broadcast(std::u8::MIN as i64), long3::broadcast(std::u8::MAX as i64)));
  }

  #[inline]
  pub fn to_short(x: long3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: long3) -> short3 {
    return long3::to_short(simd::clamp(x, long3::broadcast(std::i16::MIN as i64), long3::broadcast(std::i16::MAX as i64)));
  }

  #[inline]
  pub fn to_ushort(x: long3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: long3) -> ushort3 {
    return long3::to_ushort(simd::clamp(x, long3::broadcast(std::u16::MIN as i64), long3::broadcast(std::u16::MAX as i64)));
  }

  #[inline]
  pub fn to_int(x: long3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: long3) -> int3 {
    return long3::to_int(simd::clamp(x, long3::broadcast(std::i32::MIN as i64), long3::broadcast(std::i32::MAX as i64)));
  }

  #[inline]
  pub fn to_uint(x: long3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: long3) -> uint3 {
    return long3::to_uint(simd::clamp(x, long3::broadcast(std::u32::MIN as i64), long3::broadcast(std::u32::MAX as i64)));
  }

  #[inline]
  pub fn to_float(x: long3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: long3) -> long3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: long3) -> long3 {
    return x;
  }

  #[inline]
  pub fn to_ulong(x: long3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: long3) -> ulong3 {
    return long3::to_ulong(simd::max(x, long3::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: long3) -> double3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> long2 {
    return long2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> long2 {
    return long2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> long2 {
    return long2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> long2 {
    return long2(self.0, self.2);
  }
}
