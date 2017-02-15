use std;
use ::*;

impl Vector for double2 {
  type Scalar = f64;
  type Boolean = long2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return double2(f(self.0), f(self.1));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return double2(f(self.0, other.0), f(self.1, other.1));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let x = Self::Boolean::broadcast(std::i64::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.0.min(self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.0.max(self.1);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return double2::to_char(self.clamp(Self::broadcast(std::i8::MIN as f64), Self::broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return double2::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as f64), Self::broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return double2::to_short(self.clamp(Self::broadcast(std::i16::MIN as f64), Self::broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return double2::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as f64), Self::broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return double2::to_int(self.clamp(Self::broadcast(std::i32::MIN as f64), Self::broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return double2::to_uint(self.clamp(Self::broadcast(std::u32::MIN as f64), Self::broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return double2::to_long(self.clamp(Self::broadcast(std::i64::MIN as f64), Self::broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return double2::to_ulong(self.clamp(Self::broadcast(std::u64::MIN as f64), Self::broadcast(std::u64::MAX as f64)));
  }
}

impl Cross for double2 {
  type CrossProduct = double3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    return double3(0.0, 0.0, self.0 * other.1 - self.1 * other.0);
  }
}

impl Dot<double2> for double2 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for double2 {
  type FloatScalar = f64;
  const SIGN_MASK: long2 = long2(std::i64::MAX, std::i64::MAX);
}

impl Geometry for double2 {
}

impl double2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> f64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> f64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> f64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> f64 {
    return self.0;
  }
}
