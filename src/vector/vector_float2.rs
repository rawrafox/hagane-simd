use std;
use ::*;

impl Vector for float2 {
  type Scalar = f32;
  type Boolean = int2;

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

  const ZERO: Self = float2(0.0, 0.0);
  const ONE: Self = float2(1.0, 1.0);
  const TWO: Self = float2(2.0, 2.0);
  const THREE: Self = float2(3.0, 3.0);

  #[inline(always)]
  fn abs(self) -> Self {
    let x: Self::Boolean = std::i32::MAX.broadcast();

    return x.bitselect(Self::ZERO, self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return float2(self.0.max(other.0), self.1.max(other.1));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return float2(self.0.min(other.0), self.1.min(other.1));
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
    return float2::to_char(clamp(self, broadcast(std::i8::MIN as f32), broadcast(std::i8::MAX as f32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return float2::to_uchar(clamp(self, broadcast(std::u8::MIN as f32), broadcast(std::u8::MAX as f32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return float2::to_short(clamp(self, broadcast(std::i16::MIN as f32), broadcast(std::i16::MAX as f32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return float2::to_ushort(clamp(self, broadcast(std::u16::MIN as f32), broadcast(std::u16::MAX as f32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return float2::to_int(clamp(self, broadcast(std::i32::MIN as f32), broadcast(std::i32::MAX as f32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return float2::to_uint(clamp(self, broadcast(std::u32::MIN as f32), broadcast(std::u32::MAX as f32)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return float2::to_long(clamp(self, broadcast(std::i64::MIN as f32), broadcast(std::i64::MAX as f32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return float2::to_ulong(clamp(self, broadcast(std::u64::MIN as f32), broadcast(std::u64::MAX as f32)));
  }
}

impl Cross for float2 {
  type CrossProduct = float3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    return float3(0.0, 0.0, self.0 * other.1 - self.1 * other.0);
  }
}

impl Dot<float2> for float2 {
  type DotProduct = f32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for float2 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = std::i32::MAX.broadcast();

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return float2(self.0.sqrt(), self.1.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return float2(self.0.fract(), self.1.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return float2(self.0.ceil(), self.1.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return float2(self.0.floor(), self.1.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return float2(self.0.trunc(), self.1.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return float2(self.0.sin(), self.1.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return float2(self.0.cos(), self.1.cos());
  }
}

impl Geometry for float2 {
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

impl float2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> f32 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> f32 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> f32 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> f32 {
    return self.0;
  }
}
