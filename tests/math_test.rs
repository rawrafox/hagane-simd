#![allow(overflowing_literals)]

extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn copysign() {
  assert_eq!(float2::copysign(float2(10.0, -3.0), float2(-2.0, 4.0)), float2(-10.0, 3.0));
  assert_eq!(float3::copysign(float3(10.0, -3.0, 2.0), float3(-2.0, 4.0, 3.0)), float3(-10.0, 3.0, 2.0));
  assert_eq!(float4::copysign(float4(10.0, -3.0, 2.0, -3.0), float4(-2.0, 4.0, 3.0, -4.0)), float4(-10.0, 3.0, 2.0, -3.0));
}

#[test]
fn sqrt() {
  assert_eq!(float2::sqrt(float2(100.0, 4.0)), float2(10.0, 2.0));
  assert_eq!(float3::sqrt(float3(100.0, 4.0, 1.0)), float3(10.0, 2.0, 1.0));
  assert_eq!(float4::sqrt(float4(100.0, 4.0, 1.0, 9.0)), float4(10.0, 2.0, 1.0, 3.0));
}

#[test]
fn ceil() {
  assert_eq!(float2::ceil(float2(100.1, -4.4)), float2(101.0, -4.0));
  assert_eq!(float3::ceil(float3(100.1, -4.4, 1.9)), float3(101.0, -4.0, 2.0));
  assert_eq!(float4::ceil(float4(100.1, -4.4, 1.9, 9.0)), float4(101.0, -4.0, 2.0, 9.0));
}

#[test]
fn floor() {
  assert_eq!(float2::floor(float2(100.1, -4.4)), float2(100.0, -5.0));
  assert_eq!(float3::floor(float3(100.1, -4.4, 1.9)), float3(100.0, -5.0, 1.0));
  assert_eq!(float4::floor(float4(100.1, -4.4, 1.9, 9.0)), float4(100.0, -5.0, 1.0, 9.0));
}

#[test]
fn trunc() {
  assert_eq!(float2::trunc(float2(100.1, -4.4)), float2(100.0, -4.0));
  assert_eq!(float3::trunc(float3(100.1, -4.4, 1.9)), float3(100.0, -4.0, 1.0));
  assert_eq!(float4::trunc(float4(100.1, -4.4, 1.9, 9.0)), float4(100.0, -4.0, 1.0, 9.0));
}
