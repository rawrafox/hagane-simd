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

impl std::ops::Add for char3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char3::broadcast(other)) };
  }
}

impl std::ops::Add<char3> for i8 {
  type Output = char3;

  #[inline]
  fn add(self, other: char3) -> char3 {
    return unsafe { simd_add(char3::broadcast(self), other) };
  }
}

impl std::ops::Sub for char3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char3::broadcast(other)) };
  }
}

impl std::ops::Sub<char3> for i8 {
  type Output = char3;

  #[inline]
  fn sub(self, other: char3) -> char3 {
    return unsafe { simd_sub(char3::broadcast(self), other) };
  }
}

impl std::ops::Mul for char3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char3::broadcast(other)) };
  }
}

impl std::ops::Mul<char3> for i8 {
  type Output = char3;

  #[inline]
  fn mul(self, other: char3) -> char3 {
    return unsafe { simd_mul(char3::broadcast(self), other) };
  }
}

impl std::ops::Div for char3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char3::broadcast(other)) };
  }
}

impl std::ops::Div<char3> for i8 {
  type Output = char3;

  #[inline]
  fn div(self, other: char3) -> char3 {
    return unsafe { simd_div(char3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitand(self, other: char3) -> char3 {
    return unsafe { simd_and(char3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitOr<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitor(self, other: char3) -> char3 {
    return unsafe { simd_or(char3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitXor<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitxor(self, other: char3) -> char3 {
    return unsafe { simd_xor(char3::broadcast(self), other) };
  }
}

impl std::ops::Shl<char3> for char3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char3::broadcast(other)) };
  }
}

impl std::ops::Shl<char3> for i8 {
  type Output = char3;

  #[inline]
  fn shl(self, other: char3) -> char3 {
    return unsafe { simd_shl(char3::broadcast(self), other) };
  }
}

impl std::ops::Shr<char3> for char3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char3::broadcast(other)) };
  }
}

impl std::ops::Shr<char3> for i8 {
  type Output = char3;

  #[inline]
  fn shr(self, other: char3) -> char3 {
    return unsafe { simd_shr(char3::broadcast(self), other) };
  }
}

impl std::ops::Not for char3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for char3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for char3 {
  type Scalar = i8;
  type Boolean = char3;

  type CharVector = char3;
  type ShortVector = short3;
  type IntVector = int3;
  type LongVector = long3;

  type UCharVector = uchar3;
  type UShortVector = ushort3;
  type UIntVector = uint3;
  type ULongVector = ulong3;

  type FloatVector = float3;
  type DoubleVector = double3;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;
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
  fn to_char_sat(self) -> char3 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return char3::to_uchar(simd::max(self, char3::broadcast(0)));
  }

  #[inline(always)]
  fn to_short(self) -> short3 {
    return short3(self.0 as i16, self.1 as i16, self.2 as i16);
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return char3::to_short(self);
  }

  #[inline(always)]
  fn to_ushort(self) -> ushort3 {
    return ushort3(self.0 as u16, self.1 as u16, self.2 as u16);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return char3::to_ushort(simd::max(self, char3::broadcast(0)));
  }

  #[inline(always)]
  fn to_int(self) -> int3 {
    return int3(self.0 as i32, self.1 as i32, self.2 as i32);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return char3::to_int(self);
  }

  #[inline(always)]
  fn to_uint(self) -> uint3 {
    return uint3(self.0 as u32, self.1 as u32, self.2 as u32);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return char3::to_uint(simd::max(self, char3::broadcast(0)));
  }

  #[inline(always)]
  fn to_float(self) -> float3 {
    return float3(self.0 as f32, self.1 as f32, self.2 as f32);
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return char3::to_long(self);
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return char3::to_ulong(simd::max(self, char3::broadcast(0)));
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl simd::Dot for char3 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for char3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & std::i8::MIN != 0;
  }
}

impl simd::Reduce for char3 {
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

impl simd::Select<char3> for char3 {
  #[inline(always)]
  fn select(self, a: char3, b: char3) -> char3 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char3, b: char3) -> char3 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uchar3> for char3 {
  #[inline(always)]
  fn select(self, a: uchar3, b: uchar3) -> uchar3 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar3, b: uchar3) -> uchar3 {
    return uchar3::bitcast(self.bitselect(char3::bitcast(a), char3::bitcast(b)));
  }
}

impl char3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> Self {
    return char3(x, x, x);
  }

  #[inline]
  pub fn madd(x: char3, y: char3, z: char3) -> char3 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> char2 {
    return char2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> char2 {
    return char2(self.0, self.2);
  }
}
