extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn abs() {
  assert_eq!(simd::abs(float2(10.0, -3.0)), float2(10.0, 3.0));
  assert_eq!(simd::abs(float3(10.0, -3.0, -9.5)), float3(10.0, 3.0, 9.5));
  assert_eq!(simd::abs(float4(10.0, -3.0, -9.5, 0.0)), float4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(simd::abs(double2(10.0, -3.0)), double2(10.0, 3.0));
  assert_eq!(simd::abs(double3(10.0, -3.0, -9.5)), double3(10.0, 3.0, 9.5));
  assert_eq!(simd::abs(double4(10.0, -3.0, -9.5, 0.0)), double4(10.0, 3.0, 9.5, 0.0));

  assert_eq!(simd::abs(char2(10, -3)), char2(10, 3));
  assert_eq!(simd::abs(char3(10, -3, -9)), char3(10, 3, 9));
  assert_eq!(simd::abs(char4(10, -3, -9, 0)), char4(10, 3, 9, 0));

  assert_eq!(simd::abs(short2(10, -3)), short2(10, 3));
  assert_eq!(simd::abs(short3(10, -3, -9)), short3(10, 3, 9));
  assert_eq!(simd::abs(short4(10, -3, -9, 0)), short4(10, 3, 9, 0));

  assert_eq!(simd::abs(int2(10, -3)), int2(10, 3));
  assert_eq!(simd::abs(int3(10, -3, -9)), int3(10, 3, 9));
  assert_eq!(simd::abs(int4(10, -3, -9, 0)), int4(10, 3, 9, 0));

  assert_eq!(simd::abs(long2(10, -3)), long2(10, 3));
  assert_eq!(simd::abs(long3(10, -3, -9)), long3(10, 3, 9));
  assert_eq!(simd::abs(long4(10, -3, -9, 0)), long4(10, 3, 9, 0));
}

#[test]
fn max() {
  assert_eq!(simd::max(float2(10.0, -2.0), float2(11.0, -3.0)), float2(11.0, -2.0));
  assert_eq!(simd::max(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(simd::max(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(simd::max(double2(10.0, -2.0), double2(11.0, -3.0)), double2(11.0, -2.0));
  assert_eq!(simd::max(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(simd::max(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(11.0, -2.0, 10.0, 1.0));

  assert_eq!(simd::max(char2(10, -2), char2(11, -3)), char2(11, -2));
  assert_eq!(simd::max(char3(10, -2, -9), char3(11, -3, 10)), char3(11, -2, 10));
  assert_eq!(simd::max(char4(10, -2, -9, 1), char4(11, -3, 10, 0)), char4(11, -2, 10, 1));

  assert_eq!(simd::max(uchar2(10, 2), uchar2(11, 3)), uchar2(11, 3));
  assert_eq!(simd::max(uchar3(10, 2, 9), uchar3(11, 3, 10)), uchar3(11, 3, 10));
  assert_eq!(simd::max(uchar4(10, 2, 9, 1), uchar4(11, 3, 10, 0)), uchar4(11, 3, 10, 1));

  assert_eq!(simd::max(short2(10, -2), short2(11, -3)), short2(11, -2));
  assert_eq!(simd::max(short3(10, -2, -9), short3(11, -3, 10)), short3(11, -2, 10));
  assert_eq!(simd::max(short4(10, -2, -9, 1), short4(11, -3, 10, 0)), short4(11, -2, 10, 1));

  assert_eq!(simd::max(ushort2(10, 2), ushort2(11, 3)), ushort2(11, 3));
  assert_eq!(simd::max(ushort3(10, 2, 9), ushort3(11, 3, 10)), ushort3(11, 3, 10));
  assert_eq!(simd::max(ushort4(10, 2, 9, 1), ushort4(11, 3, 10, 0)), ushort4(11, 3, 10, 1));

  assert_eq!(simd::max(int2(10, -2), int2(11, -3)), int2(11, -2));
  assert_eq!(simd::max(int3(10, -2, -9), int3(11, -3, 10)), int3(11, -2, 10));
  assert_eq!(simd::max(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(11, -2, 10, 1));

  assert_eq!(simd::max(uint2(10, 2), uint2(11, 3)), uint2(11, 3));
  assert_eq!(simd::max(uint3(10, 2, 9), uint3(11, 3, 10)), uint3(11, 3, 10));
  assert_eq!(simd::max(uint4(10, 2, 9, 1), uint4(11, 3, 10, 0)), uint4(11, 3, 10, 1));

  assert_eq!(simd::max(long2(10, -2), long2(11, -3)), long2(11, -2));
  assert_eq!(simd::max(long3(10, -2, -9), long3(11, -3, 10)), long3(11, -2, 10));
  assert_eq!(simd::max(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(11, -2, 10, 1));

  assert_eq!(simd::max(ulong2(10, 2), ulong2(11, 3)), ulong2(11, 3));
  assert_eq!(simd::max(ulong3(10, 2, 9), ulong3(11, 3, 10)), ulong3(11, 3, 10));
  assert_eq!(simd::max(ulong4(10, 2, 9, 1), ulong4(11, 3, 10, 0)), ulong4(11, 3, 10, 1));
}

#[test]
fn min() {
  assert_eq!(simd::min(float2(10.0, -2.0), float2(11.0, -3.0)), float2(10.0, -3.0));
  assert_eq!(simd::min(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0)), float3(10.0, -3.0, -9.5));
  assert_eq!(simd::min(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0)), float4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(simd::min(double2(10.0, -2.0), double2(11.0, -3.0)), double2(10.0, -3.0));
  assert_eq!(simd::min(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0)), double3(10.0, -3.0, -9.5));
  assert_eq!(simd::min(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0)), double4(10.0, -3.0, -9.5, 0.0));

  assert_eq!(simd::min(char2(10, -2), char2(11, -3)), char2(10, -3));
  assert_eq!(simd::min(char3(10, -2, -9), char3(11, -3, 10)), char3(10, -3, -9));
  assert_eq!(simd::min(char4(10, -2, -9, 1), char4(11, -3, 10, 0)), char4(10, -3, -9, 0));

  assert_eq!(simd::min(uchar2(10, 2), uchar2(11, 3)), uchar2(10, 2));
  assert_eq!(simd::min(uchar3(10, 2, 9), uchar3(11, 3, 10)), uchar3(10, 2, 9));
  assert_eq!(simd::min(uchar4(10, 2, 9, 1), uchar4(11, 3, 10, 0)), uchar4(10, 2, 9, 0));

  assert_eq!(simd::min(short2(10, -2), short2(11, -3)), short2(10, -3));
  assert_eq!(simd::min(short3(10, -2, -9), short3(11, -3, 10)), short3(10, -3, -9));
  assert_eq!(simd::min(short4(10, -2, -9, 1), short4(11, -3, 10, 0)), short4(10, -3, -9, 0));

  assert_eq!(simd::min(ushort2(10, 2), ushort2(11, 3)), ushort2(10, 2));
  assert_eq!(simd::min(ushort3(10, 2, 9), ushort3(11, 3, 10)), ushort3(10, 2, 9));
  assert_eq!(simd::min(ushort4(10, 2, 9, 1), ushort4(11, 3, 10, 0)), ushort4(10, 2, 9, 0));

  assert_eq!(simd::min(int2(10, -2), int2(11, -3)), int2(10, -3));
  assert_eq!(simd::min(int3(10, -2, -9), int3(11, -3, 10)), int3(10, -3, -9));
  assert_eq!(simd::min(int4(10, -2, -9, 1), int4(11, -3, 10, 0)), int4(10, -3, -9, 0));

  assert_eq!(simd::min(uint2(10, 2), uint2(11, 3)), uint2(10, 2));
  assert_eq!(simd::min(uint3(10, 2, 9), uint3(11, 3, 10)), uint3(10, 2, 9));
  assert_eq!(simd::min(uint4(10, 2, 9, 1), uint4(11, 3, 10, 0)), uint4(10, 2, 9, 0));

  assert_eq!(simd::min(long2(10, -2), long2(11, -3)), long2(10, -3));
  assert_eq!(simd::min(long3(10, -2, -9), long3(11, -3, 10)), long3(10, -3, -9));
  assert_eq!(simd::min(long4(10, -2, -9, 1), long4(11, -3, 10, 0)), long4(10, -3, -9, 0));

  assert_eq!(simd::min(ulong2(10, 2), ulong2(11, 3)), ulong2(10, 2));
  assert_eq!(simd::min(ulong3(10, 2, 9), ulong3(11, 3, 10)), ulong3(10, 2, 9));
  assert_eq!(simd::min(ulong4(10, 2, 9, 1), ulong4(11, 3, 10, 0)), ulong4(10, 2, 9, 0));
}

#[test]
fn clamp() {
  assert_eq!(simd::clamp(float2(10.0, -2.0), float2(11.0, -3.0), float2(11.0, -1.0)), float2(11.0, -2.0));
  assert_eq!(simd::clamp(float3(10.0, -2.0, -9.5), float3(11.0, -3.0, 10.0), float3(11.0, -1.0, 11.0)), float3(11.0, -2.0, 10.0));
  assert_eq!(simd::clamp(float4(10.0, -2.0, -9.5, 1.0), float4(11.0, -3.0, 10.0, 0.0), float4(11.0, -1.0, 10.0, 0.5)), float4(11.0, -2.0, 10.0, 0.5));

  assert_eq!(simd::clamp(double2(10.0, -2.0), double2(11.0, -3.0), double2(11.0, -1.0)), double2(11.0, -2.0));
  assert_eq!(simd::clamp(double3(10.0, -2.0, -9.5), double3(11.0, -3.0, 10.0), double3(11.0, -1.0, 11.0)), double3(11.0, -2.0, 10.0));
  assert_eq!(simd::clamp(double4(10.0, -2.0, -9.5, 1.0), double4(11.0, -3.0, 10.0, 0.0), double4(11.0, -1.0, 10.0, 0.5)), double4(11.0, -2.0, 10.0, 0.5));
}
