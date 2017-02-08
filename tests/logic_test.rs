#![allow(overflowing_literals)]

extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn any() {
  assert_eq!(int2::any(int2(0x80000000, 0x00000000)), true);
  assert_eq!(int2::any(int2(0x7FFFFFFF, 0x7FFFFFFF)), false);
  assert_eq!(int2::any(int2(0x7FFFFFFF, 0xFFFFFFFF)), true);
  assert_eq!(int2::any(int2(0x10000000, 0x40000000)), false);
  assert_eq!(int3::any(int3(0x80000000, 0x00000000, 0x80000000)), true);
  assert_eq!(int3::any(int3(0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(int3::any(int3(0x00000000, 0x00010000, 0x00002000)), false);
  assert_eq!(int4::any(int4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), true);
  assert_eq!(int4::any(int4(0x00000000, 0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(int4::any(int4(0x00000040, 0x00000060, 0x00000008, 0x0000000F)), false);
}

#[test]
fn all() {
  assert_eq!(int2::all(int2(0x80000000, 0x00000000)), false);
  assert_eq!(int2::all(int2(0x80000000, 0x80000000)), true);
  assert_eq!(int3::all(int3(0x80000000, 0x00000000, 0x80000000)), false);
  assert_eq!(int3::all(int3(0x80000000, 0x80000000, 0x80000000)), true);
  assert_eq!(int4::all(int4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), false);
  assert_eq!(int4::all(int4(0x80000000, 0x80000000, 0x80000000, 0x80000000)), true);
}

#[test]
fn select() {
  assert_eq!(float2::select(float2(10.0, -2.0), float2(11.0, -3.0), int2(0x80000000, 0x00000000)), float2(11.0, -2.0));
  assert_eq!(float3::select(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), int3(0x80000000, 0x00000000, 0x00000000)), float3(11.0, -2.0, -9.5));
  assert_eq!(float4::select(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), int4(0x80000000, 0x00000000, 0x00000000, 0x80000000)), float4(11.0, -2.0, -9.5, 0.0));
}

#[test]
fn bitselect() {
  assert_eq!(int2::bitselect(int2(0xFFFFFFFF, 0x00000000), int2(0x00000000, 0xFFFFFFFF), int2(0x11111111, 0x33333333)), int2(0xEEEEEEEE, 0x33333333));
  assert_eq!(int3::bitselect(int3(0xFFFFFFFF, 0x00000000, 0x00000000), int3(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF), int3(0x11111111, 0x33333333, 0x55555555)), int3(0xEEEEEEEE, 0x33333333, 0x55555555));
  assert_eq!(int4::bitselect(int4(0xFFFFFFFF, 0x00000000, 0x00000000, 0xFFFFFFFF), int4(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000), int4(0x11111111, 0x33333333, 0x55555555, 0x77777777)), int4(0xEEEEEEEE, 0x33333333, 0x55555555, 0x88888888));
}
