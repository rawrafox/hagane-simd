use std;
use ::*;

impl Vector for ushort8 {
  type Scalar = u16;
  type Boolean = short8;

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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return ushort8(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return ushort8(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
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
    return ushort8::to_char(self.min(Self::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return ushort8::to_uchar(self.min(Self::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return ushort8::to_short(self.min(Self::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return ushort8::to_int(self.min(Self::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return ushort8::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return ushort8::to_long(self.min(Self::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return ushort8::to_ulong(self);
  }
}

impl Dot<ushort8> for ushort8 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort8 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return (self.lo() & self.hi()).reduce_and();
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return (self.lo() | self.hi()).reduce_or();
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return (self.lo() ^ self.hi()).reduce_xor();
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000 != 0;
  }
}

impl ushort8 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort8 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> ushort4 {
    return ushort4(self.0, self.1, self.2, self.3);
  }

  #[inline]
  pub fn hi(self) -> ushort4 {
    return ushort4(self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn odd(self) -> ushort4 {
    return ushort4(self.1, self.3, self.5, self.7);
  }

  #[inline]
  pub fn even(self) -> ushort4 {
    return ushort4(self.0, self.2, self.4, self.6);
  }
}
