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
