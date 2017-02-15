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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return float3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return float3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.2, f(self.1, self.0));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let x = Self::Boolean::broadcast(std::i32::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return float3::to_char(self.clamp(Self::broadcast(std::i8::MIN as f32), Self::broadcast(std::i8::MAX as f32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return float3::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as f32), Self::broadcast(std::u8::MAX as f32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return float3::to_short(self.clamp(Self::broadcast(std::i16::MIN as f32), Self::broadcast(std::i16::MAX as f32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return float3::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as f32), Self::broadcast(std::u16::MAX as f32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return float3::to_int(self.clamp(Self::broadcast(std::i32::MIN as f32), Self::broadcast(std::i32::MAX as f32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return float3::to_uint(self.clamp(Self::broadcast(std::u32::MIN as f32), Self::broadcast(std::u32::MAX as f32)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return float3::to_long(self.clamp(Self::broadcast(std::i64::MIN as f32), Self::broadcast(std::i64::MAX as f32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return float3::to_ulong(self.clamp(Self::broadcast(std::u64::MIN as f32), Self::broadcast(std::u64::MAX as f32)));
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
  type FloatScalar = f32;

  const SIGN_MASK: i32 = std::i32::MAX;
}

impl Geometry for float3 {
}

impl float3 {
  #[inline(always)]
  pub fn bitcast<T>(x: T) -> float3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline(always)]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> float2 {
    return float2(self.2, 0.0);
  }

  #[inline(always)]
  pub fn odd(self) -> float2 {
    return float2(self.1, 0.0);
  }

  #[inline(always)]
  pub fn even(self) -> float2 {
    return float2(self.0, self.2);
  }
}
