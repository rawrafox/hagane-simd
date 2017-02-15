use std;
use ::*;

impl Vector for double3 {
  type Scalar = f64;
  type Boolean = long3;

  type CharVector = char3;
  type ShortVector = short3;
  type IntVector = int3;
  type LongVector = long3;

  type UCharVector = uchar3;
  type UShortVector = ushort3;
  type UIntVector = uint3;
  type ULongVector = ulong3;

  type FloatVector = float3;
  type DoubleVector = double3;

  #[inline(always)]
  fn abs(self) -> Self {
    let x = Self::Boolean::broadcast(std::i64::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double3(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double3(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2));
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.2.min(reduce_min(self.lo()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.2.max(reduce_max(self.lo()));
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return double3::to_char(self.clamp(Self::broadcast(std::i8::MIN as f64), Self::broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return double3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as f64), Self::broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return double3::to_short(self.clamp(Self::broadcast(std::i16::MIN as f64), Self::broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return double3::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as f64), Self::broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return double3::to_int(self.clamp(Self::broadcast(std::i32::MIN as f64), Self::broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return double3::to_uint(self.clamp(Self::broadcast(std::u32::MIN as f64), Self::broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return double3::to_long(self.clamp(Self::broadcast(std::i64::MIN as f64), Self::broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return double3::to_ulong(self.clamp(Self::broadcast(std::u64::MIN as f64), Self::broadcast(std::u64::MAX as f64)));
  }
}

impl Cross for double3 {
  type CrossProduct = double3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    let a = self * double3(other.2, other.1, other.0) - double3(self.2, self.1, self.0) * other;

    return double3(a.2, a.1, a.0);
  }
}

impl Dot<double3> for double3 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for double3 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = broadcast(std::i64::MAX);

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double3(self.0.fract(), self.1.fract(), self.2.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double3(self.0.ceil(), self.1.ceil(), self.2.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double3(self.0.floor(), self.1.floor(), self.2.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double3(self.0.trunc(), self.1.trunc(), self.2.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double3(self.0.sin(), self.1.sin(), self.2.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double3(self.0.cos(), self.1.cos(), self.2.cos());
  }
}

impl Geometry for double3 {
  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }
}

impl double3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, 0.0);
  }

  #[inline]
  pub fn odd(self) -> double2 {
    return double2(self.1, 0.0);
  }

  #[inline]
  pub fn even(self) -> double2 {
    return double2(self.0, self.2);
  }
}
