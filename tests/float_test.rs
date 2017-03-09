extern crate hagane_simd;

use hagane_simd::*;

#[macro_use] mod macros;

#[test]
fn test_sign() {
  assert_eq!(sign(float2(10.0, -3.0)), float2(1.0, -1.0));
  assert_eq!(sign(float3(10.0, -3.0, -9.5)), float3(1.0, -1.0, -1.0));
  assert_eq!(sign(float4(10.0, -3.0, -9.5, 0.0)), float4(1.0, -1.0, -1.0, 0.0));

  assert_eq!(sign(double2(10.0, -3.0)), double2(1.0, -1.0));
  assert_eq!(sign(double3(10.0, -3.0, -9.5)), double3(1.0, -1.0, -1.0));
  assert_eq!(sign(double4(10.0, -3.0, -9.5, 0.0)), double4(1.0, -1.0, -1.0, 0.0));
}

#[test]
fn test_mix() {
  assert_eq!(mix(float2(0.5, 1.0), float2(10.0, -2.0), float2(11.0, -3.0)), float2(10.5, -3.0));
  assert_eq!(mix(float3(0.5, 1.0, 0.0), float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(10.5, -3.0, -9.5));
  assert_eq!(mix(float4(0.5, 1.0, 0.0, 0.3), float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(10.5, -3.0, -9.5, 0.7));

  assert_eq!(mix(double2(0.5, 1.0), double2(10.0, -2.0), double2(11.0, -3.0)), double2(10.5, -3.0));
  assert_eq!(mix(double3(0.5, 1.0, 0.0), double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(10.5, -3.0, -9.5));
  assert_eq!(mix(double4(0.5, 1.0, 0.0, 0.3), double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(10.5, -3.0, -9.5, 0.7));
}

#[test]
fn test_recip() {
  assert_near_f32!(recip(float2(10.0, -2.0)), float2(0.1, -0.5), 1);
  assert_near_f32!(recip(float3(10.0, -2.0, -1.0)), float3(0.1, -0.5, -1.0), 1);
  assert_near_f32!(recip(float4(10.0, -2.0, -1.0, 1.0)), float4(0.1, -0.5, -1.0, 1.0), 1);

  assert_eq!(recip(double2(10.0, -2.0)), double2(0.1, -0.5));
  assert_eq!(recip(double3(10.0, -2.0, -1.0)), double3(0.1, -0.5, -1.0));
  assert_eq!(recip(double4(10.0, -2.0, -1.0, 1.0)), double4(0.1, -0.5, -1.0, 1.0));
}

#[test]
fn test_rsqrt() {
  assert_near_f32!(rsqrt(float2(100.0, 4.0)), float2(0.1, 0.5), 1);
  assert_near_f32!(rsqrt(float3(100.0, 4.0, 1.0)), float3(0.1, 0.5, 1.0), 1);
  assert_near_f32!(rsqrt(float4(100.0, 4.0, 1.0, 9.0)), float4(0.1, 0.5, 1.0, 1.0 / 3.0), 1);

  assert_eq!(rsqrt(double2(100.0, 4.0)), double2(0.1, 0.5));
  assert_eq!(rsqrt(double3(100.0, 4.0, 1.0)), double3(0.1, 0.5, 1.0));
  assert_eq!(rsqrt(double4(100.0, 4.0, 1.0, 9.0)), double4(0.1, 0.5, 1.0, 1.0 / 3.0));
}

#[test]
fn test_fract() {
  assert_eq!(fract(float2(1.5, 2.75)), float2(0.5, 0.75));
  assert_eq!(fract(float3(1.5, 2.75, 3.8125)), float3(0.5, 0.75, 0.8125));
  assert_eq!(fract(float4(1.5, 2.75, 3.8125, 1.25)), float4(0.5, 0.75, 0.8125, 0.25));

  assert_eq!(fract(double2(1.5, 2.75)), double2(0.5, 0.75));
  assert_eq!(fract(double3(1.5, 2.75, 3.8125)), double3(0.5, 0.75, 0.8125));
  assert_eq!(fract(double4(1.5, 2.75, 3.8125, 1.25)), double4(0.5, 0.75, 0.8125, 0.25));
}

#[test]
fn test_step() {
  assert_eq!(step(float2(0.0, 2.0), float2(1.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(step(float3(0.0, 2.0, 4.0), float3(1.0, 2.0, 3.0)), float3(0.0, 1.0, 1.0));
  assert_eq!(step(float4(0.0, 2.0, 4.0, 8.0), float4(1.0, 2.0, 3.0, 4.0)), float4(0.0, 1.0, 1.0, 1.0));

  assert_eq!(step(double2(0.0, 2.0), double2(1.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(step(double3(0.0, 2.0, 4.0), double3(1.0, 2.0, 3.0)), double3(0.0, 1.0, 1.0));
  assert_eq!(step(double4(0.0, 2.0, 4.0, 8.0), double4(1.0, 2.0, 3.0, 4.0)), double4(0.0, 1.0, 1.0, 1.0));
}

#[test]
fn test_smoothstep() {
  assert_eq!(smoothstep(float2(-1.0, 2.0), float2(-1.0, -2.0), float2(1.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(smoothstep(float3(-1.0, 2.0, 0.0), float3(-1.0, -2.0, -3.0), float3(1.0, 2.0, 3.0)), float3(0.0, 1.0, 0.5));
  assert_eq!(smoothstep(float4(-1.0, 2.0, 0.0, 2.0), float4(-1.0, -2.0, -3.0, -4.0), float4(1.0, 2.0, 3.0, 4.0)), float4(0.0, 1.0, 0.5, 0.84375));

  assert_eq!(smoothstep(double2(-1.0, 2.0), double2(-1.0, -2.0), double2(1.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(smoothstep(double3(-1.0, 2.0, 0.0), double3(-1.0, -2.0, -3.0), double3(1.0, 2.0, 3.0)), double3(0.0, 1.0, 0.5));
  assert_eq!(smoothstep(double4(-1.0, 2.0, 0.0, 2.0), double4(-1.0, -2.0, -3.0, -4.0), double4(1.0, 2.0, 3.0, 4.0)), double4(0.0, 1.0, 0.5, 0.84375));
}

#[test]
fn test_copysign() {
  assert_eq!(copysign(float2(10.0, -3.0), float2(-2.0, 4.0)), float2(-10.0, 3.0));
  assert_eq!(copysign(float3(10.0, -3.0, 2.0), float3(-2.0, 4.0, 3.0)), float3(-10.0, 3.0, 2.0));
  assert_eq!(copysign(float4(10.0, -3.0, 2.0, -3.0), float4(-2.0, 4.0, 3.0, -4.0)), float4(-10.0, 3.0, 2.0, -3.0));
}

#[test]
fn test_sqrt() {
  assert_eq!(sqrt(float2(100.0, 4.0)), float2(10.0, 2.0));
  assert_eq!(sqrt(float3(100.0, 4.0, 1.0)), float3(10.0, 2.0, 1.0));
  assert_eq!(sqrt(float4(100.0, 4.0, 1.0, 9.0)), float4(10.0, 2.0, 1.0, 3.0));
}

#[test]
fn test_ceil() {
  assert_eq!(ceil(float2(100.1, -4.4)), float2(101.0, -4.0));
  assert_eq!(ceil(float3(100.1, -4.4, 1.9)), float3(101.0, -4.0, 2.0));
  assert_eq!(ceil(float4(100.1, -4.4, 1.9, 9.0)), float4(101.0, -4.0, 2.0, 9.0));
}

#[test]
fn test_floor() {
  assert_eq!(floor(float2(100.1, -4.4)), float2(100.0, -5.0));
  assert_eq!(floor(float3(100.1, -4.4, 1.9)), float3(100.0, -5.0, 1.0));
  assert_eq!(floor(float4(100.1, -4.4, 1.9, 9.0)), float4(100.0, -5.0, 1.0, 9.0));
}

#[test]
fn test_trunc() {
  assert_eq!(trunc(float2(100.1, -4.4)), float2(100.0, -4.0));
  assert_eq!(trunc(float3(100.1, -4.4, 1.9)), float3(100.0, -4.0, 1.0));
  assert_eq!(trunc(float4(100.1, -4.4, 1.9, 9.0)), float4(100.0, -4.0, 1.0, 9.0));
}

#[test]
fn test_dot() {
  assert_near_f32_scalar!(dot(float2(10.0, -2.0), float2(0.1, 0.5)), 0.0, 1);
  assert_near_f32_scalar!(dot(float3(10.0, -2.0, 3.0), float3(0.1, 0.5, 1.0)), 3.0, 1);
  assert_near_f32_scalar!(dot(float4(10.0, -2.0, 3.0, 0.0), float4(0.1, 0.5, 1.0, -4.0)), 3.0, 1);
}