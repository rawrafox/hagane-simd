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

impl std::ops::Add for uchar4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u8) -> Self {
    return unsafe { simd_add(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Add<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn add(self, other: uchar4) -> uchar4 {
    return unsafe { simd_add(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Sub for uchar4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u8) -> Self {
    return unsafe { simd_sub(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Sub<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn sub(self, other: uchar4) -> uchar4 {
    return unsafe { simd_sub(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Mul for uchar4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u8) -> Self {
    return unsafe { simd_mul(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Mul<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn mul(self, other: uchar4) -> uchar4 {
    return unsafe { simd_mul(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Div for uchar4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u8) -> Self {
    return unsafe { simd_div(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Div<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn div(self, other: uchar4) -> uchar4 {
    return unsafe { simd_div(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uchar4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u8) -> Self {
    return unsafe { simd_and(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitand(self, other: uchar4) -> uchar4 {
    return unsafe { simd_and(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uchar4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u8) -> Self {
    return unsafe { simd_or(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitOr<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitor(self, other: uchar4) -> uchar4 {
    return unsafe { simd_or(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uchar4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u8) -> Self {
    return unsafe { simd_xor(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitXor<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitxor(self, other: uchar4) -> uchar4 {
    return unsafe { simd_xor(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Shl<uchar4> for uchar4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u8) -> Self {
    return unsafe { simd_shl(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Shl<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn shl(self, other: uchar4) -> uchar4 {
    return unsafe { simd_shl(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Shr<uchar4> for uchar4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u8) -> Self {
    return unsafe { simd_shr(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Shr<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn shr(self, other: uchar4) -> uchar4 {
    return unsafe { simd_shr(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Not for uchar4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u8::MAX;
  }
}

impl PartialEq for uchar4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for uchar4 {
  type Scalar = u8;
  type Boolean = char4;

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
  fn to_char_sat(self) -> char4 {
    return uchar4::to_char(simd::min(self, uchar4::broadcast(std::i8::MAX as u8)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return self;
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return uchar4::to_short(simd::min(self, uchar4::broadcast(std::i16::MAX as u8)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return uchar4::to_ushort(self);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return uchar4::to_int(simd::min(self, uchar4::broadcast(std::i32::MAX as u8)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return uchar4::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return uchar4::to_long(simd::min(self, uchar4::broadcast(std::i64::MAX as u8)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return uchar4::to_ulong(self);
  }
}

impl simd::Dot for uchar4 {
  type DotProduct = u8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for uchar4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & 0x80 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & 0x80 != 0;
  }
}

impl simd::Reduce for uchar4 {
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

impl uchar4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> Self {
    return uchar4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: uchar4, y: uchar4, z: uchar4) -> uchar4 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> uchar2 {
    return uchar2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uchar2 {
    return uchar2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> uchar2 {
    return uchar2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> uchar2 {
    return uchar2(self.0, self.2);
  }
}
