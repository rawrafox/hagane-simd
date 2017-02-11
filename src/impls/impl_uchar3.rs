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

impl std::ops::Add for uchar3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn add(self, other: u8) -> Self {
    return unsafe { simd_add(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Add<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn add(self, other: uchar3) -> uchar3 {
    return unsafe { simd_add(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Sub for uchar3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u8) -> Self {
    return unsafe { simd_sub(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Sub<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn sub(self, other: uchar3) -> uchar3 {
    return unsafe { simd_sub(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Mul for uchar3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u8) -> Self {
    return unsafe { simd_mul(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Mul<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn mul(self, other: uchar3) -> uchar3 {
    return unsafe { simd_mul(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Div for uchar3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn div(self, other: u8) -> Self {
    return unsafe { simd_div(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Div<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn div(self, other: uchar3) -> uchar3 {
    return unsafe { simd_div(uchar3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uchar3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u8) -> Self {
    return unsafe { simd_and(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn bitand(self, other: uchar3) -> uchar3 {
    return unsafe { simd_and(uchar3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uchar3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u8) -> Self {
    return unsafe { simd_or(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::BitOr<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn bitor(self, other: uchar3) -> uchar3 {
    return unsafe { simd_or(uchar3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uchar3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u8) -> Self {
    return unsafe { simd_xor(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::BitXor<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn bitxor(self, other: uchar3) -> uchar3 {
    return unsafe { simd_xor(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Shl<uchar3> for uchar3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u8) -> Self {
    return unsafe { simd_shl(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Shl<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn shl(self, other: uchar3) -> uchar3 {
    return unsafe { simd_shl(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Shr<uchar3> for uchar3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u8> for uchar3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u8) -> Self {
    return unsafe { simd_shr(self, uchar3::broadcast(other)) };
  }
}

impl std::ops::Shr<uchar3> for u8 {
  type Output = uchar3;

  #[inline]
  fn shr(self, other: uchar3) -> uchar3 {
    return unsafe { simd_shr(uchar3::broadcast(self), other) };
  }
}

impl std::ops::Not for uchar3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u8::MAX;
  }
}

impl PartialEq for uchar3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for uchar3 {
  type Scalar = u8;
  type Boolean = char3;

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
}

impl simd::Dot for uchar3 {
  type DotProduct = u8;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for uchar3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & 0x80 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & 0x80 != 0;
  }
}

impl simd::Reduce for uchar3 {
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

impl uchar3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> Self {
    return uchar3(x, x, x);
  }

  #[inline]
  pub fn madd(x: uchar3, y: uchar3, z: uchar3) -> uchar3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: uchar3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: uchar3) -> char3 {
    return uchar3::to_char(simd::min(x, uchar3::broadcast(std::i8::MAX as u8)));
  }

  #[inline]
  pub fn to_uchar(x: uchar3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: uchar3) -> uchar3 {
    return x;
  }

  #[inline]
  pub fn to_short(x: uchar3) -> short3 {
    return short3(x.0 as i16, x.1 as i16, x.2 as i16);
  }

  #[inline]
  pub fn to_short_sat(x: uchar3) -> short3 {
    return uchar3::to_short(simd::min(x, uchar3::broadcast(std::i16::MAX as u8)));
  }

  #[inline]
  pub fn to_ushort(x: uchar3) -> ushort3 {
    return ushort3(x.0 as u16, x.1 as u16, x.2 as u16);
  }

  #[inline]
  pub fn to_ushort_sat(x: uchar3) -> ushort3 {
    return uchar3::to_ushort(x);
  }

  #[inline]
  pub fn to_int(x: uchar3) -> int3 {
    return int3(x.0 as i32, x.1 as i32, x.2 as i32);
  }

  #[inline]
  pub fn to_int_sat(x: uchar3) -> int3 {
    return uchar3::to_int(simd::min(x, uchar3::broadcast(std::i32::MAX as u8)));
  }

  #[inline]
  pub fn to_uint(x: uchar3) -> uint3 {
    return uint3(x.0 as u32, x.1 as u32, x.2 as u32);
  }

  #[inline]
  pub fn to_uint_sat(x: uchar3) -> uint3 {
    return uchar3::to_uint(x);
  }

  #[inline]
  pub fn to_float(x: uchar3) -> float3 {
    return float3(x.0 as f32, x.1 as f32, x.2 as f32);
  }

  #[inline]
  pub fn to_long(x: uchar3) -> long3 {
    return long3(x.0 as i64, x.1 as i64, x.2 as i64);
  }

  #[inline]
  pub fn to_long_sat(x: uchar3) -> long3 {
    return uchar3::to_long(simd::min(x, uchar3::broadcast(std::i64::MAX as u8)));
  }

  #[inline]
  pub fn to_ulong(x: uchar3) -> ulong3 {
    return ulong3(x.0 as u64, x.1 as u64, x.2 as u64);
  }

  #[inline]
  pub fn to_ulong_sat(x: uchar3) -> ulong3 {
    return uchar3::to_ulong(x);
  }

  #[inline]
  pub fn to_double(x: uchar3) -> double3 {
    return double3(x.0 as f64, x.1 as f64, x.2 as f64);
  }

  #[inline]
  pub fn lo(self) -> uchar2 {
    return uchar2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uchar2 {
    return uchar2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> uchar2 {
    return uchar2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> uchar2 {
    return uchar2(self.0, self.2);
  }
}
