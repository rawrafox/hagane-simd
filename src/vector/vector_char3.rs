use std;
use ::*;

impl Vector for char3 {
  type Scalar = i8;
  type Boolean = char3;

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
    return char3(f(self.0), f(self.1), f(self.2));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return char3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(reduce_max(self.lo()), self.2);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return char3::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short(self) -> short3 {
    return short3(self.0 as i16, self.1 as i16, self.2 as i16);
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return char3::to_short(self);
  }

  #[inline(always)]
  fn to_ushort(self) -> ushort3 {
    return ushort3(self.0 as u16, self.1 as u16, self.2 as u16);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return char3::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int(self) -> int3 {
    return int3(self.0 as i32, self.1 as i32, self.2 as i32);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return char3::to_int(self);
  }

  #[inline(always)]
  fn to_uint(self) -> uint3 {
    return uint3(self.0 as u32, self.1 as u32, self.2 as u32);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return char3::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_float(self) -> float3 {
    return float3(self.0 as f32, self.1 as f32, self.2 as f32);
  }

  #[inline(always)]
  fn to_long(self) -> long3 {
    return long3(self.0 as i64, self.1 as i64, self.2 as i64);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return char3::to_long(self);
  }

  #[inline(always)]
  fn to_ulong(self) -> ulong3 {
    return ulong3(self.0 as u64, self.1 as u64, self.2 as u64);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return char3::to_ulong(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_double(self) -> double3 {
    return double3(self.0 as f64, self.1 as f64, self.2 as f64);
  }
}

impl Dot<char3> for char3 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char3 {
  type IntegerScalar = i8;

  const SIGN_MASK: i8 = std::i8::MIN;

  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1 & self.2
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1 | self.2
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1 ^ self.2
  }
}

impl Select<char3> for char3 {
  #[inline(always)]
  fn select(self, a: char3, b: char3) -> char3 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: char3, b: char3) -> char3 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar3> for char3 {
  #[inline(always)]
  fn select(self, a: uchar3, b: uchar3) -> uchar3 {
    return (self >> 7).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: uchar3, b: uchar3) -> uchar3 {
    return uchar3::bitcast(self.bitselect(char3::bitcast(a), char3::bitcast(b)));
  }
}

impl char3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> char3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> char2 {
    return char2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> char2 {
    return char2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> char2 {
    return char2(self.0, self.2);
  }
}
