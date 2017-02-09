use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct char2(pub i8, pub i8);
pub type vector_char2 = char2;

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

impl std::ops::Index<u32> for char2 {
  type Output = i8;

  #[inline]
  fn index(&self, index: u32) -> &i8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for char2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char2::broadcast(other)) };
  }
}

impl std::ops::Add<char2> for i8 {
  type Output = char2;

  #[inline]
  fn add(self, other: char2) -> char2 {
    return unsafe { simd_add(char2::broadcast(self), other) };
  }
}

impl std::ops::Sub for char2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char2::broadcast(other)) };
  }
}

impl std::ops::Sub<char2> for i8 {
  type Output = char2;

  #[inline]
  fn sub(self, other: char2) -> char2 {
    return unsafe { simd_sub(char2::broadcast(self), other) };
  }
}

impl std::ops::Mul for char2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char2::broadcast(other)) };
  }
}

impl std::ops::Mul<char2> for i8 {
  type Output = char2;

  #[inline]
  fn mul(self, other: char2) -> char2 {
    return unsafe { simd_mul(char2::broadcast(self), other) };
  }
}

impl std::ops::Div for char2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char2::broadcast(other)) };
  }
}

impl std::ops::Div<char2> for i8 {
  type Output = char2;

  #[inline]
  fn div(self, other: char2) -> char2 {
    return unsafe { simd_div(char2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitand(self, other: char2) -> char2 {
    return unsafe { simd_and(char2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitOr<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitor(self, other: char2) -> char2 {
    return unsafe { simd_or(char2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char2::broadcast(other)) };
  }
}

impl std::ops::BitXor<char2> for i8 {
  type Output = char2;

  #[inline]
  fn bitxor(self, other: char2) -> char2 {
    return unsafe { simd_xor(char2::broadcast(self), other) };
  }
}

impl std::ops::Shl<char2> for char2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char2::broadcast(other)) };
  }
}

impl std::ops::Shl<char2> for i8 {
  type Output = char2;

  #[inline]
  fn shl(self, other: char2) -> char2 {
    return unsafe { simd_shl(char2::broadcast(self), other) };
  }
}

impl std::ops::Shr<char2> for char2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char2::broadcast(other)) };
  }
}

impl std::ops::Shr<char2> for i8 {
  type Output = char2;

  #[inline]
  fn shr(self, other: char2) -> char2 {
    return unsafe { simd_shr(char2::broadcast(self), other) };
  }
}

impl std::ops::Not for char2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ char2::broadcast(-1);
  }
}

impl PartialEq for char2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return char2::all(char2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return char2::all(char2::ne(*self, *other));
  }
}

impl Dot for char2 {
  type Output = i8;

  #[inline]
  fn dot(self, other: char2) -> i8 {
    return char2::reduce_add(self * other);
  }
}

impl char2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> char2 {
    return char2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i8 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i8) -> char2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: char2, y: char2) -> char2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: char2, y: char2) -> char2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: char2, y: char2) -> char2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: char2, y: char2) -> char2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: char2, y: char2) -> char2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: char2, y: char2) -> char2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: char2) -> char2 {
    let mask = x >> 7;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: char2, y: char2) -> char2 {
    return char2(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1));
  }

  #[inline]
  pub fn min(x: char2, y: char2) -> char2 {
    return char2(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1));
  }

  #[inline]
  pub fn clamp(x: char2, min: char2, max: char2) -> char2 {
    return char2::min(char2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: char2) -> i8 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: char2) -> i8 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: char2) -> i8 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn all(x: char2) -> bool {
    return (x.0 & x.1) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn any(x: char2) -> bool {
    return (x.0 | x.1) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: char2, y: char2, z: char2) -> char2 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> char1 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> char1 {
    return self.1;
  }
}
