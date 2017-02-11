macro_rules! declare_vector {
  ($name2:ident, $name3:ident, $name4:ident, $name8:ident, $name16:ident, $scalar:ident, $kind:ident) => (
    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name2(pub $scalar, pub $scalar);

    impl_vector!($name2, $scalar, $kind);

    impl Broadcast<$name2> for $scalar  {
      fn broadcast(self) -> $name2 {
        return $name2(self, self);
      }
    }

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name3(pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name3, $scalar, $kind);

    impl Broadcast<$name3> for $scalar  {
      fn broadcast(self) -> $name3 {
        return $name3(self, self, self);
      }
    }

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name4(pub $scalar, pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name4, $scalar, $kind);

    impl Broadcast<$name4> for $scalar  {
      fn broadcast(self) -> $name4 {
        return $name4(self, self, self, self);
      }
    }

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name8(pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name8, $scalar, $kind);

    impl Broadcast<$name8> for $scalar  {
      fn broadcast(self) -> $name8 {
        return $name8(self, self, self, self, self, self, self, self);
      }
    }

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name16(pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name16, $scalar, $kind);

    impl Broadcast<$name16> for $scalar  {
      fn broadcast(self) -> $name16 {
        return $name16(self, self, self, self, self, self, self, self, self, self, self, self, self, self, self, self);
      }
    }
  );
}

macro_rules! impl_trait {
  ($vector:ident, $scalar:ident, $intrinsic:ident, $trait_name:ident, $fn_name:ident) => {
    impl $trait_name<$vector> for $vector {
      type Output = Self;

      #[inline]
      fn $fn_name(self, other: Self) -> Self {
        return unsafe { $intrinsic(self, other) };
      }
    }

    impl $trait_name<$scalar> for $vector {
      type Output = Self;

      #[inline]
      fn $fn_name(self, other: $scalar) -> Self {
        return unsafe { $intrinsic(self, broadcast(other)) };
      }
    }

    impl $trait_name<$vector> for $scalar {
      type Output = $vector;

      #[inline]
      fn $fn_name(self, other: $vector) -> $vector {
        return unsafe { $intrinsic(broadcast(self), other) };
      }
    }
  }
}

macro_rules! impl_vector {
  ($vector:ident, $scalar:ident, integer) => {
    impl_vector!($vector, $scalar, float);

    impl_trait!($vector, $scalar, simd_and, BitAnd, bitand);
    impl_trait!($vector, $scalar, simd_or, BitOr, bitor);
    impl_trait!($vector, $scalar, simd_xor, BitXor, bitxor);

    impl_trait!($vector, $scalar, simd_shl, Shl, shl);
    impl_trait!($vector, $scalar, simd_shr, Shr, shr);

  };
  ($vector:ident, $scalar:ident, signed) => {
    impl_vector!($vector, $scalar, integer);

    impl std::ops::Not for $vector {
      type Output = Self;

      #[inline]
      fn not(self) -> Self {
        return self ^ -1;
      }
    }
  };
  ($vector:ident, $scalar:ident, unsigned) => {
    impl_vector!($vector, $scalar, integer);

    impl std::ops::Not for $vector {
      type Output = Self;

      #[inline]
      fn not(self) -> Self {
        return self ^ std::$scalar::MAX;
      }
    }
  };
  ($vector:ident, $scalar:ident, float) => {
    impl_trait!($vector, $scalar, simd_add, Add, add);
    impl_trait!($vector, $scalar, simd_sub, Sub, sub);
    impl_trait!($vector, $scalar, simd_mul, Mul, mul);
    impl_trait!($vector, $scalar, simd_div, Div, div);

    impl PartialEq for $vector {
      #[inline]
      fn eq(&self, other: &Self) -> bool {
        return eq(*self, *other).all();
      }

      #[inline]
      fn ne(&self, other: &Self) -> bool {
        return ne(*self, *other).all();
      }
    }
  }
}

macro_rules! declare_matrix {
  ($name2:ident, $name3:ident, $name4:ident, $vector:ty) => (
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name2(pub $vector, pub $vector);

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name3(pub $vector, pub $vector, pub $vector);

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name4(pub $vector, pub $vector, pub $vector, pub $vector);
  );
}
