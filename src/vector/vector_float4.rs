use std;
use ::*;

#[cfg(target_feature = "sse")]
extern "platform-intrinsic" {
  fn x86_mm_sqrt_ps(x: float4) -> float4;
  fn x86_mm_rsqrt_ps(x: float4) -> float4;
}

#[cfg(target_feature = "sse4.1")]
extern "platform-intrinsic" {
  fn x86_mm_dp_ps(x: float4, y: float4, z: i32) -> float4;
}

#[cfg(target_feature = "avx2")] // TODO: Compiler bug, should actually be "fma"
extern {
  #[link_name = "llvm.fma.v4f32"]
  fn fma_v4f32(a: float4, b: float4, c: float4) -> float4;
}

impl Vector for float4 {
  type Scalar = f32;
  type Boolean = int4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return float4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return float4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.3, f(self.2, f(self.1, self.0)));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let x = Self::Boolean::broadcast(std::i32::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[cfg(target_feature = "avx2")] // TODO: Compiler bug, should actually be "fma"
  #[inline(always)]
  fn add_mul(self, a: Self, b: Self) -> Self {
    return unsafe { fma_v4f32(a, b, self) };
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return float4::to_char(self.clamp(Self::broadcast(std::i8::MIN as f32), Self::broadcast(std::i8::MAX as f32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return float4::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as f32), Self::broadcast(std::u8::MAX as f32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return float4::to_short(self.clamp(Self::broadcast(std::i16::MIN as f32), Self::broadcast(std::i16::MAX as f32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return float4::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as f32), Self::broadcast(std::u16::MAX as f32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return float4::to_int(self.clamp(Self::broadcast(std::i32::MIN as f32), Self::broadcast(std::i32::MAX as f32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return float4::to_uint(self.clamp(Self::broadcast(std::u32::MIN as f32), Self::broadcast(std::u32::MAX as f32)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return float4::to_long(self.clamp(Self::broadcast(std::i64::MIN as f32), Self::broadcast(std::i64::MAX as f32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return float4::to_ulong(self.clamp(Self::broadcast(std::u64::MIN as f32), Self::broadcast(std::u64::MAX as f32)));
  }
}

impl Dot<float4> for float4 {
  type DotProduct = f32;

  #[cfg(target_feature = "sse4.1")]
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return unsafe { x86_mm_dp_ps(self, other, 0xF1).0 };
  }

  #[cfg(not(target_feature = "sse4.1"))]
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for float4 {
  type FloatScalar = f32;

  const SIGN_MASK: i32 = std::i32::MAX;

  #[cfg(target_feature = "sse")]
  #[inline(always)]
  fn sqrt(self) -> Self {
    return unsafe { x86_mm_sqrt_ps(self) };
  }

  #[cfg(target_feature = "sse")]
  #[inline(always)]
  fn rsqrt(self) -> Self {
    let r = unsafe { x86_mm_rsqrt_ps(self) };

    return r * (1.5 - 0.5 * self.eq(Self::broadcast(0.0)).bitselect(self, Self::broadcast(-std::f32::INFINITY)) * r * r);
  }
}

impl Geometry for float4 {
}

impl float4 {
  #[inline(always)]
  pub fn lo(self) -> float2 {
    return float2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> float2 {
    return float2(self.2, self.3);
  }

  #[inline(always)]
  pub fn odd(self) -> float2 {
    return float2(self.1, self.3);
  }

  #[inline(always)]
  pub fn even(self) -> float2 {
    return float2(self.0, self.2);
  }
}
