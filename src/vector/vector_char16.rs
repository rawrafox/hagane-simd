use std;
use ::*;

impl Vector for char16 {
  type Scalar = i8;
  type Boolean = char16;

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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return char16(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return char16(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7), f(self.8, other.8), f(self.9, other.9), f(self.10, other.10), f(self.11, other.11), f(self.12, other.12), f(self.13, other.13), f(self.14, other.14), f(self.15, other.15));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

    return (self ^ mask) - mask;
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
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return char16::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return char16::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return char16::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return char16::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return char16::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return char16::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return char16::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<char16> for char16 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char16 {
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
    return self.reduce_and() & std::i8::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i8::MIN != 0;
  }
}

impl Select<char16> for char16 {
  #[inline(always)]
  fn select(self, a: char16, b: char16) -> char16 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char16, b: char16) -> char16 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar16> for char16 {
  #[inline(always)]
  fn select(self, a: uchar16, b: uchar16) -> uchar16 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar16, b: uchar16) -> uchar16 {
    return uchar16::bitcast(self.bitselect(char16::bitcast(a), char16::bitcast(b)));
  }
}

impl char16 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> char8 {
    return char8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn hi(self) -> char8 {
    return char8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline]
  pub fn odd(self) -> char8 {
    return char8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline]
  pub fn even(self) -> char8 {
    return char8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}
