extern crate hagane_simd;

#[macro_use] mod macros;

use hagane_simd::*;

#[test]
fn test_cross() {
  assert_near_f32!(float3(0.0, 1.0, 0.0).cross(float3(0.0, 0.0, 1.0)), float3(1.0, 0.0, 0.0), 1);
  assert_near_f32!(float3(3.0, 2.0, -2.0).cross(float3(1.0, 0.0, -5.0)), float3(-10.0, 13.0, -2.0), 1);
}
