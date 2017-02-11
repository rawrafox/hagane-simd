use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ulong3(pub u64, pub u64, pub u64);

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

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_cast<T, U>(x: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for ulong3 {
  type Output = u64;

  #[inline]
  fn index(&self, index: u32) -> &u64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ulong3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Add<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn add(self, other: ulong3) -> ulong3 {
    return unsafe { simd_add(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn sub(self, other: ulong3) -> ulong3 {
    return unsafe { simd_sub(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn mul(self, other: ulong3) -> ulong3 {
    return unsafe { simd_mul(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Div<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn div(self, other: ulong3) -> ulong3 {
    return unsafe { simd_div(ulong3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn bitand(self, other: ulong3) -> ulong3 {
    return unsafe { simd_and(ulong3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn bitor(self, other: ulong3) -> ulong3 {
    return unsafe { simd_or(ulong3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn bitxor(self, other: ulong3) -> ulong3 {
    return unsafe { simd_xor(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong3> for ulong3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn shl(self, other: ulong3) -> ulong3 {
    return unsafe { simd_shl(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong3> for ulong3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong3::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong3> for u64 {
  type Output = ulong3;

  #[inline]
  fn shr(self, other: ulong3) -> ulong3 {
    return unsafe { simd_shr(ulong3::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(ulong3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(ulong3::ne(*self, *other));
  }
}

impl simd::Vector for ulong3 {
}

impl simd::Logic for ulong3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & 0x8000000000000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & 0x8000000000000000 != 0;
  }
}

impl simd::Dot for ulong3 {
  type Output = u64;

  #[inline]
  fn dot(self, other: ulong3) -> u64 {
    return ulong3::reduce_add(self * other);
  }
}

impl ulong3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> ulong3 {
    return ulong3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u64) -> ulong3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ulong3, y: ulong3) -> long3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: ulong3, y: ulong3, z: ulong3) -> ulong3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: ulong3) -> ulong3 {
    return x;
  }

  #[inline]
  pub fn max(x: ulong3, y: ulong3) -> ulong3 {
    return ulong3::bitselect(x, y, ulong3::gt(y, x));
  }

  #[inline]
  pub fn min(x: ulong3, y: ulong3) -> ulong3 {
    return ulong3::bitselect(x, y, ulong3::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ulong3, min: ulong3, max: ulong3) -> ulong3 {
    return ulong3::min(ulong3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ulong3) -> u64 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: ulong3) -> u64 {
    return std::cmp::min(ulong2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: ulong3) -> u64 {
    return std::cmp::max(ulong2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn bitselect(x: ulong3, y: ulong3, z: long3) -> ulong3 {
    return ulong3::bitcast(long3::bitselect(long3::bitcast(x), long3::bitcast(y), z));
  }

  #[inline]
  pub fn to_char(x: ulong3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: ulong3) -> char3 {
    return ulong3::to_char(ulong3::min(x, ulong3::broadcast(std::i8::MAX as u64)));
  }

  #[inline]
  pub fn to_uchar(x: ulong3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: ulong3) -> uchar3 {
    return ulong3::to_uchar(ulong3::min(x, ulong3::broadcast(std::u8::MAX as u64)));
  }

  #[inline]
  pub fn to_short(x: ulong3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: ulong3) -> short3 {
    return ulong3::to_short(ulong3::min(x, ulong3::broadcast(std::i16::MAX as u64)));
  }

  #[inline]
  pub fn to_ushort(x: ulong3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: ulong3) -> ushort3 {
    return ulong3::to_ushort(ulong3::min(x, ulong3::broadcast(std::u16::MAX as u64)));
  }

  #[inline]
  pub fn to_int(x: ulong3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: ulong3) -> int3 {
    return ulong3::to_int(ulong3::min(x, ulong3::broadcast(std::i32::MAX as u64)));
  }

  #[inline]
  pub fn to_uint(x: ulong3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: ulong3) -> uint3 {
    return ulong3::to_uint(ulong3::min(x, ulong3::broadcast(std::u32::MAX as u64)));
  }

  #[inline]
  pub fn to_float(x: ulong3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: ulong3) -> long3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: ulong3) -> long3 {
    return ulong3::to_long(ulong3::min(x, ulong3::broadcast(std::i64::MAX as u64)));
  }

  #[inline]
  pub fn to_ulong(x: ulong3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: ulong3) -> ulong3 {
    return x;
  }

  #[inline]
  pub fn to_double(x: ulong3) -> double3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> ulong2 {
    return ulong2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ulong2 {
    return ulong2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> ulong2 {
    return ulong2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> ulong2 {
    return ulong2(self.0, self.2);
  }
}
