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

impl std::ops::Add for ushort4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u16) -> Self {
    return unsafe { simd_add(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Add<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn add(self, other: ushort4) -> ushort4 {
    return unsafe { simd_add(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Sub for ushort4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u16) -> Self {
    return unsafe { simd_sub(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Sub<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn sub(self, other: ushort4) -> ushort4 {
    return unsafe { simd_sub(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Mul for ushort4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u16) -> Self {
    return unsafe { simd_mul(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Mul<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn mul(self, other: ushort4) -> ushort4 {
    return unsafe { simd_mul(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Div for ushort4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u16) -> Self {
    return unsafe { simd_div(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Div<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn div(self, other: ushort4) -> ushort4 {
    return unsafe { simd_div(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ushort4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u16) -> Self {
    return unsafe { simd_and(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitand(self, other: ushort4) -> ushort4 {
    return unsafe { simd_and(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ushort4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u16) -> Self {
    return unsafe { simd_or(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitOr<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitor(self, other: ushort4) -> ushort4 {
    return unsafe { simd_or(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ushort4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u16) -> Self {
    return unsafe { simd_xor(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitXor<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitxor(self, other: ushort4) -> ushort4 {
    return unsafe { simd_xor(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Shl<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shl(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shl(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shr(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shr(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for ushort4 {
  type Scalar = u16;
  type Boolean = short4;

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
    return ushort4::to_char(simd::min(self, ushort4::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return ushort4::to_uchar(simd::min(self, ushort4::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return ushort4::to_short(simd::min(self, ushort4::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return ushort4::to_int(simd::min(self, ushort4::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return ushort4::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return ushort4::to_long(simd::min(self, ushort4::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return ushort4::to_ulong(self);
  }
}

impl simd::Dot for ushort4 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for ushort4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & 0x8000 != 0;
  }
}

impl simd::Reduce for ushort4 {
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

impl ushort4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: ushort4, y: ushort4, z: ushort4) -> ushort4 {
    return x * y + z;
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}
