use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct char3(pub i8, pub i8, pub i8);
pub type vector_char3 = char3;

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

impl std::ops::Index<u32> for char3 {
  type Output = i8;

  #[inline]
  fn index(&self, index: u32) -> &i8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for char3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char3 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char3::broadcast(other)) };
  }
}

impl std::ops::Add<char3> for i8 {
  type Output = char3;

  #[inline]
  fn add(self, other: char3) -> char3 {
    return unsafe { simd_add(char3::broadcast(self), other) };
  }
}

impl std::ops::Sub for char3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char3::broadcast(other)) };
  }
}

impl std::ops::Sub<char3> for i8 {
  type Output = char3;

  #[inline]
  fn sub(self, other: char3) -> char3 {
    return unsafe { simd_sub(char3::broadcast(self), other) };
  }
}

impl std::ops::Mul for char3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char3::broadcast(other)) };
  }
}

impl std::ops::Mul<char3> for i8 {
  type Output = char3;

  #[inline]
  fn mul(self, other: char3) -> char3 {
    return unsafe { simd_mul(char3::broadcast(self), other) };
  }
}

impl std::ops::Div for char3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char3 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char3::broadcast(other)) };
  }
}

impl std::ops::Div<char3> for i8 {
  type Output = char3;

  #[inline]
  fn div(self, other: char3) -> char3 {
    return unsafe { simd_div(char3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitand(self, other: char3) -> char3 {
    return unsafe { simd_and(char3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitOr<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitor(self, other: char3) -> char3 {
    return unsafe { simd_or(char3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char3::broadcast(other)) };
  }
}

impl std::ops::BitXor<char3> for i8 {
  type Output = char3;

  #[inline]
  fn bitxor(self, other: char3) -> char3 {
    return unsafe { simd_xor(char3::broadcast(self), other) };
  }
}

impl std::ops::Shl<char3> for char3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char3::broadcast(other)) };
  }
}

impl std::ops::Shl<char3> for i8 {
  type Output = char3;

  #[inline]
  fn shl(self, other: char3) -> char3 {
    return unsafe { simd_shl(char3::broadcast(self), other) };
  }
}

impl std::ops::Shr<char3> for char3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char3::broadcast(other)) };
  }
}

impl std::ops::Shr<char3> for i8 {
  type Output = char3;

  #[inline]
  fn shr(self, other: char3) -> char3 {
    return unsafe { simd_shr(char3::broadcast(self), other) };
  }
}

impl std::ops::Not for char3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ char3::broadcast(-1);
  }
}

impl PartialEq for char3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return char3::all(char3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return char3::all(char3::ne(*self, *other));
  }
}

impl Dot for char3 {
  type Output = i8;

  #[inline]
  fn dot(self, other: char3) -> i8 {
    return char3::reduce_add(self * other);
  }
}

impl char3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> char3 {
    return char3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i8 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i8) -> char3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: char3, y: char3) -> char3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: char3, y: char3) -> char3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: char3, y: char3) -> char3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: char3, y: char3) -> char3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: char3, y: char3) -> char3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: char3, y: char3) -> char3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: char3) -> char3 {
    let mask = x >> 7;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: char3, y: char3) -> char3 {
    return char3(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1), std::cmp::max(x.2, y.2));
  }

  #[inline]
  pub fn min(x: char3, y: char3) -> char3 {
    return char3(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1), std::cmp::min(x.2, y.2));
  }

  #[inline]
  pub fn clamp(x: char3, min: char3, max: char3) -> char3 {
    return char3::min(char3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: char3) -> i8 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: char3) -> i8 {
    return std::cmp::min(char2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: char3) -> i8 {
    return std::cmp::max(char2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn all(x: char3) -> bool {
    return (x.0 & x.1 & x.2) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn any(x: char3) -> bool {
    return (x.0 | x.1 | x.2) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: char3, y: char3, z: char3) -> char3 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, 0);
  }
}
