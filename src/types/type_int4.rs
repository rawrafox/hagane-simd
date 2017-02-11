use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int4(pub i32, pub i32, pub i32, pub i32);

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

impl std::ops::Index<u32> for int4 {
  type Output = i32;

  #[inline]
  fn index(&self, index: u32) -> &i32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for int4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int4::broadcast(other)) };
  }
}

impl std::ops::Add<int4> for i32 {
  type Output = int4;

  #[inline]
  fn add(self, other: int4) -> int4 {
    return unsafe { simd_add(int4::broadcast(self), other) };
  }
}

impl std::ops::Sub for int4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int4::broadcast(other)) };
  }
}

impl std::ops::Sub<int4> for i32 {
  type Output = int4;

  #[inline]
  fn sub(self, other: int4) -> int4 {
    return unsafe { simd_sub(int4::broadcast(self), other) };
  }
}

impl std::ops::Mul for int4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int4::broadcast(other)) };
  }
}

impl std::ops::Mul<int4> for i32 {
  type Output = int4;

  #[inline]
  fn mul(self, other: int4) -> int4 {
    return unsafe { simd_mul(int4::broadcast(self), other) };
  }
}

impl std::ops::Div for int4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int4::broadcast(other)) };
  }
}

impl std::ops::Div<int4> for i32 {
  type Output = int4;

  #[inline]
  fn div(self, other: int4) -> int4 {
    return unsafe { simd_div(int4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for int4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i32> for int4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i32) -> Self {
    return unsafe { simd_and(self, int4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<int4> for i32 {
  type Output = int4;

  #[inline]
  fn bitand(self, other: int4) -> int4 {
    return unsafe { simd_and(int4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for int4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i32> for int4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i32) -> Self {
    return unsafe { simd_or(self, int4::broadcast(other)) };
  }
}

impl std::ops::BitOr<int4> for i32 {
  type Output = int4;

  #[inline]
  fn bitor(self, other: int4) -> int4 {
    return unsafe { simd_or(int4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for int4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i32> for int4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i32) -> Self {
    return unsafe { simd_xor(self, int4::broadcast(other)) };
  }
}

impl std::ops::BitXor<int4> for i32 {
  type Output = int4;

  #[inline]
  fn bitxor(self, other: int4) -> int4 {
    return unsafe { simd_xor(int4::broadcast(self), other) };
  }
}

impl std::ops::Shl<int4> for int4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i32> for int4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i32) -> Self {
    return unsafe { simd_shl(self, int4::broadcast(other)) };
  }
}

impl std::ops::Shl<int4> for i32 {
  type Output = int4;

  #[inline]
  fn shl(self, other: int4) -> int4 {
    return unsafe { simd_shl(int4::broadcast(self), other) };
  }
}

impl std::ops::Shr<int4> for int4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i32> for int4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i32) -> Self {
    return unsafe { simd_shr(self, int4::broadcast(other)) };
  }
}

impl std::ops::Shr<int4> for i32 {
  type Output = int4;

  #[inline]
  fn shr(self, other: int4) -> int4 {
    return unsafe { simd_shr(int4::broadcast(self), other) };
  }
}

impl std::ops::Not for int4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for int4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for int4 {
  type Scalar = i32;
  type Boolean = int4;

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

impl simd::Dot for int4 {
  type Output = i32;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::Output {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for int4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & std::i32::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & std::i32::MIN != 0;
  }
}

impl simd::Reduce for int4 {
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

impl simd::Select<int4> for int4 {
  #[inline(always)]
  fn select(self, a: int4, b: int4) -> int4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: int4, b: int4) -> int4 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<uint4> for int4 {
  #[inline(always)]
  fn select(self, a: uint4, b: uint4) -> uint4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uint4, b: uint4) -> uint4 {
    return uint4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl simd::Select<float4> for int4 {
  #[inline(always)]
  fn select(self, a: float4, b: float4) -> float4 {
    return (self >> 31).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: float4, b: float4) -> float4 {
    return float4::bitcast(self.bitselect(int4::bitcast(a), int4::bitcast(b)));
  }
}

impl int4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> Self {
    return int4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: int4, y: int4, z: int4) -> int4 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: int4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: int4) -> char4 {
    return int4::to_char(simd::clamp(x, int4::broadcast(std::i8::MIN as i32), int4::broadcast(std::i8::MAX as i32)));
  }

  #[inline]
  pub fn to_uchar(x: int4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: int4) -> uchar4 {
    return int4::to_uchar(simd::clamp(x, int4::broadcast(std::u8::MIN as i32), int4::broadcast(std::u8::MAX as i32)));
  }

  #[inline]
  pub fn to_short(x: int4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: int4) -> short4 {
    return int4::to_short(simd::clamp(x, int4::broadcast(std::i16::MIN as i32), int4::broadcast(std::i16::MAX as i32)));
  }

  #[inline]
  pub fn to_ushort(x: int4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: int4) -> ushort4 {
    return int4::to_ushort(simd::clamp(x, int4::broadcast(std::u16::MIN as i32), int4::broadcast(std::u16::MAX as i32)));
  }

  #[inline]
  pub fn to_int(x: int4) -> int4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: int4) -> int4 {
    return x;
  }

  #[inline]
  pub fn to_uint(x: int4) -> uint4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: int4) -> uint4 {
    return int4::to_uint(simd::max(x, int4::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: int4) -> float4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: int4) -> long4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: int4) -> long4 {
    return int4::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: int4) -> ulong4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: int4) -> ulong4 {
    return int4::to_ulong(simd::max(x, int4::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: int4) -> double4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> int2 {
    return int2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> int2 {
    return int2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> int2 {
    return int2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> int2 {
    return int2(self.0, self.2);
  }
}
