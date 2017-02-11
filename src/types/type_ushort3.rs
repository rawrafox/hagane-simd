use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ushort3(pub u16, pub u16, pub u16);

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

  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for ushort3 {
  type Output = u16;

  #[inline]
  fn index(&self, index: u32) -> &u16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ushort3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn add(self, other: u16) -> Self {
    return unsafe { simd_add(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Add<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn add(self, other: ushort3) -> ushort3 {
    return unsafe { simd_add(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Sub for ushort3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u16) -> Self {
    return unsafe { simd_sub(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Sub<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn sub(self, other: ushort3) -> ushort3 {
    return unsafe { simd_sub(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Mul for ushort3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u16) -> Self {
    return unsafe { simd_mul(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Mul<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn mul(self, other: ushort3) -> ushort3 {
    return unsafe { simd_mul(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Div for ushort3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn div(self, other: u16) -> Self {
    return unsafe { simd_div(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Div<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn div(self, other: ushort3) -> ushort3 {
    return unsafe { simd_div(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ushort3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u16) -> Self {
    return unsafe { simd_and(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitand(self, other: ushort3) -> ushort3 {
    return unsafe { simd_and(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ushort3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u16) -> Self {
    return unsafe { simd_or(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitOr<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitor(self, other: ushort3) -> ushort3 {
    return unsafe { simd_or(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ushort3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u16) -> Self {
    return unsafe { simd_xor(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitXor<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitxor(self, other: ushort3) -> ushort3 {
    return unsafe { simd_xor(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Shl<ushort3> for ushort3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn shl(self, other: ushort3) -> ushort3 {
    return unsafe { simd_shl(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort3> for ushort3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn shr(self, other: ushort3) -> ushort3 {
    return unsafe { simd_shr(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for ushort3 {
  type Scalar = u16;
  type Boolean = short3;

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

impl simd::Dot for ushort3 {
  type Output = u16;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for ushort3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & 0x8000 != 0;
  }
}

impl simd::Reduce for ushort3 {
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

impl ushort3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort3(x, x, x);
  }

  #[inline]
  pub fn madd(x: ushort3, y: ushort3, z: ushort3) -> ushort3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: ushort3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: ushort3) -> char3 {
    return ushort3::to_char(simd::min(x, ushort3::broadcast(std::i8::MAX as u16)));
  }

  #[inline]
  pub fn to_uchar(x: ushort3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: ushort3) -> uchar3 {
    return ushort3::to_uchar(simd::min(x, ushort3::broadcast(std::u8::MAX as u16)));
  }

  #[inline]
  pub fn to_short(x: ushort3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: ushort3) -> short3 {
    return ushort3::to_short(simd::min(x, ushort3::broadcast(std::i16::MAX as u16)));
  }

  #[inline]
  pub fn to_ushort(x: ushort3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: ushort3) -> ushort3 {
    return x;
  }

  #[inline]
  pub fn to_int(x: ushort3) -> int3 {
    return int3(x.0 as i32, x.1 as i32, x.2 as i32);
  }

  #[inline]
  pub fn to_int_sat(x: ushort3) -> int3 {
    return ushort3::to_int(simd::min(x, ushort3::broadcast(std::i32::MAX as u16)));
  }

  #[inline]
  pub fn to_uint(x: ushort3) -> uint3 {
    return uint3(x.0 as u32, x.1 as u32, x.2 as u32);
  }

  #[inline]
  pub fn to_uint_sat(x: ushort3) -> uint3 {
    return ushort3::to_uint(x);
  }

  #[inline]
  pub fn to_float(x: ushort3) -> float3 {
    return float3(x.0 as f32, x.1 as f32, x.2 as f32);
  }

  #[inline]
  pub fn to_long(x: ushort3) -> long3 {
    return long3(x.0 as i64, x.1 as i64, x.2 as i64);
  }

  #[inline]
  pub fn to_long_sat(x: ushort3) -> long3 {
    return ushort3::to_long(simd::min(x, ushort3::broadcast(std::i64::MAX as u16)));
  }

  #[inline]
  pub fn to_ulong(x: ushort3) -> ulong3 {
    return ulong3(x.0 as u64, x.1 as u64, x.2 as u64);
  }

  #[inline]
  pub fn to_ulong_sat(x: ushort3) -> ulong3 {
    return ushort3::to_ulong(x);
  }

  #[inline]
  pub fn to_double(x: ushort3) -> double3 {
    return double3(x.0 as f64, x.1 as f64, x.2 as f64);
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}
