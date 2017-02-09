use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct int2(pub i32, pub i32);
pub type vector_int2 = int2;

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

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for int2 {
  type Output = i32;

  #[inline]
  fn index(&self, index: u32) -> &i32 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i32> for int2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i32) -> Self {
    return unsafe { simd_add(self, int2::broadcast(other)) };
  }
}

impl std::ops::Add<int2> for i32 {
  type Output = int2;

  #[inline]
  fn add(self, other: int2) -> int2 {
    return unsafe { simd_add(int2::broadcast(self), other) };
  }
}

impl std::ops::Sub for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i32> for int2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i32) -> Self {
    return unsafe { simd_sub(self, int2::broadcast(other)) };
  }
}

impl std::ops::Sub<int2> for i32 {
  type Output = int2;

  #[inline]
  fn sub(self, other: int2) -> int2 {
    return unsafe { simd_sub(int2::broadcast(self), other) };
  }
}

impl std::ops::Mul for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i32> for int2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i32) -> Self {
    return unsafe { simd_mul(self, int2::broadcast(other)) };
  }
}

impl std::ops::Mul<int2> for i32 {
  type Output = int2;

  #[inline]
  fn mul(self, other: int2) -> int2 {
    return unsafe { simd_mul(int2::broadcast(self), other) };
  }
}

impl std::ops::Div for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i32> for int2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i32) -> Self {
    return unsafe { simd_div(self, int2::broadcast(other)) };
  }
}

impl std::ops::Div<int2> for i32 {
  type Output = int2;

  #[inline]
  fn div(self, other: int2) -> int2 {
    return unsafe { simd_div(int2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for int2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i32) -> Self {
    return unsafe { simd_and(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitand(self, other: int2) -> int2 {
    return unsafe { simd_and(int2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for int2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i32) -> Self {
    return unsafe { simd_or(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitOr<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitor(self, other: int2) -> int2 {
    return unsafe { simd_or(int2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for int2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i32> for int2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i32) -> Self {
    return unsafe { simd_xor(self, int2::broadcast(other)) };
  }
}

impl std::ops::BitXor<int2> for i32 {
  type Output = int2;

  #[inline]
  fn bitxor(self, other: int2) -> int2 {
    return unsafe { simd_xor(int2::broadcast(self), other) };
  }
}

impl std::ops::Shl<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i32) -> Self {
    return unsafe { simd_shl(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shl<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shl(self, other: int2) -> int2 {
    return unsafe { simd_shl(int2::broadcast(self), other) };
  }
}

impl std::ops::Shr<int2> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i32> for int2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i32) -> Self {
    return unsafe { simd_shr(self, int2::broadcast(other)) };
  }
}

impl std::ops::Shr<int2> for i32 {
  type Output = int2;

  #[inline]
  fn shr(self, other: int2) -> int2 {
    return unsafe { simd_shr(int2::broadcast(self), other) };
  }
}

impl std::ops::Not for int2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for int2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return int2::all(int2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return int2::all(int2::ne(*self, *other));
  }
}

impl Dot for int2 {
  type Output = i32;

  #[inline]
  fn dot(self, other: int2) -> i32 {
    return int2::reduce_add(self * other);
  }
}

impl int2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> int2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i32) -> int2 {
    return int2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i32 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i32) -> int2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: int2, y: int2) -> int2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: int2, y: int2) -> int2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: int2, y: int2) -> int2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: int2, y: int2) -> int2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: int2, y: int2) -> int2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: int2, y: int2) -> int2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: int2) -> int2 {
    let mask = x >> 31;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: int2, y: int2) -> int2 {
    return int2::bitselect(x, y, int2::gt(y, x));
  }

  #[inline]
  pub fn min(x: int2, y: int2) -> int2 {
    return int2::bitselect(x, y, int2::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: int2, min: int2, max: int2) -> int2 {
    return int2::min(int2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: int2) -> i32 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: int2) -> i32 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: int2) -> i32 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: int2) -> bool {
    return (x.0 & x.1) & std::i32::MIN != 0;
  }

  #[inline]
  pub fn any(x: int2) -> bool {
    return (x.0 | x.1) & std::i32::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: int2, y: int2, z: int2) -> int2 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> int1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> int1 {
    return self.1;
  }
}
