extern crate hagane_simd;

#[macro_use] mod macros;

use hagane_simd::*;

#[test]
fn test_constructors() {
  assert_eq!(float2x3::from_rows(float2(0.0, 1.0), float2(2.0, 3.0), float2(4.0, 5.0)), float2x3::from_columns(float3(0.0, 2.0, 4.0), float3(1.0, 3.0, 5.0)));
}

#[test]
fn test_mul() {
  assert_eq!(float2x2(float2(1.0, 3.0), float2(2.0, 4.0)) * float2(2.0, 3.0), float2(8.0, 18.0));
  assert_eq!(float2x3::from_rows(float2(1.0, 2.0), float2(3.0, 4.0), float2(5.0, 6.0)) * float2(2.0, 3.0), float3(8.0, 18.0, 28.0));
  
  let a = float4x4::from_rows(float4(1.0, 2.0, 3.0, 4.0), float4(5.0, 6.0, 7.0, 8.0), float4(9.0, 10.0, 11.0, 12.0), float4(13.0, 14.0, 15.0, 16.0));
  assert_eq!(a * a, float4x4::from_rows(float4(90.0, 100.0, 110.0, 120.0), float4(202.0, 228.0, 254.0, 280.0), float4(314.0, 356.0, 398.0, 440.0), float4(426.0, 484.0, 542.0, 600.0)));
}

#[test]
fn look_at() {
  let look_at = float4x4::look_at(float3(0.0, 0.0, -5.0), float3(0.0, 0.0, 0.0), float3(0.0, 1.0, 0.0));
  let translation = float4x4::from_translation(0.0, 0.0, 5.0);

  assert_near_f32!(look_at.0, translation.0, 1);
  assert_near_f32!(look_at.1, translation.1, 1);
  assert_near_f32!(look_at.2, translation.2, 1);
  assert_near_f32!(look_at.3, translation.3, 1);
}