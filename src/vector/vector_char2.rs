use std;
use ::*;

impl Vector for char2 {
  type Scalar = i8;
  type Boolean = char2;

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

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return char2(f(self.0), f(self.1));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return char2(f(self.0, other.0), f(self.1, other.1));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return char2::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return char2::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return char2::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return char2::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return char2::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return char2::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return char2::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<char2> for char2 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char2 {
  type IntegerScalar = i8;

  const SIGN_MASK: i8 = std::i8::MIN;

  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1
  }
}

impl Select<char2> for char2 {
  #[inline(always)]
  fn select(self, a: char2, b: char2) -> char2 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char2, b: char2) -> char2 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar2> for char2 {
  #[inline(always)]
  fn select(self, a: uchar2, b: uchar2) -> uchar2 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar2, b: uchar2) -> uchar2 {
    return uchar2::bitcast(self.bitselect(char2::bitcast(a), char2::bitcast(b)));
  }
}

impl char2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> i8 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i8 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i8 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i8 {
    return self.0;
  }
}
