#[macro_use] extern crate quickcheck;

extern crate hagane_simd;

#[macro_use] mod macros;

use hagane_simd::*;

#[test]
fn test_normalize() {
  let n = float4::broadcast(0.0).normalize();

  assert!(n.0.is_nan());
  assert!(n.1.is_nan());
  assert!(n.2.is_nan());
  assert!(n.3.is_nan());
}

quickcheck! {
  fn length_is_one(x: f32, y: f32, z: f32, w: f32) -> bool {
    let v = float4(x, y, z, w);

    if v == float4::broadcast(0.0) {
      return true;
    }

    return (float4(x, y, z, w).normalize().length_squared() - 1.0).abs() <= (2.0 * std::f32::EPSILON);
  }
}
