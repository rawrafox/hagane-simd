extern crate hagane_simd;

use hagane_simd::*;

#[test]
fn test_constructors() {
  assert_eq!(float2x3::from_rows(float2(0.0, 1.0), float2(2.0, 3.0), float2(4.0, 5.0)), float2x3::from_columns(float3(0.0, 2.0, 4.0), float3(1.0, 3.0, 5.0)));
}

#[test]
fn test_mul() {
  assert_eq!(float2x2(float2(1.0, 3.0), float2(2.0, 4.0)) * float2(2.0, 3.0), float2(8.0, 18.0));
  assert_eq!(float2x3::from_rows(float2(1.0, 2.0), float2(3.0, 4.0), float2(5.0, 6.0)) * float2(2.0, 3.0), float3(8.0, 18.0, 28.0));
}
