use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct short3(pub i16, pub i16, pub i16);

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

impl std::ops::Index<u32> for short3 {
  type Output = i16;

  #[inline]
  fn index(&self, index: u32) -> &i16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for short3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short3::broadcast(other)) };
  }
}

impl std::ops::Add<short3> for i16 {
  type Output = short3;

  #[inline]
  fn add(self, other: short3) -> short3 {
    return unsafe { simd_add(short3::broadcast(self), other) };
  }
}

impl std::ops::Sub for short3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short3::broadcast(other)) };
  }
}

impl std::ops::Sub<short3> for i16 {
  type Output = short3;

  #[inline]
  fn sub(self, other: short3) -> short3 {
    return unsafe { simd_sub(short3::broadcast(self), other) };
  }
}

impl std::ops::Mul for short3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short3::broadcast(other)) };
  }
}

impl std::ops::Mul<short3> for i16 {
  type Output = short3;

  #[inline]
  fn mul(self, other: short3) -> short3 {
    return unsafe { simd_mul(short3::broadcast(self), other) };
  }
}

impl std::ops::Div for short3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short3::broadcast(other)) };
  }
}

impl std::ops::Div<short3> for i16 {
  type Output = short3;

  #[inline]
  fn div(self, other: short3) -> short3 {
    return unsafe { simd_div(short3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitand(self, other: short3) -> short3 {
    return unsafe { simd_and(short3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitOr<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitor(self, other: short3) -> short3 {
    return unsafe { simd_or(short3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short3::broadcast(other)) };
  }
}

impl std::ops::BitXor<short3> for i16 {
  type Output = short3;

  #[inline]
  fn bitxor(self, other: short3) -> short3 {
    return unsafe { simd_xor(short3::broadcast(self), other) };
  }
}

impl std::ops::Shl<short3> for short3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short3::broadcast(other)) };
  }
}

impl std::ops::Shl<short3> for i16 {
  type Output = short3;

  #[inline]
  fn shl(self, other: short3) -> short3 {
    return unsafe { simd_shl(short3::broadcast(self), other) };
  }
}

impl std::ops::Shr<short3> for short3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short3::broadcast(other)) };
  }
}

impl std::ops::Shr<short3> for i16 {
  type Output = short3;

  #[inline]
  fn shr(self, other: short3) -> short3 {
    return unsafe { simd_shr(short3::broadcast(self), other) };
  }
}

impl std::ops::Not for short3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for short3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(short3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(short3::ne(*self, *other));
  }
}

impl simd::Vector for short3 {
}

impl simd::Logic for short3 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2) & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2) & std::i16::MIN != 0;
  }
}

impl simd::Dot for short3 {
  type Output = i16;

  #[inline]
  fn dot(self, other: short3) -> i16 {
    return short3::reduce_add(self * other);
  }
}

impl short3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> short3 {
    return short3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i16) -> short3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: short3, y: short3) -> short3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: short3, y: short3) -> short3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: short3, y: short3) -> short3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: short3, y: short3) -> short3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: short3, y: short3) -> short3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: short3, y: short3) -> short3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: short3, y: short3, z: short3) -> short3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: short3) -> short3 {
    let mask = x >> 15;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: short3, y: short3) -> short3 {
    return short3::bitselect(x, y, short3::gt(y, x));
  }

  #[inline]
  pub fn min(x: short3, y: short3) -> short3 {
    return short3::bitselect(x, y, short3::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: short3, min: short3, max: short3) -> short3 {
    return short3::min(short3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: short3) -> i16 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: short3) -> i16 {
    return std::cmp::min(short2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: short3) -> i16 {
    return std::cmp::max(short2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn bitselect(x: short3, y: short3, z: short3) -> short3 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn to_char(x: short3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: short3) -> char3 {
    return short3::to_char(short3::clamp(x, short3::broadcast(std::i8::MIN as i16), short3::broadcast(std::i8::MAX as i16)));
  }

  #[inline]
  pub fn to_uchar(x: short3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: short3) -> uchar3 {
    return short3::to_uchar(short3::clamp(x, short3::broadcast(std::u8::MIN as i16), short3::broadcast(std::u8::MAX as i16)));
  }

  #[inline]
  pub fn to_short(x: short3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: short3) -> short3 {
    return x;
  }

  #[inline]
  pub fn to_ushort(x: short3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: short3) -> ushort3 {
    return short3::to_ushort(short3::max(x, short3::broadcast(0)));
  }

  #[inline]
  pub fn to_int(x: short3) -> int3 {
    return int3(x.0 as i32, x.1 as i32, x.2 as i32);
  }

  #[inline]
  pub fn to_int_sat(x: short3) -> int3 {
    return short3::to_int(x);
  }

  #[inline]
  pub fn to_uint(x: short3) -> uint3 {
    return uint3(x.0 as u32, x.1 as u32, x.2 as u32);
  }

  #[inline]
  pub fn to_uint_sat(x: short3) -> uint3 {
    return short3::to_uint(short3::max(x, short3::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: short3) -> float3 {
    return float3(x.0 as f32, x.1 as f32, x.2 as f32);
  }

  #[inline]
  pub fn to_long(x: short3) -> long3 {
    return long3(x.0 as i64, x.1 as i64, x.2 as i64);
  }

  #[inline]
  pub fn to_long_sat(x: short3) -> long3 {
    return short3::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: short3) -> ulong3 {
    return ulong3(x.0 as u64, x.1 as u64, x.2 as u64);
  }

  #[inline]
  pub fn to_ulong_sat(x: short3) -> ulong3 {
    return short3::to_ulong(short3::max(x, short3::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: short3) -> double3 {
    return double3(x.0 as f64, x.1 as f64, x.2 as f64);
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> short2 {
    return short2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> short2 {
    return short2(self.0, self.2);
  }
}
