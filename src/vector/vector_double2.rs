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

  const ZERO: Self = double2(0.0, 0.0);
  const ONE: Self = double2(1.0, 1.0);
  const TWO: Self = double2(2.0, 2.0);
  const THREE: Self = double2(3.0, 3.0);

  #[inline(always)]
  fn abs(self) -> Self {
    let x: Self::Boolean = std::i64::MAX.broadcast();

    return x.bitselect(Self::ZERO, self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double2(self.0.max(other.0), self.1.max(other.1));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double2(self.0.min(other.0), self.1.min(other.1));
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
    return double2::to_char(clamp(self, broadcast(std::i8::MIN as f64), broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return double2::to_uchar(clamp(self, broadcast(std::u8::MIN as f64), broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return double2::to_short(clamp(self, broadcast(std::i16::MIN as f64), broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return double2::to_ushort(clamp(self, broadcast(std::u16::MIN as f64), broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return double2::to_int(clamp(self, broadcast(std::i32::MIN as f64), broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return double2::to_uint(clamp(self, broadcast(std::u32::MIN as f64), broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return double2::to_long(clamp(self, broadcast(std::i64::MIN as f64), broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return double2::to_ulong(clamp(self, broadcast(std::u64::MIN as f64), broadcast(std::u64::MAX as f64)));
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
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = std::i64::MAX.broadcast();

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double2(self.0.sqrt(), self.1.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double2(self.0.fract(), self.1.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double2(self.0.ceil(), self.1.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double2(self.0.floor(), self.1.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double2(self.0.trunc(), self.1.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double2(self.0.sin(), self.1.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double2(self.0.cos(), self.1.cos());
  }
}

impl Geometry for double2 {
  #[inline(always)]
  fn project(self, onto: Self) -> Self {
    return (self.dot(onto) / onto.dot(onto)) * onto;
  }

  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }

  #[inline(always)]
  fn length_squared(self) -> Self::Scalar {
    return self.dot(self);
  }

  #[inline(always)]
  fn normalize(self) -> Self {
    let x: Self = self.length_squared().broadcast();

    return self * x.rsqrt();
  }

  #[inline(always)]
  fn reflect(self, n: Self) -> Self {
    return self - 2.0 * self.dot(n) * n;
  }

  #[inline(always)]
  fn refract(self, n: Self, eta: Self::Scalar) -> Self {
    let dp = self.dot(n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);

    return if k >= 0.0 { eta * self - (eta * dp + k.sqrt()) } else { Self::ZERO };
  }
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
