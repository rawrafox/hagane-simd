pub trait Dot<RHS=Self> {
  type DotProduct;

  fn dot(self, rhs: RHS) -> Self::DotProduct;
}

#[inline(always)]
pub fn dot<RHS, T: Dot<RHS>>(x: T, y: RHS) -> T::DotProduct {
  return x.dot(y);
}
