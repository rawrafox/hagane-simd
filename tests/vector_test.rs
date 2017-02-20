extern crate hagane_simd;

use hagane_simd::*;

#[test]
fn test_abs() {
  assert_eq!(abs(float2(10.0, -3.0)), float2(10.0, 3.0));
  assert_eq!(abs(float3(10.0, -3.0, -9.5)), float3(10.0, 3.0, 9.5));
  assert_eq!(abs(float4(10.0, -3.0, -9.5, 0.0)), float4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(abs(double2(10.0, -3.0)), double2(10.0, 3.0));
  assert_eq!(abs(double3(10.0, -3.0, -9.5)), double3(10.0, 3.0, 9.5));
  assert_eq!(abs(double4(10.0, -3.0, -9.5, 0.0)), double4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(abs(char2(10, -3)), char2(10, 3));
  assert_eq!(abs(char3(10, -3, -9)), char3(10, 3, 9));
  assert_eq!(abs(char4(10, -3, -9, 0)), char4(10, 3, 9, 0));

  assert_eq!(abs(short2(10, -3)), short2(10, 3));
  assert_eq!(abs(short3(10, -3, -9)), short3(10, 3, 9));
  assert_eq!(abs(short4(10, -3, -9, 0)), short4(10, 3, 9, 0));

  assert_eq!(abs(int2(10, -3)), int2(10, 3));
  assert_eq!(abs(int3(10, -3, -9)), int3(10, 3, 9));
  assert_eq!(abs(int4(10, -3, -9, 0)), int4(10, 3, 9, 0));

  assert_eq!(abs(long2(10, -3)), long2(10, 3));
  assert_eq!(abs(long3(10, -3, -9)), long3(10, 3, 9));
  assert_eq!(abs(long4(10, -3, -9, 0)), long4(10, 3, 9, 0));
}

#[test]
fn test_max() {
  assert_eq!(max(float2(10.0, -2.0), float2(11.0, -3.0)), float2(11.0, -2.0));
  assert_eq!(max(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(max(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(max(double2(10.0, -2.0), double2(11.0, -3.0)), double2(11.0, -2.0));
  assert_eq!(max(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(max(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(max(char2(10, -2), char2(11, -3)), char2(11, -2));
  assert_eq!(max(char3(10, -2, -9), char3(11, -3, 10)), char3(11, -2, 10));
  assert_eq!(max(char4(10, -2, -9, 1), char4(11, -3, 10, 0)), char4(11, -2, 10, 1));

  assert_eq!(max(uchar2(10, 2), uchar2(11, 3)), uchar2(11, 3));
  assert_eq!(max(uchar3(10, 2, 9), uchar3(11, 3, 10)), uchar3(11, 3, 10));
  assert_eq!(max(uchar4(10, 2, 9, 1), uchar4(11, 3, 10, 0)), uchar4(11, 3, 10, 1));

  assert_eq!(max(short2(10, -2), short2(11, -3)), short2(11, -2));
  assert_eq!(max(short3(10, -2, -9), short3(11, -3, 10)), short3(11, -2, 10));
  assert_eq!(max(short4(10, -2, -9, 1), short4(11, -3, 10, 0)), short4(11, -2, 10, 1));

  assert_eq!(max(ushort2(10, 2), ushort2(11, 3)), ushort2(11, 3));
  assert_eq!(max(ushort3(10, 2, 9), ushort3(11, 3, 10)), ushort3(11, 3, 10));
  assert_eq!(max(ushort4(10, 2, 9, 1), ushort4(11, 3, 10, 0)), ushort4(11, 3, 10, 1));

  assert_eq!(max(int2(10, -2), int2(11, -3)), int2(11, -2));
  assert_eq!(max(int3(10, -2, -9), int3(11, -3, 10)), int3(11, -2, 10));
  assert_eq!(max(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(11, -2, 10, 1));

  assert_eq!(max(uint2(10, 2), uint2(11, 3)), uint2(11, 3));
  assert_eq!(max(uint3(10, 2, 9), uint3(11, 3, 10)), uint3(11, 3, 10));
  assert_eq!(max(uint4(10, 2, 9, 1), uint4(11, 3, 10, 0)), uint4(11, 3, 10, 1));

  assert_eq!(max(long2(10, -2), long2(11, -3)), long2(11, -2));
  assert_eq!(max(long3(10, -2, -9), long3(11, -3, 10)), long3(11, -2, 10));
  assert_eq!(max(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(11, -2, 10, 1));

  assert_eq!(max(ulong2(10, 2), ulong2(11, 3)), ulong2(11, 3));
  assert_eq!(max(ulong3(10, 2, 9), ulong3(11, 3, 10)), ulong3(11, 3, 10));
  assert_eq!(max(ulong4(10, 2, 9, 1), ulong4(11, 3, 10, 0)), ulong4(11, 3, 10, 1));
}

#[test]
fn test_min() {
  assert_eq!(min(float2(10.0, -2.0), float2(11.0, -3.0)), float2(10.0, -3.0));
  assert_eq!(min(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(10.0, -3.0, -9.5));
  assert_eq!(min(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(min(double2(10.0, -2.0), double2(11.0, -3.0)), double2(10.0, -3.0));
  assert_eq!(min(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(10.0, -3.0, -9.5));
  assert_eq!(min(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(min(char2(10, -2), char2(11, -3)), char2(10, -3));
  assert_eq!(min(char3(10, -2, -9), char3(11, -3, 10)), char3(10, -3, -9));
  assert_eq!(min(char4(10, -2, -9, 1), char4(11, -3, 10, 0)), char4(10, -3, -9, 0));

  assert_eq!(min(uchar2(10, 2), uchar2(11, 3)), uchar2(10, 2));
  assert_eq!(min(uchar3(10, 2, 9), uchar3(11, 3, 10)), uchar3(10, 2, 9));
  assert_eq!(min(uchar4(10, 2, 9, 1), uchar4(11, 3, 10, 0)), uchar4(10, 2, 9, 0));

  assert_eq!(min(short2(10, -2), short2(11, -3)), short2(10, -3));
  assert_eq!(min(short3(10, -2, -9), short3(11, -3, 10)), short3(10, -3, -9));
  assert_eq!(min(short4(10, -2, -9, 1), short4(11, -3, 10, 0)), short4(10, -3, -9, 0));

  assert_eq!(min(ushort2(10, 2), ushort2(11, 3)), ushort2(10, 2));
  assert_eq!(min(ushort3(10, 2, 9), ushort3(11, 3, 10)), ushort3(10, 2, 9));
  assert_eq!(min(ushort4(10, 2, 9, 1), ushort4(11, 3, 10, 0)), ushort4(10, 2, 9, 0));

  assert_eq!(min(int2(10, -2), int2(11, -3)), int2(10, -3));
  assert_eq!(min(int3(10, -2, -9), int3(11, -3, 10)), int3(10, -3, -9));
  assert_eq!(min(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(10, -3, -9, 0));

  assert_eq!(min(uint2(10, 2), uint2(11, 3)), uint2(10, 2));
  assert_eq!(min(uint3(10, 2, 9), uint3(11, 3, 10)), uint3(10, 2, 9));
  assert_eq!(min(uint4(10, 2, 9, 1), uint4(11, 3, 10, 0)), uint4(10, 2, 9, 0));

  assert_eq!(min(long2(10, -2), long2(11, -3)), long2(10, -3));
  assert_eq!(min(long3(10, -2, -9), long3(11, -3, 10)), long3(10, -3, -9));
  assert_eq!(min(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(10, -3, -9, 0));

  assert_eq!(min(ulong2(10, 2), ulong2(11, 3)), ulong2(10, 2));
  assert_eq!(min(ulong3(10, 2, 9), ulong3(11, 3, 10)), ulong3(10, 2, 9));
  assert_eq!(min(ulong4(10, 2, 9, 1), ulong4(11, 3, 10, 0)), ulong4(10, 2, 9, 0));
}

#[test]
fn test_clamp() {
  assert_eq!(clamp(float2(10.0, -2.0), float2(11.0, -3.0), float2(11.0, -1.0)), float2(11.0, -2.0));
  assert_eq!(clamp(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), float3(11.0, -1.0, 11.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(clamp(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), float4(11.0, -1.0, 10.0, 0.5)), float4(11.0, -2.0, 10.0, 0.5));

  assert_eq!(clamp(double2(10.0, -2.0), double2(11.0, -3.0), double2(11.0, -1.0)), double2(11.0, -2.0));
  assert_eq!(clamp(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0), double3(11.0, -1.0, 11.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(clamp(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0), double4(11.0, -1.0, 10.0, 0.5)), double4(11.0, -2.0, 10.0, 0.5));
}

#[test]
fn test_reduce_add() {
  assert_eq!(reduce_add(float2(10.0, -3.0)), 7.0);
  assert_eq!(reduce_add(float3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(reduce_add(float4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(reduce_add(double2(10.0, -3.0)), 7.0);
  assert_eq!(reduce_add(double3(10.0, -3.0, -9.5)), -2.5);
  assert_eq!(reduce_add(double4(10.0, -3.0, -9.5, 0.0)), -2.5);

  assert_eq!(reduce_add(char2(10, -3)), 7);
  assert_eq!(reduce_add(char3(10, -3, -9)), -2);
  assert_eq!(reduce_add(char4(10, -3, -9, 0)), -2);

  assert_eq!(reduce_add(uchar2(10, 3)), 13);
  assert_eq!(reduce_add(uchar3(10, 3, 9)), 22);
  assert_eq!(reduce_add(uchar4(10, 3, 9, 0)), 22);

  assert_eq!(reduce_add(short2(10, -3)), 7);
  assert_eq!(reduce_add(short3(10, -3, -9)), -2);
  assert_eq!(reduce_add(short4(10, -3, -9, 0)), -2);

  assert_eq!(reduce_add(ushort2(10, 3)), 13);
  assert_eq!(reduce_add(ushort3(10, 3, 9)), 22);
  assert_eq!(reduce_add(ushort4(10, 3, 9, 0)), 22);

  assert_eq!(reduce_add(int2(10, -3)), 7);
  assert_eq!(reduce_add(int3(10, -3, -9)), -2);
  assert_eq!(reduce_add(int4(10, -3, -9, 0)), -2);

  assert_eq!(reduce_add(uint2(10, 3)), 13);
  assert_eq!(reduce_add(uint3(10, 3, 9)), 22);
  assert_eq!(reduce_add(uint4(10, 3, 9, 0)), 22);

  assert_eq!(reduce_add(long2(10, -3)), 7);
  assert_eq!(reduce_add(long3(10, -3, -9)), -2);
  assert_eq!(reduce_add(long4(10, -3, -9, 0)), -2);

  assert_eq!(reduce_add(ulong2(10, 3)), 13);
  assert_eq!(reduce_add(ulong3(10, 3, 9)), 22);
  assert_eq!(reduce_add(ulong4(10, 3, 9, 0)), 22);
}

#[test]
fn test_reduce_min() {
  assert_eq!(reduce_min(float2(10.0, -3.0)), -3.0);
  assert_eq!(reduce_min(float3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(reduce_min(float4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(reduce_min(double2(10.0, -3.0)), -3.0);
  assert_eq!(reduce_min(double3(10.0, -3.0, -9.5)), -9.5);
  assert_eq!(reduce_min(double4(10.0, -3.0, -9.5, 0.0)), -9.5);

  assert_eq!(reduce_min(char2(10, -3)), -3);
  assert_eq!(reduce_min(char3(10, -3, -9)), -9);
  assert_eq!(reduce_min(char4(10, -3, -9, 0)), -9);

  assert_eq!(reduce_min(uchar2(10, 3)), 3);
  assert_eq!(reduce_min(uchar3(10, 3, 9)), 3);
  assert_eq!(reduce_min(uchar4(10, 3, 9, 0)), 0);

  assert_eq!(reduce_min(short2(10, -3)), -3);
  assert_eq!(reduce_min(short3(10, -3, -9)), -9);
  assert_eq!(reduce_min(short4(10, -3, -9, 0)), -9);

  assert_eq!(reduce_min(ushort2(10, 3)), 3);
  assert_eq!(reduce_min(ushort3(10, 3, 9)), 3);
  assert_eq!(reduce_min(ushort4(10, 3, 9, 0)), 0);

  assert_eq!(reduce_min(int2(10, -3)), -3);
  assert_eq!(reduce_min(int3(10, -3, -9)), -9);
  assert_eq!(reduce_min(int4(10, -3, -9, 0)), -9);

  assert_eq!(reduce_min(uint2(10, 3)), 3);
  assert_eq!(reduce_min(uint3(10, 3, 9)), 3);
  assert_eq!(reduce_min(uint4(10, 3, 9, 0)), 0);

  assert_eq!(reduce_min(long2(10, -3)), -3);
  assert_eq!(reduce_min(long3(10, -3, -9)), -9);
  assert_eq!(reduce_min(long4(10, -3, -9, 0)), -9);

  assert_eq!(reduce_min(ulong2(10, 3)), 3);
  assert_eq!(reduce_min(ulong3(10, 3, 9)), 3);
  assert_eq!(reduce_min(ulong4(10, 3, 9, 0)), 0);
}

#[test]
fn test_reduce_max() {
  assert_eq!(reduce_max(float2(10.0, -3.0)), 10.0);
  assert_eq!(reduce_max(float3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(reduce_max(float4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(reduce_max(double2(10.0, -3.0)), 10.0);
  assert_eq!(reduce_max(double3(10.0, -3.0, -9.5)), 10.0);
  assert_eq!(reduce_max(double4(10.0, -3.0, -9.5, 0.0)), 10.0);

  assert_eq!(reduce_max(char2(10, -3)), 10);
  assert_eq!(reduce_max(char3(10, -3, -9)), 10);
  assert_eq!(reduce_max(char4(10, -3, -9, 0)), 10);

  assert_eq!(reduce_max(uchar2(10, 3)), 10);
  assert_eq!(reduce_max(uchar3(10, 3, 9)), 10);
  assert_eq!(reduce_max(uchar4(10, 3, 9, 0)), 10);

  assert_eq!(reduce_max(short2(10, -3)), 10);
  assert_eq!(reduce_max(short3(10, -3, -9)), 10);
  assert_eq!(reduce_max(short4(10, -3, -9, 0)), 10);

  assert_eq!(reduce_max(ushort2(10, 3)), 10);
  assert_eq!(reduce_max(ushort3(10, 3, 9)), 10);
  assert_eq!(reduce_max(ushort4(10, 3, 9, 0)), 10);

  assert_eq!(reduce_max(int2(10, -3)), 10);
  assert_eq!(reduce_max(int3(10, -3, -9)), 10);
  assert_eq!(reduce_max(int4(10, -3, -9, 0)), 10);

  assert_eq!(reduce_max(uint2(10, 3)), 10);
  assert_eq!(reduce_max(uint3(10, 3, 9)), 10);
  assert_eq!(reduce_max(uint4(10, 3, 9, 0)), 10);

  assert_eq!(reduce_max(long2(10, -3)), 10);
  assert_eq!(reduce_max(long3(10, -3, -9)), 10);
  assert_eq!(reduce_max(long4(10, -3, -9, 0)), 10);

  assert_eq!(reduce_max(ulong2(10, 3)), 10);
  assert_eq!(reduce_max(ulong3(10, 3, 9)), 10);
  assert_eq!(reduce_max(ulong4(10, 3, 9, 0)), 10);
}
