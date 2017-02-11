macro_rules! declare_vector {
  ($name2:ident, $name3:ident, $name4:ident, $scalar:ident, $kind:ident) => (
    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name2(pub $scalar, pub $scalar);

    impl_vector!($name2, $scalar, $kind);

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name3(pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name3, $scalar, $kind);

    #[repr(C)]
    #[repr(simd)]
    #[derive(Copy, Clone, Debug)]
    pub struct $name4(pub $scalar, pub $scalar, pub $scalar, pub $scalar);

    impl_vector!($name4, $scalar, $kind);
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
        return unsafe { $intrinsic(self, $vector::broadcast(other)) };
      }
    }

    impl $trait_name<$vector> for $scalar {
      type Output = $vector;

      #[inline]
      fn $fn_name(self, other: $vector) -> $vector {
        return unsafe { $intrinsic($vector::broadcast(self), other) };
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
