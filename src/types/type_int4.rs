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
    return simd::all(int4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(int4::ne(*self, *other));
  }
}

impl simd::Vector for int4 {
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

impl simd::Dot for int4 {
  type Output = i32;

  #[inline]
  fn dot(self, other: int4) -> i32 {
    return int4::reduce_add(self * other);
  }
}

impl int4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> int4 {
    return int4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i32) -> int4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: int4, y: int4) -> int4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: int4, y: int4) -> int4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: int4, y: int4) -> int4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: int4, y: int4) -> int4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: int4, y: int4) -> int4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: int4, y: int4) -> int4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: int4, y: int4, z: int4) -> int4 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: int4) -> int4 {
    let mask = x >> 31;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: int4, y: int4) -> int4 {
    return int4::bitselect(x, y, int4::gt(y, x));
  }

  #[inline]
  pub fn min(x: int4, y: int4) -> int4 {
    return int4::bitselect(x, y, int4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: int4, min: int4, max: int4) -> int4 {
    return int4::min(int4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: int4) -> i32 {
    return int2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: int4) -> i32 {
    return int2::reduce_min(int2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: int4) -> i32 {
    return int2::reduce_max(int2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn bitselect(x: int4, y: int4, z: int4) -> int4 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn to_char(x: int4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: int4) -> char4 {
    return int4::to_char(int4::clamp(x, int4::broadcast(std::i8::MIN as i32), int4::broadcast(std::i8::MAX as i32)));
  }

  #[inline]
  pub fn to_uchar(x: int4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: int4) -> uchar4 {
    return int4::to_uchar(int4::clamp(x, int4::broadcast(std::u8::MIN as i32), int4::broadcast(std::u8::MAX as i32)));
  }

  #[inline]
  pub fn to_short(x: int4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: int4) -> short4 {
    return int4::to_short(int4::clamp(x, int4::broadcast(std::i16::MIN as i32), int4::broadcast(std::i16::MAX as i32)));
  }

  #[inline]
  pub fn to_ushort(x: int4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: int4) -> ushort4 {
    return int4::to_ushort(int4::clamp(x, int4::broadcast(std::u16::MIN as i32), int4::broadcast(std::u16::MAX as i32)));
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
    return int4::to_uint(int4::max(x, int4::broadcast(0)));
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
    return int4::to_ulong(int4::max(x, int4::broadcast(0)));
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
