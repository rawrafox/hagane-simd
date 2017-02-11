extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn sign() {
  assert_eq!(simd::sign(float2(10.0, -3.0)), float2(1.0, -1.0));
  assert_eq!(simd::sign(float3(10.0, -3.0, -9.5)), float3(1.0, -1.0, -1.0));
  assert_eq!(simd::sign(float4(10.0, -3.0, -9.5, 0.0)), float4(1.0, -1.0, -1.0, 0.0));

  assert_eq!(simd::sign(double2(10.0, -3.0)), double2(1.0, -1.0));
  assert_eq!(simd::sign(double3(10.0, -3.0, -9.5)), double3(1.0, -1.0, -1.0));
  assert_eq!(simd::sign(double4(10.0, -3.0, -9.5, 0.0)), double4(1.0, -1.0, -1.0, 0.0));
}

#[test]
fn mix() {
  assert_eq!(simd::mix(float2(0.5, 1.0), float2(10.0, -2.0), float2(11.0, -3.0)), float2(10.5, -3.0));
  assert_eq!(simd::mix(float3(0.5, 1.0, 0.0), float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(10.5, -3.0, -9.5));
  assert_eq!(simd::mix(float4(0.5, 1.0, 0.0, 0.3), float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(10.5, -3.0, -9.5, 0.7));

  assert_eq!(simd::mix(double2(0.5, 1.0), double2(10.0, -2.0), double2(11.0, -3.0)), double2(10.5, -3.0));
  assert_eq!(simd::mix(double3(0.5, 1.0, 0.0), double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(10.5, -3.0, -9.5));
  assert_eq!(simd::mix(double4(0.5, 1.0, 0.0, 0.3), double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(10.5, -3.0, -9.5, 0.7));
}

#[test]
fn recip() {
  assert_eq!(simd::recip(float2(10.0, -2.0)), float2(0.1, -0.5));
  assert_eq!(simd::recip(float3(10.0, -2.0, -1.0)), float3(0.1, -0.5, -1.0));
  assert_eq!(simd::recip(float4(10.0, -2.0, -1.0, 1.0)), float4(0.1, -0.5, -1.0, 1.0));

  assert_eq!(simd::recip(double2(10.0, -2.0)), double2(0.1, -0.5));
  assert_eq!(simd::recip(double3(10.0, -2.0, -1.0)), double3(0.1, -0.5, -1.0));
  assert_eq!(simd::recip(double4(10.0, -2.0, -1.0, 1.0)), double4(0.1, -0.5, -1.0, 1.0));
}

#[test]
fn rsqrt() {
  assert_eq!(simd::rsqrt(float2(100.0, 4.0)), float2(0.1, 0.5));
  assert_eq!(simd::rsqrt(float3(100.0, 4.0, 1.0)), float3(0.1, 0.5, 1.0));
  assert_eq!(simd::rsqrt(float4(100.0, 4.0, 1.0, 9.0)), float4(0.1, 0.5, 1.0, 1.0 / 3.0));

  assert_eq!(simd::rsqrt(double2(100.0, 4.0)), double2(0.1, 0.5));
  assert_eq!(simd::rsqrt(double3(100.0, 4.0, 1.0)), double3(0.1, 0.5, 1.0));
  assert_eq!(simd::rsqrt(double4(100.0, 4.0, 1.0, 9.0)), double4(0.1, 0.5, 1.0, 1.0 / 3.0));
}

#[test]
fn fract() {
  assert_eq!(simd::fract(float2(1.5, 2.75)), float2(0.5, 0.75));
  assert_eq!(simd::fract(float3(1.5, 2.75, 3.8125)), float3(0.5, 0.75, 0.8125));
  assert_eq!(simd::fract(float4(1.5, 2.75, 3.8125, 1.25)), float4(0.5, 0.75, 0.8125, 0.25));

  assert_eq!(simd::fract(double2(1.5, 2.75)), double2(0.5, 0.75));
  assert_eq!(simd::fract(double3(1.5, 2.75, 3.8125)), double3(0.5, 0.75, 0.8125));
  assert_eq!(simd::fract(double4(1.5, 2.75, 3.8125, 1.25)), double4(0.5, 0.75, 0.8125, 0.25));
}

#[test]
fn step() {
  assert_eq!(simd::step(float2(0.0, 2.0), float2(1.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(simd::step(float3(0.0, 2.0, 4.0), float3(1.0, 2.0, 3.0)), float3(0.0, 1.0, 1.0));
  assert_eq!(simd::step(float4(0.0, 2.0, 4.0, 8.0), float4(1.0, 2.0, 3.0, 4.0)), float4(0.0, 1.0, 1.0, 1.0));

  assert_eq!(simd::step(double2(0.0, 2.0), double2(1.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(simd::step(double3(0.0, 2.0, 4.0), double3(1.0, 2.0, 3.0)), double3(0.0, 1.0, 1.0));
  assert_eq!(simd::step(double4(0.0, 2.0, 4.0, 8.0), double4(1.0, 2.0, 3.0, 4.0)), double4(0.0, 1.0, 1.0, 1.0));
}

#[test]
fn smoothstep() {
  assert_eq!(simd::smoothstep(float2(-1.0, 2.0), float2(-1.0, -2.0), float2(1.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(simd::smoothstep(float3(-1.0, 2.0, 0.0), float3(-1.0, -2.0, -3.0), float3(1.0, 2.0, 3.0)), float3(0.0, 1.0, 0.5));
  assert_eq!(simd::smoothstep(float4(-1.0, 2.0, 0.0, 2.0), float4(-1.0, -2.0, -3.0, -4.0), float4(1.0, 2.0, 3.0, 4.0)), float4(0.0, 1.0, 0.5, 0.84375));

  assert_eq!(simd::smoothstep(double2(-1.0, 2.0), double2(-1.0, -2.0), double2(1.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(simd::smoothstep(double3(-1.0, 2.0, 0.0), double3(-1.0, -2.0, -3.0), double3(1.0, 2.0, 3.0)), double3(0.0, 1.0, 0.5));
  assert_eq!(simd::smoothstep(double4(-1.0, 2.0, 0.0, 2.0), double4(-1.0, -2.0, -3.0, -4.0), double4(1.0, 2.0, 3.0, 4.0)), double4(0.0, 1.0, 0.5, 0.84375));
}
