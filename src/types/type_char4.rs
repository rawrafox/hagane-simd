use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct char4(pub i8, pub i8, pub i8, pub i8);
pub type vector_char4 = char4;

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

impl std::ops::Index<u32> for char4 {
  type Output = i8;

  #[inline]
  fn index(&self, index: u32) -> &i8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for char4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i8> for char4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i8) -> Self {
    return unsafe { simd_add(self, char4::broadcast(other)) };
  }
}

impl std::ops::Add<char4> for i8 {
  type Output = char4;

  #[inline]
  fn add(self, other: char4) -> char4 {
    return unsafe { simd_add(char4::broadcast(self), other) };
  }
}

impl std::ops::Sub for char4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i8> for char4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i8) -> Self {
    return unsafe { simd_sub(self, char4::broadcast(other)) };
  }
}

impl std::ops::Sub<char4> for i8 {
  type Output = char4;

  #[inline]
  fn sub(self, other: char4) -> char4 {
    return unsafe { simd_sub(char4::broadcast(self), other) };
  }
}

impl std::ops::Mul for char4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i8> for char4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i8) -> Self {
    return unsafe { simd_mul(self, char4::broadcast(other)) };
  }
}

impl std::ops::Mul<char4> for i8 {
  type Output = char4;

  #[inline]
  fn mul(self, other: char4) -> char4 {
    return unsafe { simd_mul(char4::broadcast(self), other) };
  }
}

impl std::ops::Div for char4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i8> for char4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i8) -> Self {
    return unsafe { simd_div(self, char4::broadcast(other)) };
  }
}

impl std::ops::Div<char4> for i8 {
  type Output = char4;

  #[inline]
  fn div(self, other: char4) -> char4 {
    return unsafe { simd_div(char4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for char4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i8) -> Self {
    return unsafe { simd_and(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitand(self, other: char4) -> char4 {
    return unsafe { simd_and(char4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for char4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i8) -> Self {
    return unsafe { simd_or(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitOr<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitor(self, other: char4) -> char4 {
    return unsafe { simd_or(char4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for char4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i8> for char4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i8) -> Self {
    return unsafe { simd_xor(self, char4::broadcast(other)) };
  }
}

impl std::ops::BitXor<char4> for i8 {
  type Output = char4;

  #[inline]
  fn bitxor(self, other: char4) -> char4 {
    return unsafe { simd_xor(char4::broadcast(self), other) };
  }
}

impl std::ops::Shl<char4> for char4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i8> for char4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i8) -> Self {
    return unsafe { simd_shl(self, char4::broadcast(other)) };
  }
}

impl std::ops::Shl<char4> for i8 {
  type Output = char4;

  #[inline]
  fn shl(self, other: char4) -> char4 {
    return unsafe { simd_shl(char4::broadcast(self), other) };
  }
}

impl std::ops::Shr<char4> for char4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i8> for char4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i8) -> Self {
    return unsafe { simd_shr(self, char4::broadcast(other)) };
  }
}

impl std::ops::Shr<char4> for i8 {
  type Output = char4;

  #[inline]
  fn shr(self, other: char4) -> char4 {
    return unsafe { simd_shr(char4::broadcast(self), other) };
  }
}

impl std::ops::Not for char4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ char4::broadcast(-1);
  }
}

impl PartialEq for char4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return char4::all(char4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return char4::all(char4::ne(*self, *other));
  }
}

impl Dot for char4 {
  type Output = i8;

  #[inline]
  fn dot(self, other: char4) -> i8 {
    return char4::reduce_add(self * other);
  }
}

impl char4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i8) -> char4 {
    return char4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i8 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i8) -> char4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: char4, y: char4) -> char4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: char4, y: char4) -> char4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: char4, y: char4) -> char4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: char4, y: char4) -> char4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: char4, y: char4) -> char4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: char4, y: char4) -> char4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn abs(x: char4) -> char4 {
    return char4(x.0.abs(), x.1.abs(), x.2.abs(), x.3.abs());
  }

  #[inline]
  pub fn max(x: char4, y: char4) -> char4 {
    return char4(std::cmp::max(x.0, y.0), std::cmp::max(x.1, y.1), std::cmp::max(x.2, y.2), std::cmp::max(x.3, y.3));
  }

  #[inline]
  pub fn min(x: char4, y: char4) -> char4 {
    return char4(std::cmp::min(x.0, y.0), std::cmp::min(x.1, y.1), std::cmp::min(x.2, y.2), std::cmp::min(x.3, y.3));
  }

  #[inline]
  pub fn clamp(x: char4, min: char4, max: char4) -> char4 {
    return char4::min(char4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: char4) -> i8 {
    return char2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: char4) -> i8 {
    return char2::reduce_min(char2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: char4) -> i8 {
    return char2::reduce_max(char2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: char4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn any(x: char4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & std::i8::MIN != 0;
  }

  #[inline]
  pub fn bitselect(x: char4, y: char4, z: char4) -> char4 {
    return (x & !z) | (y & z);
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, self.3);
  }
}
