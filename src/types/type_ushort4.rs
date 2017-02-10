use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ushort4(pub u16, pub u16, pub u16, pub u16);
pub type vector_ushort4 = ushort4;

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

impl std::ops::Index<u32> for ushort4 {
  type Output = u16;

  #[inline]
  fn index(&self, index: u32) -> &u16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ushort4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn add(self, other: u16) -> Self {
    return unsafe { simd_add(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Add<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn add(self, other: ushort4) -> ushort4 {
    return unsafe { simd_add(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Sub for ushort4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u16) -> Self {
    return unsafe { simd_sub(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Sub<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn sub(self, other: ushort4) -> ushort4 {
    return unsafe { simd_sub(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Mul for ushort4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u16) -> Self {
    return unsafe { simd_mul(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Mul<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn mul(self, other: ushort4) -> ushort4 {
    return unsafe { simd_mul(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Div for ushort4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn div(self, other: u16) -> Self {
    return unsafe { simd_div(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Div<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn div(self, other: ushort4) -> ushort4 {
    return unsafe { simd_div(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ushort4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u16) -> Self {
    return unsafe { simd_and(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitand(self, other: ushort4) -> ushort4 {
    return unsafe { simd_and(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ushort4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u16) -> Self {
    return unsafe { simd_or(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitOr<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitor(self, other: ushort4) -> ushort4 {
    return unsafe { simd_or(ushort4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ushort4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u16) -> Self {
    return unsafe { simd_xor(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::BitXor<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn bitxor(self, other: ushort4) -> ushort4 {
    return unsafe { simd_xor(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Shl<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shl(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shl(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort4> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort4::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort4> for u16 {
  type Output = ushort4;

  #[inline]
  fn shr(self, other: ushort4) -> ushort4 {
    return unsafe { simd_shr(ushort4::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short4::all(ushort4::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short4::all(ushort4::ne(*self, *other));
  }
}

impl Dot for ushort4 {
  type Output = u16;

  #[inline]
  fn dot(self, other: ushort4) -> u16 {
    return ushort4::reduce_add(self * other);
  }
}

impl ushort4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> ushort4 {
    return ushort4(x, x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u16) -> ushort4 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ushort4, y: ushort4) -> short4 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn max(x: ushort4, y: ushort4) -> ushort4 {
    return ushort4::bitselect(x, y, ushort4::gt(y, x));
  }

  #[inline]
  pub fn min(x: ushort4, y: ushort4) -> ushort4 {
    return ushort4::bitselect(x, y, ushort4::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ushort4, min: ushort4, max: ushort4) -> ushort4 {
    return ushort4::min(ushort4::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ushort4) -> u16 {
    return ushort2::reduce_add(x.lo() + x.hi());
  }

  #[inline]
  pub fn reduce_min(x: ushort4) -> u16 {
    return ushort2::reduce_min(ushort2::min(x.lo(), x.hi()));
  }

  #[inline]
  pub fn reduce_max(x: ushort4) -> u16 {
    return ushort2::reduce_max(ushort2::max(x.lo(), x.hi()));
  }

  #[inline]
  pub fn all(x: ushort4) -> bool {
    return (x.0 & x.1 & x.2 & x.3) & 0x8000 != 0;
  }

  #[inline]
  pub fn any(x: ushort4) -> bool {
    return (x.0 | x.1 | x.2 | x.3) & 0x8000 != 0;
  }

  #[inline]
  pub fn bitselect(x: ushort4, y: ushort4, z: short4) -> ushort4 {
    return ushort4::bitcast(short4::bitselect(short4::bitcast(x), short4::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, self.3);
  }
}
