use std;
use ::*;

impl Vector for double16 {
  type Scalar = f64;
  type Boolean = long16;

  type CharVector = char16;
  type ShortVector = short16;
  type IntVector = int16;
  type LongVector = long16;

  type UCharVector = uchar16;
  type UShortVector = ushort16;
  type UIntVector = uint16;
  type ULongVector = ulong16;

  type FloatVector = float16;
  type DoubleVector = double16;

  #[inline(always)]
  fn abs(self) -> Self {
    let x: Self::Boolean = broadcast(std::i64::MAX);

    return x.bitselect(Self::from(0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double16(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2), self.3.max(other.3), self.4.max(other.4), self.5.max(other.5), self.6.max(other.6), self.7.max(other.7), self.8.max(other.8), self.9.max(other.9), self.10.max(other.10), self.11.max(other.11), self.12.max(other.12), self.13.max(other.13), self.14.max(other.14), self.15.max(other.15));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double16(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2), self.3.min(other.3), self.4.min(other.4), self.5.min(other.5), self.6.min(other.6), self.7.min(other.7), self.8.min(other.8), self.9.min(other.9), self.10.min(other.10), self.11.min(other.11), self.12.min(other.12), self.13.min(other.13), self.14.min(other.14), self.15.min(other.15));
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
  fn to_char_sat(self) -> char16 {
    return double16::to_char(clamp(self, broadcast(std::i8::MIN as f64), broadcast(std::i8::MAX as f64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return double16::to_uchar(clamp(self, broadcast(std::u8::MIN as f64), broadcast(std::u8::MAX as f64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return double16::to_short(clamp(self, broadcast(std::i16::MIN as f64), broadcast(std::i16::MAX as f64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return double16::to_ushort(clamp(self, broadcast(std::u16::MIN as f64), broadcast(std::u16::MAX as f64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return double16::to_int(clamp(self, broadcast(std::i32::MIN as f64), broadcast(std::i32::MAX as f64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return double16::to_uint(clamp(self, broadcast(std::u32::MIN as f64), broadcast(std::u32::MAX as f64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return double16::to_long(clamp(self, broadcast(std::i64::MIN as f64), broadcast(std::i64::MAX as f64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return double16::to_ulong(clamp(self, broadcast(std::u64::MIN as f64), broadcast(std::u64::MAX as f64)));
  }
}

impl Dot<double16> for double16 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Float for double16 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    let x: Self::Boolean = broadcast(std::i64::MAX);

    return x.bitselect(magnitude, self);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double16(self.0.sqrt(), self.1.sqrt(), self.2.sqrt(), self.3.sqrt(), self.4.sqrt(), self.5.sqrt(), self.6.sqrt(), self.7.sqrt(), self.8.sqrt(), self.9.sqrt(), self.10.sqrt(), self.11.sqrt(), self.12.sqrt(), self.13.sqrt(), self.14.sqrt(), self.15.sqrt());
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double16(self.0.fract(), self.1.fract(), self.2.fract(), self.3.fract(), self.4.fract(), self.5.fract(), self.6.fract(), self.7.fract(), self.8.fract(), self.9.fract(), self.10.fract(), self.11.fract(), self.12.fract(), self.13.fract(), self.14.fract(), self.15.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double16(self.0.ceil(), self.1.ceil(), self.2.ceil(), self.3.ceil(), self.4.ceil(), self.5.ceil(), self.6.ceil(), self.7.ceil(), self.8.ceil(), self.9.ceil(), self.10.ceil(), self.11.ceil(), self.12.ceil(), self.13.ceil(), self.14.ceil(), self.15.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double16(self.0.floor(), self.1.floor(), self.2.floor(), self.3.floor(), self.4.floor(), self.5.floor(), self.6.floor(), self.7.floor(), self.8.floor(), self.9.floor(), self.10.floor(), self.11.floor(), self.12.floor(), self.13.floor(), self.14.floor(), self.15.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double16(self.0.trunc(), self.1.trunc(), self.2.trunc(), self.3.trunc(), self.4.trunc(), self.5.trunc(), self.6.trunc(), self.7.trunc(), self.8.trunc(), self.9.trunc(), self.10.trunc(), self.11.trunc(), self.12.trunc(), self.13.trunc(), self.14.trunc(), self.15.trunc());
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double16(self.0.sin(), self.1.sin(), self.2.sin(), self.3.sin(), self.4.sin(), self.5.sin(), self.6.sin(), self.7.sin(), self.8.sin(), self.9.sin(), self.10.sin(), self.11.sin(), self.12.sin(), self.13.sin(), self.14.sin(), self.15.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double16(self.0.cos(), self.1.cos(), self.2.cos(), self.3.cos(), self.4.cos(), self.5.cos(), self.6.cos(), self.7.cos(), self.8.cos(), self.9.cos(), self.10.cos(), self.11.cos(), self.12.cos(), self.13.cos(), self.14.cos(), self.15.cos());
  }
}

impl Geometry for double16 {
  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }
}

impl double16 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> double8 {
    return double8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn hi(self) -> double8 {
    return double8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline]
  pub fn odd(self) -> double8 {
    return double8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline]
  pub fn even(self) -> double8 {
    return double8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
