extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn abs() {
  assert_eq!(float2::abs(float2(10.0, -3.0)), float2(10.0, 3.0));
  assert_eq!(float3::abs(float3(10.0, -3.0, -9.5)), float3(10.0, 3.0, 9.5));
  assert_eq!(float4::abs(float4(10.0, -3.0, -9.5, 0.0)), float4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(double2::abs(double2(10.0, -3.0)), double2(10.0, 3.0));
  assert_eq!(double3::abs(double3(10.0, -3.0, -9.5)), double3(10.0, 3.0, 9.5));
  assert_eq!(double4::abs(double4(10.0, -3.0, -9.5, 0.0)), double4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(int2::abs(int2(10, -3)), int2(10, 3));
  assert_eq!(int3::abs(int3(10, -3, -9)), int3(10, 3, 9));
  assert_eq!(int4::abs(int4(10, -3, -9, 0)), int4(10, 3, 9, 0));

  assert_eq!(long2::abs(long2(10, -3)), long2(10, 3));
  assert_eq!(long3::abs(long3(10, -3, -9)), long3(10, 3, 9));
  assert_eq!(long4::abs(long4(10, -3, -9, 0)), long4(10, 3, 9, 0));
}

#[test]
fn max() {
  assert_eq!(float2::max(float2(10.0, -2.0), float2(11.0, -3.0)), float2(11.0, -2.0));
  assert_eq!(float3::max(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(float4::max(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(double2::max(double2(10.0, -2.0), double2(11.0, -3.0)), double2(11.0, -2.0));
  assert_eq!(double3::max(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(double4::max(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(int2::max(int2(10, -2), int2(11, -3)), int2(11, -2));
  assert_eq!(int3::max(int3(10, -2, -9), int3(11, -3, 10)), int3(11, -2, 10));
  assert_eq!(int4::max(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(11, -2, 10, 1));

  assert_eq!(long2::max(long2(10, -2), long2(11, -3)), long2(11, -2));
  assert_eq!(long3::max(long3(10, -2, -9), long3(11, -3, 10)), long3(11, -2, 10));
  assert_eq!(long4::max(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(11, -2, 10, 1));
}

#[test]
fn min() {
  assert_eq!(float2::min(float2(10.0, -2.0), float2(11.0, -3.0)), float2(10.0, -3.0));
  assert_eq!(float3::min(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(10.0, -3.0, -9.5));
  assert_eq!(float4::min(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(double2::min(double2(10.0, -2.0), double2(11.0, -3.0)), double2(10.0, -3.0));
  assert_eq!(double3::min(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(10.0, -3.0, -9.5));
  assert_eq!(double4::min(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(int2::min(int2(10, -2), int2(11, -3)), int2(10, -3));
  assert_eq!(int3::min(int3(10, -2, -9), int3(11, -3, 10)), int3(10, -3, -9));
  assert_eq!(int4::min(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(10, -3, -9, 0));

  assert_eq!(long2::min(long2(10, -2), long2(11, -3)), long2(10, -3));
  assert_eq!(long3::min(long3(10, -2, -9), long3(11, -3, 10)), long3(10, -3, -9));
  assert_eq!(long4::min(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(10, -3, -9, 0));
}

#[test]
fn clamp() {
  assert_eq!(float2::clamp(float2(10.0, -2.0), float2(11.0, -3.0), float2(11.0, -1.0)), float2(11.0, -2.0));
  assert_eq!(float3::clamp(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), float3(11.0, -1.0, 11.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(float4::clamp(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), float4(11.0, -1.0, 10.0, 0.5)), float4(11.0, -2.0, 10.0, 0.5));

  assert_eq!(double2::clamp(double2(10.0, -2.0), double2(11.0, -3.0), double2(11.0, -1.0)), double2(11.0, -2.0));
  assert_eq!(double3::clamp(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0), double3(11.0, -1.0, 11.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(double4::clamp(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0), double4(11.0, -1.0, 10.0, 0.5)), double4(11.0, -2.0, 10.0, 0.5));
}

#[test]
fn sign() {
  assert_eq!(float2::sign(float2(10.0, -3.0)), float2(1.0, -1.0));
  assert_eq!(float3::sign(float3(10.0, -3.0, -9.5)), float3(1.0, -1.0, -1.0));
  assert_eq!(float4::sign(float4(10.0, -3.0, -9.5, 0.0)), float4(1.0, -1.0, -1.0, 0.0));

  assert_eq!(double2::sign(double2(10.0, -3.0)), double2(1.0, -1.0));
  assert_eq!(double3::sign(double3(10.0, -3.0, -9.5)), double3(1.0, -1.0, -1.0));
  assert_eq!(double4::sign(double4(10.0, -3.0, -9.5, 0.0)), double4(1.0, -1.0, -1.0, 0.0));
}

#[test]
fn mix() {
  assert_eq!(float2::mix(float2(10.0, -2.0), float2(11.0, -3.0), float2(0.5, 1.0)), float2(10.5, -3.0));
  assert_eq!(float3::mix(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), float3(0.5, 1.0, 0.0)), float3(10.5, -3.0, -9.5));
  assert_eq!(float4::mix(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), float4(0.5, 1.0, 0.0, 0.3)), float4(10.5, -3.0, -9.5, 0.7));

  assert_eq!(double2::mix(double2(10.0, -2.0), double2(11.0, -3.0), double2(0.5, 1.0)), double2(10.5, -3.0));
  assert_eq!(double3::mix(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0), double3(0.5, 1.0, 0.0)), double3(10.5, -3.0, -9.5));
  assert_eq!(double4::mix(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0), double4(0.5, 1.0, 0.0, 0.3)), double4(10.5, -3.0, -9.5, 0.7));
}

#[test]
fn recip() {
  assert_eq!(float2::recip(float2(10.0, -2.0)), float2(0.1, -0.5));
  assert_eq!(float3::recip(float3(10.0, -2.0, -1.0)), float3(0.1, -0.5, -1.0));
  assert_eq!(float4::recip(float4(10.0, -2.0, -1.0, 1.0)), float4(0.1, -0.5, -1.0, 1.0));

  assert_eq!(double2::recip(double2(10.0, -2.0)), double2(0.1, -0.5));
  assert_eq!(double3::recip(double3(10.0, -2.0, -1.0)), double3(0.1, -0.5, -1.0));
  assert_eq!(double4::recip(double4(10.0, -2.0, -1.0, 1.0)), double4(0.1, -0.5, -1.0, 1.0));
}

#[test]
fn rsqrt() {
  assert_eq!(float2::rsqrt(float2(100.0, 4.0)), float2(0.1, 0.5));
  assert_eq!(float3::rsqrt(float3(100.0, 4.0, 1.0)), float3(0.1, 0.5, 1.0));
  assert_eq!(float4::rsqrt(float4(100.0, 4.0, 1.0, 9.0)), float4(0.1, 0.5, 1.0, 1.0 / 3.0));

  assert_eq!(double2::rsqrt(double2(100.0, 4.0)), double2(0.1, 0.5));
  assert_eq!(double3::rsqrt(double3(100.0, 4.0, 1.0)), double3(0.1, 0.5, 1.0));
  assert_eq!(double4::rsqrt(double4(100.0, 4.0, 1.0, 9.0)), double4(0.1, 0.5, 1.0, 1.0 / 3.0));
}

#[test]
fn fract() {
  assert_eq!(float2::fract(float2(1.5, 2.75)), float2(0.5, 0.75));
  assert_eq!(float3::fract(float3(1.5, 2.75, 3.8125)), float3(0.5, 0.75, 0.8125));
  assert_eq!(float4::fract(float4(1.5, 2.75, 3.8125, 1.25)), float4(0.5, 0.75, 0.8125, 0.25));

  assert_eq!(double2::fract(double2(1.5, 2.75)), double2(0.5, 0.75));
  assert_eq!(double3::fract(double3(1.5, 2.75, 3.8125)), double3(0.5, 0.75, 0.8125));
  assert_eq!(double4::fract(double4(1.5, 2.75, 3.8125, 1.25)), double4(0.5, 0.75, 0.8125, 0.25));
}

#[test]
fn step() {
  assert_eq!(float2::step(float2(1.0, 2.0), float2(0.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(float3::step(float3(1.0, 2.0, 3.0), float3(0.0, 2.0, 4.0)), float3(0.0, 1.0, 1.0));
  assert_eq!(float4::step(float4(1.0, 2.0, 3.0, 4.0), float4(0.0, 2.0, 4.0, 8.0)), float4(0.0, 1.0, 1.0, 1.0));

  assert_eq!(double2::step(double2(1.0, 2.0), double2(0.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(double3::step(double3(1.0, 2.0, 3.0), double3(0.0, 2.0, 4.0)), double3(0.0, 1.0, 1.0));
  assert_eq!(double4::step(double4(1.0, 2.0, 3.0, 4.0), double4(0.0, 2.0, 4.0, 8.0)), double4(0.0, 1.0, 1.0, 1.0));
}

#[test]
fn smoothstep() {
  assert_eq!(float2::smoothstep(float2(-1.0, -2.0), float2(1.0, 2.0), float2(-1.0, 2.0)), float2(0.0, 1.0));
  assert_eq!(float3::smoothstep(float3(-1.0, -2.0, -3.0), float3(1.0, 2.0, 3.0), float3(-1.0, 2.0, 0.0)), float3(0.0, 1.0, 0.5));
  assert_eq!(float4::smoothstep(float4(-1.0, -2.0, -3.0, -4.0), float4(1.0, 2.0, 3.0, 4.0), float4(-1.0, 2.0, 0.0, 2.0)), float4(0.0, 1.0, 0.5, 0.84375));

  assert_eq!(double2::smoothstep(double2(-1.0, -2.0), double2(1.0, 2.0), double2(-1.0, 2.0)), double2(0.0, 1.0));
  assert_eq!(double3::smoothstep(double3(-1.0, -2.0, -3.0), double3(1.0, 2.0, 3.0), double3(-1.0, 2.0, 0.0)), double3(0.0, 1.0, 0.5));
  assert_eq!(double4::smoothstep(double4(-1.0, -2.0, -3.0, -4.0), double4(1.0, 2.0, 3.0, 4.0), double4(-1.0, 2.0, 0.0, 2.0)), double4(0.0, 1.0, 0.5, 0.84375));
}

#[test]
fn reduce_add() {
  assert_eq!(float2::reduce_add(float2(10.0, -3.0)), 7.0);
  assert_eq!(float3::reduce_add(float3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(float4::reduce_add(float4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(double2::reduce_add(double2(10.0, -3.0)), 7.0);
  assert_eq!(double3::reduce_add(double3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(double4::reduce_add(double4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(int2::reduce_add(int2(10, -3)), 7);
  assert_eq!(int3::reduce_add(int3(10, -3, -9)), -2);
  assert_eq!(int4::reduce_add(int4(10, -3, -9, 0)), -2);

  assert_eq!(long2::reduce_add(long2(10, -3)), 7);
  assert_eq!(long3::reduce_add(long3(10, -3, -9)), -2);
  assert_eq!(long4::reduce_add(long4(10, -3, -9, 0)), -2);
}

#[test]
fn reduce_min() {
  assert_eq!(float2::reduce_min(float2(10.0, -3.0)), -3.0);
  assert_eq!(float3::reduce_min(float3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(float4::reduce_min(float4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(double2::reduce_min(double2(10.0, -3.0)), -3.0);
  assert_eq!(double3::reduce_min(double3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(double4::reduce_min(double4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(int2::reduce_min(int2(10, -3)), -3);
  assert_eq!(int3::reduce_min(int3(10, -3, -9)), -9);
  assert_eq!(int4::reduce_min(int4(10, -3, -9, 0)), -9);

  assert_eq!(long2::reduce_min(long2(10, -3)), -3);
  assert_eq!(long3::reduce_min(long3(10, -3, -9)), -9);
  assert_eq!(long4::reduce_min(long4(10, -3, -9, 0)), -9);
}

#[test]
fn reduce_max() {
  assert_eq!(float2::reduce_max(float2(10.0, -3.0)), 10.0);
  assert_eq!(float3::reduce_max(float3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(float4::reduce_max(float4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(double2::reduce_max(double2(10.0, -3.0)), 10.0);
  assert_eq!(double3::reduce_max(double3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(double4::reduce_max(double4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(int2::reduce_max(int2(10, -3)), 10);
  assert_eq!(int3::reduce_max(int3(10, -3, -9)), 10);
  assert_eq!(int4::reduce_max(int4(10, -3, -9, 0)), 10);

  assert_eq!(long2::reduce_max(long2(10, -3)), 10);
  assert_eq!(long3::reduce_max(long3(10, -3, -9)), 10);
  assert_eq!(long4::reduce_max(long4(10, -3, -9, 0)), 10);
}
