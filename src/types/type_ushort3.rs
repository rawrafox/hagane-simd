use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct ushort3(pub u16, pub u16, pub u16);
pub type vector_ushort3 = ushort3;

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

impl std::ops::Index<u32> for ushort3 {
  type Output = u16;

  #[inline]
  fn index(&self, index: u32) -> &u16 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for ushort3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn add(self, other: u16) -> Self {
    return unsafe { simd_add(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Add<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn add(self, other: ushort3) -> ushort3 {
    return unsafe { simd_add(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Sub for ushort3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: u16) -> Self {
    return unsafe { simd_sub(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Sub<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn sub(self, other: ushort3) -> ushort3 {
    return unsafe { simd_sub(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Mul for ushort3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: u16) -> Self {
    return unsafe { simd_mul(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Mul<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn mul(self, other: ushort3) -> ushort3 {
    return unsafe { simd_mul(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Div for ushort3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn div(self, other: u16) -> Self {
    return unsafe { simd_div(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Div<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn div(self, other: ushort3) -> ushort3 {
    return unsafe { simd_div(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for ushort3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: u16) -> Self {
    return unsafe { simd_and(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitAnd<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitand(self, other: ushort3) -> ushort3 {
    return unsafe { simd_and(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitOr for ushort3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: u16) -> Self {
    return unsafe { simd_or(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitOr<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitor(self, other: ushort3) -> ushort3 {
    return unsafe { simd_or(ushort3::broadcast(self), other) };
  }
}

impl std::ops::BitXor for ushort3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: u16) -> Self {
    return unsafe { simd_xor(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::BitXor<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn bitxor(self, other: ushort3) -> ushort3 {
    return unsafe { simd_xor(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Shl<ushort3> for ushort3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn shl(self, other: u16) -> Self {
    return unsafe { simd_shl(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Shl<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn shl(self, other: ushort3) -> ushort3 {
    return unsafe { simd_shl(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Shr<ushort3> for ushort3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<u16> for ushort3 {
  type Output = Self;

  #[inline]
  fn shr(self, other: u16) -> Self {
    return unsafe { simd_shr(self, ushort3::broadcast(other)) };
  }
}

impl std::ops::Shr<ushort3> for u16 {
  type Output = ushort3;

  #[inline]
  fn shr(self, other: ushort3) -> ushort3 {
    return unsafe { simd_shr(ushort3::broadcast(self), other) };
  }
}

impl std::ops::Not for ushort3 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ std::u16::MAX;
  }
}

impl PartialEq for ushort3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return short3::all(ushort3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return short3::all(ushort3::ne(*self, *other));
  }
}

impl Dot for ushort3 {
  type Output = u16;

  #[inline]
  fn dot(self, other: ushort3) -> u16 {
    return ushort3::reduce_add(self * other);
  }
}

impl ushort3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> ushort3 {
    return ushort3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> u16 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: u16) -> ushort3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: ushort3, y: ushort3) -> short3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: ushort3, y: ushort3, z: ushort3) -> ushort3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: ushort3) -> ushort3 {
    return x;
  }

  #[inline]
  pub fn max(x: ushort3, y: ushort3) -> ushort3 {
    return ushort3::bitselect(x, y, ushort3::gt(y, x));
  }

  #[inline]
  pub fn min(x: ushort3, y: ushort3) -> ushort3 {
    return ushort3::bitselect(x, y, ushort3::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: ushort3, min: ushort3, max: ushort3) -> ushort3 {
    return ushort3::min(ushort3::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: ushort3) -> u16 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: ushort3) -> u16 {
    return std::cmp::min(ushort2::reduce_min(x.lo()), x.2);
  }

  #[inline]
  pub fn reduce_max(x: ushort3) -> u16 {
    return std::cmp::max(ushort2::reduce_max(x.lo()), x.2);
  }

  #[inline]
  pub fn all(x: ushort3) -> bool {
    return (x.0 & x.1 & x.2) & 0x8000 != 0;
  }

  #[inline]
  pub fn any(x: ushort3) -> bool {
    return (x.0 | x.1 | x.2) & 0x8000 != 0;
  }

  #[inline]
  pub fn bitselect(x: ushort3, y: ushort3, z: short3) -> ushort3 {
    return ushort3::bitcast(short3::bitselect(short3::bitcast(x), short3::bitcast(y), z));
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}
