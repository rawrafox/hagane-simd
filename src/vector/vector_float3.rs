use std;
use ::*;

impl Vector for float3 {
  type Scalar = f32;
  type Boolean = int3;

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
    let x: Self::Boolean = broadcast(std::i32::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return float3(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return float3(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2));
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
    return float3::to_char(clamp(self, broadcast(std::i8::MIN as f32), broadcast(std::i8::MAX as f32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return float3::to_uchar(clamp(self, broadcast(std::u8::MIN as f32), broadcast(std::u8::MAX as f32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return float3::to_short(clamp(self, broadcast(std::i16::MIN as f32), broadcast(std::i16::MAX as f32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return float3::to_ushort(clamp(self, broadcast(std::u16::MIN as f32), broadcast(std::u16::MAX as f32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return float3::to_int(clamp(self, broadcast(std::i32::MIN as f32), broadcast(std::i32::MAX as f32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return float3::to_uint(clamp(self, broadcast(std::u32::MIN as f32), broadcast(std::u32::MAX as f32)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return float3::to_long(clamp(self, broadcast(std::i64::MIN as f32), broadcast(std::i64::MAX as f32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return float3::to_ulong(clamp(self, broadcast(std::u64::MIN as f32), broadcast(std::u64::MAX as f32)));
  }
}

impl Cross for float3 {
  type CrossProduct = float3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    let a = self * float3(other.2, other.1, other.0) - float3(self.2, self.1, self.0) * other;

    return float3(a.2, a.1, a.0);
  }
}

impl Dot<float3> for float3 {
  type DotProduct = f32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for float3 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = broadcast(std::i32::MAX);

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return float3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return float3(self.0.fract(), self.1.fract(), self.2.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return float3(self.0.ceil(), self.1.ceil(), self.2.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return float3(self.0.floor(), self.1.floor(), self.2.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return float3(self.0.trunc(), self.1.trunc(), self.2.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return float3(self.0.sin(), self.1.sin(), self.2.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return float3(self.0.cos(), self.1.cos(), self.2.cos());
  }
}

impl Geometry for float3 {
  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }
}

impl float3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> float3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> float2 {
    return float2(self.2, 0.0);
  }

  #[inline]
  pub fn odd(self) -> float2 {
    return float2(self.1, 0.0);
  }

  #[inline]
  pub fn even(self) -> float2 {
    return float2(self.0, self.2);
  }
}
