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

impl std::ops::Add for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int2::broadcast(other)) };
  }
}

impl std::ops::Add<int2> for i32 {
  type Output = int2;

  #[inline]
  fn add(self, other: int2) -> int2 {
    return unsafe { simd_add(int2::broadcast(self), other) };
  }
}

impl std::ops::Sub for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int2::broadcast(other)) };
  }
}

impl std::ops::Sub<int2> for i32 {
  type Output = int2;

  #[inline]
  fn sub(self, other: int2) -> int2 {
    return unsafe { simd_sub(int2::broadcast(self), other) };
  }
}

impl std::ops::Mul for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int2::broadcast(other)) };
  }
}

impl std::ops::Mul<int2> for i32 {
  type Output = int2;

  #[inline]
  fn mul(self, other: int2) -> int2 {
    return unsafe { simd_mul(int2::broadcast(self), other) };
  }
}

impl std::ops::Div for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int2::broadcast(other)) };
  }
}

impl std::ops::Div<int2> for i32 {
  type Output = int2;

  #[inline]
  fn div(self, other: int2) -> int2 {
    return unsafe { simd_div(int2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for int2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i32) -> Self {
    return unsafe { simd_and(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitand(self, other: int2) -> int2 {
    return unsafe { simd_and(int2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for int2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i32) -> Self {
    return unsafe { simd_or(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitOr<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitor(self, other: int2) -> int2 {
    return unsafe { simd_or(int2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for int2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i32) -> Self {
    return unsafe { simd_xor(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitXor<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitxor(self, other: int2) -> int2 {
    return unsafe { simd_xor(int2::broadcast(self), other) };
  }
}

impl std::ops::Shl<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i32) -> Self {
    return unsafe { simd_shl(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shl<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shl(self, other: int2) -> int2 {
    return unsafe { simd_shl(int2::broadcast(self), other) };
  }
}

impl std::ops::Shr<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i32) -> Self {
    return unsafe { simd_shr(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shr<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shr(self, other: int2) -> int2 {
    return unsafe { simd_shr(int2::broadcast(self), other) };
  }
}

impl std::ops::Not for int2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for int2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for int2 {
  type Scalar = i32;
  type Boolean = int2;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;
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

impl simd::Dot for int2 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for int2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & std::i32::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & std::i32::MIN != 0;
  }
}

impl simd::Reduce for int2 {
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

impl simd::Select<int2> for int2 {
  #[inline(always)]
  fn select(self, a: int2, b: int2) -> int2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int2, b: int2) -> int2 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uint2> for int2 {
  #[inline(always)]
  fn select(self, a: uint2, b: uint2) -> uint2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint2, b: uint2) -> uint2 {
    return uint2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl simd::Select<float2> for int2 {
  #[inline(always)]
  fn select(self, a: float2, b: float2) -> float2 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float2, b: float2) -> float2 {
    return float2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl int2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> Self {
    return int2(x, x);
  }

  #[inline]
  pub fn madd(x: int2, y: int2, z: int2) -> int2 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: int2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: int2) -> char2 {
    return int2::to_char(simd::clamp(x, int2::broadcast(std::i8::MIN as i32), int2::broadcast(std::i8::MAX as i32)));
  }

  #[inline]
  pub fn to_uchar(x: int2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: int2) -> uchar2 {
    return int2::to_uchar(simd::clamp(x, int2::broadcast(std::u8::MIN as i32), int2::broadcast(std::u8::MAX as i32)));
  }

  #[inline]
  pub fn to_short(x: int2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: int2) -> short2 {
    return int2::to_short(simd::clamp(x, int2::broadcast(std::i16::MIN as i32), int2::broadcast(std::i16::MAX as i32)));
  }

  #[inline]
  pub fn to_ushort(x: int2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: int2) -> ushort2 {
    return int2::to_ushort(simd::clamp(x, int2::broadcast(std::u16::MIN as i32), int2::broadcast(std::u16::MAX as i32)));
  }

  #[inline]
  pub fn to_int(x: int2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: int2) -> int2 {
    return x;
  }

  #[inline]
  pub fn to_uint(x: int2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: int2) -> uint2 {
    return int2::to_uint(simd::max(x, int2::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: int2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: int2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: int2) -> long2 {
    return int2::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: int2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: int2) -> ulong2 {
    return int2::to_ulong(simd::max(x, int2::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: int2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> i32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i32 {
    return self.0;
  }
}
