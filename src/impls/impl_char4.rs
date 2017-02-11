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

impl std::ops::Add for char4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char4::broadcast(other)) };
  }
}

impl std::ops::Add<char4> for i8 {
  type Output = char4;

  #[inline]
  fn add(self, other: char4) -> char4 {
    return unsafe { simd_add(char4::broadcast(self), other) };
  }
}

impl std::ops::Sub for char4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char4::broadcast(other)) };
  }
}

impl std::ops::Sub<char4> for i8 {
  type Output = char4;

  #[inline]
  fn sub(self, other: char4) -> char4 {
    return unsafe { simd_sub(char4::broadcast(self), other) };
  }
}

impl std::ops::Mul for char4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char4::broadcast(other)) };
  }
}

impl std::ops::Mul<char4> for i8 {
  type Output = char4;

  #[inline]
  fn mul(self, other: char4) -> char4 {
    return unsafe { simd_mul(char4::broadcast(self), other) };
  }
}

impl std::ops::Div for char4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char4::broadcast(other)) };
  }
}

impl std::ops::Div<char4> for i8 {
  type Output = char4;

  #[inline]
  fn div(self, other: char4) -> char4 {
    return unsafe { simd_div(char4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitand(self, other: char4) -> char4 {
    return unsafe { simd_and(char4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitOr<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitor(self, other: char4) -> char4 {
    return unsafe { simd_or(char4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitXor<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitxor(self, other: char4) -> char4 {
    return unsafe { simd_xor(char4::broadcast(self), other) };
  }
}

impl std::ops::Shl<char4> for char4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char4::broadcast(other)) };
  }
}

impl std::ops::Shl<char4> for i8 {
  type Output = char4;

  #[inline]
  fn shl(self, other: char4) -> char4 {
    return unsafe { simd_shl(char4::broadcast(self), other) };
  }
}

impl std::ops::Shr<char4> for char4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char4::broadcast(other)) };
  }
}

impl std::ops::Shr<char4> for i8 {
  type Output = char4;

  #[inline]
  fn shr(self, other: char4) -> char4 {
    return unsafe { simd_shr(char4::broadcast(self), other) };
  }
}

impl std::ops::Not for char4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for char4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for char4 {
  type Scalar = i8;
  type Boolean = char4;

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
}

impl simd::Dot for char4 {
  type DotProduct = i8;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for char4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & std::i8::MIN != 0;
  }
}

impl simd::Reduce for char4 {
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

impl simd::Select<char4> for char4 {
  #[inline(always)]
  fn select(self, a: char4, b: char4) -> char4 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char4, b: char4) -> char4 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uchar4> for char4 {
  #[inline(always)]
  fn select(self, a: uchar4, b: uchar4) -> uchar4 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar4, b: uchar4) -> uchar4 {
    return uchar4::bitcast(self.bitselect(char4::bitcast(a), char4::bitcast(b)));
  }
}

impl char4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> Self {
    return char4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: char4, y: char4, z: char4) -> char4 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: char4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: char4) -> char4 {
    return x;
  }

  #[inline]
  pub fn to_uchar(x: char4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: char4) -> uchar4 {
    return char4::to_uchar(simd::max(x, char4::broadcast(0)));
  }

  #[inline]
  pub fn to_short(x: char4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: char4) -> short4 {
    return char4::to_short(x);
  }

  #[inline]
  pub fn to_ushort(x: char4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: char4) -> ushort4 {
    return char4::to_ushort(simd::max(x, char4::broadcast(0)));
  }

  #[inline]
  pub fn to_int(x: char4) -> int4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: char4) -> int4 {
    return char4::to_int(x);
  }

  #[inline]
  pub fn to_uint(x: char4) -> uint4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: char4) -> uint4 {
    return char4::to_uint(simd::max(x, char4::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: char4) -> float4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: char4) -> long4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: char4) -> long4 {
    return char4::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: char4) -> ulong4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: char4) -> ulong4 {
    return char4::to_ulong(simd::max(x, char4::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: char4) -> double4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> char2 {
    return char2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> char2 {
    return char2(self.0, self.2);
  }
}
