#![allow(overflowing_literals)]

extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn any() {
  assert_eq!(simd::any(char2(0x80, 0x00)), true);
  assert_eq!(simd::any(char2(0x7F, 0x7F)), false);
  assert_eq!(simd::any(char2(0x7F, 0xFF)), true);
  assert_eq!(simd::any(char2(0x10, 0x40)), false);
  assert_eq!(simd::any(char3(0x80, 0x00, 0x80)), true);
  assert_eq!(simd::any(char3(0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(char3(0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(char4(0x80, 0x00, 0x80, 0x80)), true);
  assert_eq!(simd::any(char4(0x00, 0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(char4(0x00, 0x00, 0x00, 0x00)), false);

  assert_eq!(simd::any(uchar2(0x80, 0x00)), true);
  assert_eq!(simd::any(uchar2(0x7F, 0x7F)), false);
  assert_eq!(simd::any(uchar2(0x7F, 0xFF)), true);
  assert_eq!(simd::any(uchar2(0x10, 0x40)), false);
  assert_eq!(simd::any(uchar3(0x80, 0x00, 0x80)), true);
  assert_eq!(simd::any(uchar3(0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(uchar3(0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(uchar4(0x80, 0x00, 0x80, 0x80)), true);
  assert_eq!(simd::any(uchar4(0x00, 0x00, 0x00, 0x00)), false);
  assert_eq!(simd::any(uchar4(0x00, 0x00, 0x00, 0x00)), false);

  assert_eq!(simd::any(short2(0x8000, 0x0000)), true);
  assert_eq!(simd::any(short2(0x7FFF, 0x7FFF)), false);
  assert_eq!(simd::any(short2(0x7FFF, 0xFFFF)), true);
  assert_eq!(simd::any(short2(0x1000, 0x4000)), false);
  assert_eq!(simd::any(short3(0x8000, 0x0000, 0x8000)), true);
  assert_eq!(simd::any(short3(0x0000, 0x0000, 0x0000)), false);
  assert_eq!(simd::any(short3(0x0000, 0x0001, 0x0000)), false);
  assert_eq!(simd::any(short4(0x8000, 0x0000, 0x8000, 0x8000)), true);
  assert_eq!(simd::any(short4(0x0000, 0x0000, 0x0000, 0x0000)), false);
  assert_eq!(simd::any(short4(0x0000, 0x0000, 0x0000, 0x0000)), false);

  assert_eq!(simd::any(ushort2(0x8000, 0x0000)), true);
  assert_eq!(simd::any(ushort2(0x7FFF, 0x7FFF)), false);
  assert_eq!(simd::any(ushort2(0x7FFF, 0xFFFF)), true);
  assert_eq!(simd::any(ushort2(0x1000, 0x4000)), false);
  assert_eq!(simd::any(ushort3(0x8000, 0x0000, 0x8000)), true);
  assert_eq!(simd::any(ushort3(0x0000, 0x0000, 0x0000)), false);
  assert_eq!(simd::any(ushort3(0x0000, 0x0001, 0x0000)), false);
  assert_eq!(simd::any(ushort4(0x8000, 0x0000, 0x8000, 0x8000)), true);
  assert_eq!(simd::any(ushort4(0x0000, 0x0000, 0x0000, 0x0000)), false);
  assert_eq!(simd::any(ushort4(0x0000, 0x0000, 0x0000, 0x0000)), false);

  assert_eq!(simd::any(int2(0x80000000, 0x00000000)), true);
  assert_eq!(simd::any(int2(0x7FFFFFFF, 0x7FFFFFFF)), false);
  assert_eq!(simd::any(int2(0x7FFFFFFF, 0xFFFFFFFF)), true);
  assert_eq!(simd::any(int2(0x10000000, 0x40000000)), false);
  assert_eq!(simd::any(int3(0x80000000, 0x00000000, 0x80000000)), true);
  assert_eq!(simd::any(int3(0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(simd::any(int3(0x00000000, 0x00010000, 0x00002000)), false);
  assert_eq!(simd::any(int4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), true);
  assert_eq!(simd::any(int4(0x00000000, 0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(simd::any(int4(0x00000040, 0x00000060, 0x00000008, 0x0000000F)), false);

  assert_eq!(simd::any(uint2(0x80000000, 0x00000000)), true);
  assert_eq!(simd::any(uint2(0x7FFFFFFF, 0x7FFFFFFF)), false);
  assert_eq!(simd::any(uint2(0x7FFFFFFF, 0xFFFFFFFF)), true);
  assert_eq!(simd::any(uint2(0x10000000, 0x40000000)), false);
  assert_eq!(simd::any(uint3(0x80000000, 0x00000000, 0x80000000)), true);
  assert_eq!(simd::any(uint3(0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(simd::any(uint3(0x00000000, 0x00010000, 0x00002000)), false);
  assert_eq!(simd::any(uint4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), true);
  assert_eq!(simd::any(uint4(0x00000000, 0x00000000, 0x00000000, 0x00000000)), false);
  assert_eq!(simd::any(uint4(0x00000040, 0x00000060, 0x00000008, 0x0000000F)), false);

  assert_eq!(simd::any(long2(0x8000000000000000, 0x0000000000000000)), true);
  assert_eq!(simd::any(long2(0x7FFFFFFF00000000, 0x7FFFFFFF00000000)), false);
  assert_eq!(simd::any(long2(0x7FFFFFFF00000000, 0xFFFFFFFF00000000)), true);
  assert_eq!(simd::any(long2(0x1000000000000000, 0x4000000000000000)), false);
  assert_eq!(simd::any(long3(0x8000000000000000, 0x0000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::any(long3(0x0000000000000000, 0x0000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::any(long3(0x0000000000000000, 0x0001000000000000, 0x0000200000000000)), false);
  assert_eq!(simd::any(long4(0x8000000000000000, 0x0000000000000000, 0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::any(long4(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::any(long4(0x0000004000000000, 0x0000006000000000, 0x0000000800000000, 0x0000000F00000000)), false);

  assert_eq!(simd::any(ulong2(0x8000000000000000, 0x0000000000000000)), true);
  assert_eq!(simd::any(ulong2(0x7FFFFFFF00000000, 0x7FFFFFFF00000000)), false);
  assert_eq!(simd::any(ulong2(0x7FFFFFFF00000000, 0xFFFFFFFF00000000)), true);
  assert_eq!(simd::any(ulong2(0x1000000000000000, 0x4000000000000000)), false);
  assert_eq!(simd::any(ulong3(0x8000000000000000, 0x0000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::any(ulong3(0x0000000000000000, 0x0000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::any(ulong3(0x0000000000000000, 0x0001000000000000, 0x0000200000000000)), false);
  assert_eq!(simd::any(ulong4(0x8000000000000000, 0x0000000000000000, 0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::any(ulong4(0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::any(ulong4(0x0000004000000000, 0x0000006000000000, 0x0000000800000000, 0x0000000F00000000)), false);
}

#[test]
fn all() {
  assert_eq!(simd::all(char2(0x80, 0x00)), false);
  assert_eq!(simd::all(char2(0x80, 0x80)), true);
  assert_eq!(simd::all(char3(0x80, 0x00, 0x80)), false);
  assert_eq!(simd::all(char3(0x80, 0x80, 0x80)), true);
  assert_eq!(simd::all(char4(0x80, 0x00, 0x80, 0x80)), false);
  assert_eq!(simd::all(char4(0x80, 0x80, 0x80, 0x80)), true);

  assert_eq!(simd::all(uchar2(0x80, 0x00)), false);
  assert_eq!(simd::all(uchar2(0x80, 0x80)), true);
  assert_eq!(simd::all(uchar3(0x80, 0x00, 0x80)), false);
  assert_eq!(simd::all(uchar3(0x80, 0x80, 0x80)), true);
  assert_eq!(simd::all(uchar4(0x80, 0x00, 0x80, 0x80)), false);
  assert_eq!(simd::all(uchar4(0x80, 0x80, 0x80, 0x80)), true);

  assert_eq!(simd::all(short2(0x8000, 0x0000)), false);
  assert_eq!(simd::all(short2(0x8000, 0x8000)), true);
  assert_eq!(simd::all(short3(0x8000, 0x0000, 0x8000)), false);
  assert_eq!(simd::all(short3(0x8000, 0x8000, 0x8000)), true);
  assert_eq!(simd::all(short4(0x8000, 0x0000, 0x8000, 0x8000)), false);
  assert_eq!(simd::all(short4(0x8000, 0x8000, 0x8000, 0x8000)), true);

  assert_eq!(simd::all(ushort2(0x8000, 0x0000)), false);
  assert_eq!(simd::all(ushort2(0x8000, 0x8000)), true);
  assert_eq!(simd::all(ushort3(0x8000, 0x0000, 0x8000)), false);
  assert_eq!(simd::all(ushort3(0x8000, 0x8000, 0x8000)), true);
  assert_eq!(simd::all(ushort4(0x8000, 0x0000, 0x8000, 0x8000)), false);
  assert_eq!(simd::all(ushort4(0x8000, 0x8000, 0x8000, 0x8000)), true);

  assert_eq!(simd::all(int2(0x80000000, 0x00000000)), false);
  assert_eq!(simd::all(int2(0x80000000, 0x80000000)), true);
  assert_eq!(simd::all(int3(0x80000000, 0x00000000, 0x80000000)), false);
  assert_eq!(simd::all(int3(0x80000000, 0x80000000, 0x80000000)), true);
  assert_eq!(simd::all(int4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), false);
  assert_eq!(simd::all(int4(0x80000000, 0x80000000, 0x80000000, 0x80000000)), true);

  assert_eq!(simd::all(uint2(0x80000000, 0x00000000)), false);
  assert_eq!(simd::all(uint2(0x80000000, 0x80000000)), true);
  assert_eq!(simd::all(uint3(0x80000000, 0x00000000, 0x80000000)), false);
  assert_eq!(simd::all(uint3(0x80000000, 0x80000000, 0x80000000)), true);
  assert_eq!(simd::all(uint4(0x80000000, 0x00000000, 0x80000000, 0x80000000)), false);
  assert_eq!(simd::all(uint4(0x80000000, 0x80000000, 0x80000000, 0x80000000)), true);

  assert_eq!(simd::all(long2(0x8000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::all(long2(0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::all(long3(0x8000000000000000, 0x0000000000000000, 0x8000000000000000)), false);
  assert_eq!(simd::all(long3(0x8000000000000000, 0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::all(long4(0x8000000000000000, 0x0000000000000000, 0x8000000000000000, 0x8000000000000000)), false);
  assert_eq!(simd::all(long4(0x8000000000000000, 0x8000000000000000, 0x8000000000000000, 0x8000000000000000)), true);

  assert_eq!(simd::all(ulong2(0x8000000000000000, 0x0000000000000000)), false);
  assert_eq!(simd::all(ulong2(0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::all(ulong3(0x8000000000000000, 0x0000000000000000, 0x8000000000000000)), false);
  assert_eq!(simd::all(ulong3(0x8000000000000000, 0x8000000000000000, 0x8000000000000000)), true);
  assert_eq!(simd::all(ulong4(0x8000000000000000, 0x0000000000000000, 0x8000000000000000, 0x8000000000000000)), false);
  assert_eq!(simd::all(ulong4(0x8000000000000000, 0x8000000000000000, 0x8000000000000000, 0x8000000000000000)), true);
}

#[test]
fn select() {
  assert_eq!(float2::select(float2(10.0, -2.0), float2(11.0, -3.0), int2(0x80000000, 0x00000000)), float2(11.0, -2.0));
  assert_eq!(float3::select(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), int3(0x80000000, 0x00000000, 0x00000000)), float3(11.0, -2.0, -9.5));
  assert_eq!(float4::select(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), int4(0x80000000, 0x00000000, 0x00000000, 0x80000000)), float4(11.0, -2.0, -9.5, 0.0));

  assert_eq!(double2::select(double2(10.0, -2.0), double2(11.0, -3.0), long2(0x8000000000000000, 0x0000000000000000)), double2(11.0, -2.0));
  assert_eq!(double3::select(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0), long3(0x8000000000000000, 0x0000000000000000, 0x0000000000000000)), double3(11.0, -2.0, -9.5));
  assert_eq!(double4::select(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0), long4(0x8000000000000000, 0x0000000000000000, 0x0000000000000000, 0x8000000000000000)), double4(11.0, -2.0, -9.5, 0.0));
}

#[test]
fn bitselect() {
  assert_eq!(char2::bitselect(char2(0xFF, 0x00), char2(0x00, 0xFF), char2(0x11, 0x33)), char2(0xEE, 0x33));
  assert_eq!(char3::bitselect(char3(0xFF, 0x00, 0x00), char3(0x00, 0xFF, 0xFF), char3(0x11, 0x33, 0x55)), char3(0xEE, 0x33, 0x55));
  assert_eq!(char4::bitselect(char4(0xFF, 0x00, 0x00, 0xFF), char4(0x00, 0xFF, 0xFF, 0x00), char4(0x11, 0x33, 0x55, 0x77)), char4(0xEE, 0x33, 0x55, 0x88));

  assert_eq!(uchar2::bitselect(uchar2(0xFF, 0x00), uchar2(0x00, 0xFF), char2(0x11, 0x33)), uchar2(0xEE, 0x33));
  assert_eq!(uchar3::bitselect(uchar3(0xFF, 0x00, 0x00), uchar3(0x00, 0xFF, 0xFF), char3(0x11, 0x33, 0x55)), uchar3(0xEE, 0x33, 0x55));
  assert_eq!(uchar4::bitselect(uchar4(0xFF, 0x00, 0x00, 0xFF), uchar4(0x00, 0xFF, 0xFF, 0x00), char4(0x11, 0x33, 0x55, 0x77)), uchar4(0xEE, 0x33, 0x55, 0x88));

  assert_eq!(short2::bitselect(short2(0xFFFF, 0x0000), short2(0x0000, 0xFFFF), short2(0x1111, 0x3333)), short2(0xEEEE, 0x3333));
  assert_eq!(short3::bitselect(short3(0xFFFF, 0x0000, 0x0000), short3(0x0000, 0xFFFF, 0xFFFF), short3(0x1111, 0x3333, 0x5555)), short3(0xEEEE, 0x3333, 0x5555));
  assert_eq!(short4::bitselect(short4(0xFFFF, 0x0000, 0x0000, 0xFFFF), short4(0x0000, 0xFFFF, 0xFFFF, 0x0000), short4(0x1111, 0x3333, 0x5555, 0x7777)), short4(0xEEEE, 0x3333, 0x5555, 0x8888));

  assert_eq!(ushort2::bitselect(ushort2(0xFFFF, 0x0000), ushort2(0x0000, 0xFFFF), short2(0x1111, 0x3333)), ushort2(0xEEEE, 0x3333));
  assert_eq!(ushort3::bitselect(ushort3(0xFFFF, 0x0000, 0x0000), ushort3(0x0000, 0xFFFF, 0xFFFF), short3(0x1111, 0x3333, 0x5555)), ushort3(0xEEEE, 0x3333, 0x5555));
  assert_eq!(ushort4::bitselect(ushort4(0xFFFF, 0x0000, 0x0000, 0xFFFF), ushort4(0x0000, 0xFFFF, 0xFFFF, 0x0000), short4(0x1111, 0x3333, 0x5555, 0x7777)), ushort4(0xEEEE, 0x3333, 0x5555, 0x8888));

  assert_eq!(int2::bitselect(int2(0xFFFFFFFF, 0x00000000), int2(0x00000000, 0xFFFFFFFF), int2(0x11111111, 0x33333333)), int2(0xEEEEEEEE, 0x33333333));
  assert_eq!(int3::bitselect(int3(0xFFFFFFFF, 0x00000000, 0x00000000), int3(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF), int3(0x11111111, 0x33333333, 0x55555555)), int3(0xEEEEEEEE, 0x33333333, 0x55555555));
  assert_eq!(int4::bitselect(int4(0xFFFFFFFF, 0x00000000, 0x00000000, 0xFFFFFFFF), int4(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000), int4(0x11111111, 0x33333333, 0x55555555, 0x77777777)), int4(0xEEEEEEEE, 0x33333333, 0x55555555, 0x88888888));

  assert_eq!(uint2::bitselect(uint2(0xFFFFFFFF, 0x00000000), uint2(0x00000000, 0xFFFFFFFF), int2(0x11111111, 0x33333333)), uint2(0xEEEEEEEE, 0x33333333));
  assert_eq!(uint3::bitselect(uint3(0xFFFFFFFF, 0x00000000, 0x00000000), uint3(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF), int3(0x11111111, 0x33333333, 0x55555555)), uint3(0xEEEEEEEE, 0x33333333, 0x55555555));
  assert_eq!(uint4::bitselect(uint4(0xFFFFFFFF, 0x00000000, 0x00000000, 0xFFFFFFFF), uint4(0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000), int4(0x11111111, 0x33333333, 0x55555555, 0x77777777)), uint4(0xEEEEEEEE, 0x33333333, 0x55555555, 0x88888888));

  assert_eq!(long2::bitselect(long2(0xFFFFFFFF00000000, 0x0000000000000000), long2(0x0000000000000000, 0xFFFFFFFF00000000), long2(0x1111111100000000, 0x3333333300000000)), long2(0xEEEEEEEE00000000, 0x3333333300000000));
  assert_eq!(long3::bitselect(long3(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000), long3(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000), long3(0x1111111100000000, 0x3333333300000000, 0x5555555500000000)), long3(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000));
  assert_eq!(long4::bitselect(long4(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFF00000000), long4(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000, 0x0000000000000000), long4(0x1111111100000000, 0x3333333300000000, 0x5555555500000000, 0x7777777700000000)), long4(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000, 0x8888888800000000));

  assert_eq!(ulong2::bitselect(ulong2(0xFFFFFFFF00000000, 0x0000000000000000), ulong2(0x0000000000000000, 0xFFFFFFFF00000000), long2(0x1111111100000000, 0x3333333300000000)), ulong2(0xEEEEEEEE00000000, 0x3333333300000000));
  assert_eq!(ulong3::bitselect(ulong3(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000), ulong3(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000), long3(0x1111111100000000, 0x3333333300000000, 0x5555555500000000)), ulong3(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000));
  assert_eq!(ulong4::bitselect(ulong4(0xFFFFFFFF00000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFF00000000), ulong4(0x0000000000000000, 0xFFFFFFFF00000000, 0xFFFFFFFF00000000, 0x0000000000000000), long4(0x1111111100000000, 0x3333333300000000, 0x5555555500000000, 0x7777777700000000)), ulong4(0xEEEEEEEE00000000, 0x3333333300000000, 0x5555555500000000, 0x8888888800000000));
}
