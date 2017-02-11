#![allow(overflowing_literals)]

extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn reduce_add() {
  assert_eq!(simd::reduce_add(float2(10.0, -3.0)), 7.0);
  assert_eq!(simd::reduce_add(float3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(simd::reduce_add(float4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(simd::reduce_add(double2(10.0, -3.0)), 7.0);
  assert_eq!(simd::reduce_add(double3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(simd::reduce_add(double4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(simd::reduce_add(char2(10, -3)), 7);
  assert_eq!(simd::reduce_add(char3(10, -3, -9)), -2);
  assert_eq!(simd::reduce_add(char4(10, -3, -9, 0)), -2);

  assert_eq!(simd::reduce_add(uchar2(10, 3)), 13);
  assert_eq!(simd::reduce_add(uchar3(10, 3, 9)), 22);
  assert_eq!(simd::reduce_add(uchar4(10, 3, 9, 0)), 22);

  assert_eq!(simd::reduce_add(short2(10, -3)), 7);
  assert_eq!(simd::reduce_add(short3(10, -3, -9)), -2);
  assert_eq!(simd::reduce_add(short4(10, -3, -9, 0)), -2);

  assert_eq!(simd::reduce_add(ushort2(10, 3)), 13);
  assert_eq!(simd::reduce_add(ushort3(10, 3, 9)), 22);
  assert_eq!(simd::reduce_add(ushort4(10, 3, 9, 0)), 22);

  assert_eq!(simd::reduce_add(int2(10, -3)), 7);
  assert_eq!(simd::reduce_add(int3(10, -3, -9)), -2);
  assert_eq!(simd::reduce_add(int4(10, -3, -9, 0)), -2);

  assert_eq!(simd::reduce_add(uint2(10, 3)), 13);
  assert_eq!(simd::reduce_add(uint3(10, 3, 9)), 22);
  assert_eq!(simd::reduce_add(uint4(10, 3, 9, 0)), 22);

  assert_eq!(simd::reduce_add(long2(10, -3)), 7);
  assert_eq!(simd::reduce_add(long3(10, -3, -9)), -2);
  assert_eq!(simd::reduce_add(long4(10, -3, -9, 0)), -2);

  assert_eq!(simd::reduce_add(ulong2(10, 3)), 13);
  assert_eq!(simd::reduce_add(ulong3(10, 3, 9)), 22);
  assert_eq!(simd::reduce_add(ulong4(10, 3, 9, 0)), 22);
}

#[test]
fn reduce_min() {
  assert_eq!(simd::reduce_min(float2(10.0, -3.0)), -3.0);
  assert_eq!(simd::reduce_min(float3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(simd::reduce_min(float4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(simd::reduce_min(double2(10.0, -3.0)), -3.0);
  assert_eq!(simd::reduce_min(double3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(simd::reduce_min(double4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(simd::reduce_min(char2(10, -3)), -3);
  assert_eq!(simd::reduce_min(char3(10, -3, -9)), -9);
  assert_eq!(simd::reduce_min(char4(10, -3, -9, 0)), -9);

  assert_eq!(simd::reduce_min(uchar2(10, 3)), 3);
  assert_eq!(simd::reduce_min(uchar3(10, 3, 9)), 3);
  assert_eq!(simd::reduce_min(uchar4(10, 3, 9, 0)), 0);

  assert_eq!(simd::reduce_min(short2(10, -3)), -3);
  assert_eq!(simd::reduce_min(short3(10, -3, -9)), -9);
  assert_eq!(simd::reduce_min(short4(10, -3, -9, 0)), -9);

  assert_eq!(simd::reduce_min(ushort2(10, 3)), 3);
  assert_eq!(simd::reduce_min(ushort3(10, 3, 9)), 3);
  assert_eq!(simd::reduce_min(ushort4(10, 3, 9, 0)), 0);

  assert_eq!(simd::reduce_min(int2(10, -3)), -3);
  assert_eq!(simd::reduce_min(int3(10, -3, -9)), -9);
  assert_eq!(simd::reduce_min(int4(10, -3, -9, 0)), -9);

  assert_eq!(simd::reduce_min(uint2(10, 3)), 3);
  assert_eq!(simd::reduce_min(uint3(10, 3, 9)), 3);
  assert_eq!(simd::reduce_min(uint4(10, 3, 9, 0)), 0);

  assert_eq!(simd::reduce_min(long2(10, -3)), -3);
  assert_eq!(simd::reduce_min(long3(10, -3, -9)), -9);
  assert_eq!(simd::reduce_min(long4(10, -3, -9, 0)), -9);

  assert_eq!(simd::reduce_min(ulong2(10, 3)), 3);
  assert_eq!(simd::reduce_min(ulong3(10, 3, 9)), 3);
  assert_eq!(simd::reduce_min(ulong4(10, 3, 9, 0)), 0);
}

#[test]
fn reduce_max() {
  assert_eq!(simd::reduce_max(float2(10.0, -3.0)), 10.0);
  assert_eq!(simd::reduce_max(float3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(simd::reduce_max(float4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(simd::reduce_max(double2(10.0, -3.0)), 10.0);
  assert_eq!(simd::reduce_max(double3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(simd::reduce_max(double4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(simd::reduce_max(char2(10, -3)), 10);
  assert_eq!(simd::reduce_max(char3(10, -3, -9)), 10);
  assert_eq!(simd::reduce_max(char4(10, -3, -9, 0)), 10);

  assert_eq!(simd::reduce_max(uchar2(10, 3)), 10);
  assert_eq!(simd::reduce_max(uchar3(10, 3, 9)), 10);
  assert_eq!(simd::reduce_max(uchar4(10, 3, 9, 0)), 10);

  assert_eq!(simd::reduce_max(short2(10, -3)), 10);
  assert_eq!(simd::reduce_max(short3(10, -3, -9)), 10);
  assert_eq!(simd::reduce_max(short4(10, -3, -9, 0)), 10);

  assert_eq!(simd::reduce_max(ushort2(10, 3)), 10);
  assert_eq!(simd::reduce_max(ushort3(10, 3, 9)), 10);
  assert_eq!(simd::reduce_max(ushort4(10, 3, 9, 0)), 10);

  assert_eq!(simd::reduce_max(int2(10, -3)), 10);
  assert_eq!(simd::reduce_max(int3(10, -3, -9)), 10);
  assert_eq!(simd::reduce_max(int4(10, -3, -9, 0)), 10);

  assert_eq!(simd::reduce_max(uint2(10, 3)), 10);
  assert_eq!(simd::reduce_max(uint3(10, 3, 9)), 10);
  assert_eq!(simd::reduce_max(uint4(10, 3, 9, 0)), 10);

  assert_eq!(simd::reduce_max(long2(10, -3)), 10);
  assert_eq!(simd::reduce_max(long3(10, -3, -9)), 10);
  assert_eq!(simd::reduce_max(long4(10, -3, -9, 0)), 10);

  assert_eq!(simd::reduce_max(ulong2(10, 3)), 10);
  assert_eq!(simd::reduce_max(ulong3(10, 3, 9)), 10);
  assert_eq!(simd::reduce_max(ulong4(10, 3, 9, 0)), 10);
}
