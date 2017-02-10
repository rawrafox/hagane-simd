use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct uchar4(pub u8, pub u8, pub u8, pub u8);
pub type vector_uchar4 = uchar4;

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

impl std::ops::Index<u32> for uchar4 {
  type Output = u8;

  #[inline]
  fn index(&self, index: u32) -> &u8 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for uchar4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u8) -> Self {
    return unsafe { simd_add(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Add<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn add(self, other: uchar4) -> uchar4 {
    return unsafe { simd_add(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Sub for uchar4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u8) -> Self {
    return unsafe { simd_sub(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Sub<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn sub(self, other: uchar4) -> uchar4 {
    return unsafe { simd_sub(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Mul for uchar4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u8) -> Self {
    return unsafe { simd_mul(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Mul<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn mul(self, other: uchar4) -> uchar4 {
    return unsafe { simd_mul(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Div for uchar4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u8) -> Self {
    return unsafe { simd_div(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Div<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn div(self, other: uchar4) -> uchar4 {
    return unsafe { simd_div(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for uchar4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u8) -> Self {
    return unsafe { simd_and(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitand(self, other: uchar4) -> uchar4 {
    return unsafe { simd_and(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for uchar4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u8) -> Self {
    return unsafe { simd_or(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitOr<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitor(self, other: uchar4) -> uchar4 {
    return unsafe { simd_or(uchar4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for uchar4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u8) -> Self {
    return unsafe { simd_xor(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::BitXor<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn bitxor(self, other: uchar4) -> uchar4 {
    return unsafe { simd_xor(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Shl<uchar4> for uchar4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u8) -> Self {
    return unsafe { simd_shl(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Shl<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn shl(self, other: uchar4) -> uchar4 {
    return unsafe { simd_shl(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Shr<uchar4> for uchar4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u8> for uchar4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u8) -> Self {
    return unsafe { simd_shr(self, uchar4::broadcast(other)) };
  }
}

impl std::ops::Shr<uchar4> for u8 {
  type Output = uchar4;

  #[inline]
  fn shr(self, other: uchar4) -> uchar4 {
    return unsafe { simd_shr(uchar4::broadcast(self), other) };
  }
}

impl std::ops::Not for uchar4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u8::MAX;
  }
}

impl PartialEq for uchar4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return char4::all(uchar4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return char4::all(uchar4::ne(*self, *other));
  }
}

impl Dot for uchar4 {
  type Output = u8;

  #[inline]
  fn dot(self, other: uchar4) -> u8 {
    return uchar4::reduce_add(self * other);
  }
}

impl uchar4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u8) -> uchar4 {
    return uchar4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u8 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u8) -> uchar4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: uchar4, y: uchar4) -> char4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: uchar4, y: uchar4, z: uchar4) -> uchar4 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: uchar4) -> uchar4 {
    return x;
  }

  #[inline]
  pub fn max(x: uchar4, y: uchar4) -> uchar4 {
    return uchar4::bitselect(x, y, uchar4::gt(y, x));
  }

  #[inline]
  pub fn min(x: uchar4, y: uchar4) -> uchar4 {
    return uchar4::bitselect(x, y, uchar4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: uchar4, min: uchar4, max: uchar4) -> uchar4 {
    return uchar4::min(uchar4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: uchar4) -> u8 {
    return uchar2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: uchar4) -> u8 {
    return uchar2::reduce_min(uchar2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: uchar4) -> u8 {
    return uchar2::reduce_max(uchar2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: uchar4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & 0x80 != 0;
  }

  #[inline]
  pub fn any(x: uchar4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & 0x80 != 0;
  }

  #[inline]
  pub fn bitselect(x: uchar4, y: uchar4, z: char4) -> uchar4 {
    return uchar4::bitcast(char4::bitselect(char4::bitcast(x), char4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> uchar2 {
    return uchar2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uchar2 {
    return uchar2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> uchar2 {
    return uchar2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> uchar2 {
    return uchar2(self.0, self.2);
  }
}
