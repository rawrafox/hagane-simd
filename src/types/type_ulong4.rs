use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ulong4(pub u64, pub u64, pub u64, pub u64);
pub type vector_ulong4 = ulong4;

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

impl std::ops::Index<u32> for ulong4 {
  type Output = u64;

  #[inline]
  fn index(&self, index: u32) -> &u64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ulong4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u64) -> Self {
    return unsafe { simd_add(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Add<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn add(self, other: ulong4) -> ulong4 {
    return unsafe { simd_add(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Sub for ulong4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u64) -> Self {
    return unsafe { simd_sub(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Sub<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn sub(self, other: ulong4) -> ulong4 {
    return unsafe { simd_sub(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Mul for ulong4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u64) -> Self {
    return unsafe { simd_mul(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Mul<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn mul(self, other: ulong4) -> ulong4 {
    return unsafe { simd_mul(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Div for ulong4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u64) -> Self {
    return unsafe { simd_div(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Div<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn div(self, other: ulong4) -> ulong4 {
    return unsafe { simd_div(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ulong4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u64) -> Self {
    return unsafe { simd_and(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitand(self, other: ulong4) -> ulong4 {
    return unsafe { simd_and(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ulong4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u64) -> Self {
    return unsafe { simd_or(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitOr<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitor(self, other: ulong4) -> ulong4 {
    return unsafe { simd_or(ulong4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ulong4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u64) -> Self {
    return unsafe { simd_xor(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::BitXor<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn bitxor(self, other: ulong4) -> ulong4 {
    return unsafe { simd_xor(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Shl<ulong4> for ulong4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u64) -> Self {
    return unsafe { simd_shl(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Shl<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn shl(self, other: ulong4) -> ulong4 {
    return unsafe { simd_shl(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Shr<ulong4> for ulong4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u64> for ulong4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u64) -> Self {
    return unsafe { simd_shr(self, ulong4::broadcast(other)) };
  }
}

impl std::ops::Shr<ulong4> for u64 {
  type Output = ulong4;

  #[inline]
  fn shr(self, other: ulong4) -> ulong4 {
    return unsafe { simd_shr(ulong4::broadcast(self), other) };
  }
}

impl std::ops::Not for ulong4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u64::MAX;
  }
}

impl PartialEq for ulong4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return long4::all(ulong4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return long4::all(ulong4::ne(*self, *other));
  }
}

impl Dot for ulong4 {
  type Output = u64;

  #[inline]
  fn dot(self, other: ulong4) -> u64 {
    return ulong4::reduce_add(self * other);
  }
}

impl ulong4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u64) -> ulong4 {
    return ulong4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u64) -> ulong4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ulong4, y: ulong4) -> long4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: ulong4, y: ulong4, z: ulong4) -> ulong4 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: ulong4) -> ulong4 {
    return x;
  }

  #[inline]
  pub fn max(x: ulong4, y: ulong4) -> ulong4 {
    return ulong4::bitselect(x, y, ulong4::gt(y, x));
  }

  #[inline]
  pub fn min(x: ulong4, y: ulong4) -> ulong4 {
    return ulong4::bitselect(x, y, ulong4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ulong4, min: ulong4, max: ulong4) -> ulong4 {
    return ulong4::min(ulong4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ulong4) -> u64 {
    return ulong2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: ulong4) -> u64 {
    return ulong2::reduce_min(ulong2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: ulong4) -> u64 {
    return ulong2::reduce_max(ulong2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: ulong4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & 0x8000000000000000 != 0;
  }

  #[inline]
  pub fn any(x: ulong4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & 0x8000000000000000 != 0;
  }

  #[inline]
  pub fn bitselect(x: ulong4, y: ulong4, z: long4) -> ulong4 {
    return ulong4::bitcast(long4::bitselect(long4::bitcast(x), long4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> ulong2 {
    return ulong2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ulong2 {
    return ulong2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> ulong2 {
    return ulong2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> ulong2 {
    return ulong2(self.0, self.2);
  }
}
