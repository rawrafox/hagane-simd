use std;
use ::*;

impl Vector for double8 {
  type Scalar = f64;
  type Boolean = long8;

  type CharVector = char8;
  type ShortVector = short8;
  type IntVector = int8;
  type LongVector = long8;

  type UCharVector = uchar8;
  type UShortVector = ushort8;
  type UIntVector = uint8;
  type ULongVector = ulong8;

  type FloatVector = float8;
  type DoubleVector = double8;

  #[inline(always)]
  fn abs(self) -> Self {
    let x: Self::Boolean = broadcast(std::i64::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double8(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2), self.3.max(other.3), self.4.max(other.4), self.5.max(other.5), self.6.max(other.6), self.7.max(other.7));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double8(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2), self.3.min(other.3), self.4.min(other.4), self.5.min(other.5), self.6.min(other.6), self.7.min(other.7));
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return reduce_min(min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return reduce_max(max(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return double8::to_char(clamp(self, broadcast(std::i8::MIN as f64), broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return double8::to_uchar(clamp(self, broadcast(std::u8::MIN as f64), broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return double8::to_short(clamp(self, broadcast(std::i16::MIN as f64), broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return double8::to_ushort(clamp(self, broadcast(std::u16::MIN as f64), broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return double8::to_int(clamp(self, broadcast(std::i32::MIN as f64), broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return double8::to_uint(clamp(self, broadcast(std::u32::MIN as f64), broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return double8::to_long(clamp(self, broadcast(std::i64::MIN as f64), broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return double8::to_ulong(clamp(self, broadcast(std::u64::MIN as f64), broadcast(std::u64::MAX as f64)));
  }
}

impl Dot<double8> for double8 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for double8 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = broadcast(std::i64::MAX);

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double8(self.0.sqrt(), self.1.sqrt(), self.2.sqrt(), self.3.sqrt(), self.4.sqrt(), self.5.sqrt(), self.6.sqrt(), self.7.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double8(self.0.fract(), self.1.fract(), self.2.fract(), self.3.fract(), self.4.fract(), self.5.fract(), self.6.fract(), self.7.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double8(self.0.ceil(), self.1.ceil(), self.2.ceil(), self.3.ceil(), self.4.ceil(), self.5.ceil(), self.6.ceil(), self.7.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double8(self.0.floor(), self.1.floor(), self.2.floor(), self.3.floor(), self.4.floor(), self.5.floor(), self.6.floor(), self.7.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double8(self.0.trunc(), self.1.trunc(), self.2.trunc(), self.3.trunc(), self.4.trunc(), self.5.trunc(), self.6.trunc(), self.7.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double8(self.0.sin(), self.1.sin(), self.2.sin(), self.3.sin(), self.4.sin(), self.5.sin(), self.6.sin(), self.7.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double8(self.0.cos(), self.1.cos(), self.2.cos(), self.3.cos(), self.4.cos(), self.5.cos(), self.6.cos(), self.7.cos());
  }
}

impl Geometry for double8 {
  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }
}

impl double8 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> double4 {
    return double4(self.0, self.1, self.2, self.3);
  }

  #[inline]
  pub fn hi(self) -> double4 {
    return double4(self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn odd(self) -> double4 {
    return double4(self.1, self.3, self.5, self.7);
  }

  #[inline]
  pub fn even(self) -> double4 {
    return double4(self.0, self.2, self.4, self.6);
  }
}
