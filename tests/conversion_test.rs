extern crate aventine_simd;

use aventine_simd::*;

#[test]
fn to_char() {
  assert_eq!(simd::to_char(float2(10.0, -3.0)), char2(10, -3));
  assert_eq!(simd::to_char(float3(10.0, -3.0, -9.5)), char3(10, -3, -9));
  assert_eq!(simd::to_char(float4(10.0, -3.0, -9.5, 400.0)), char4(10, -3, -9, -112));

  assert_eq!(simd::to_char(double2(10.0, -3.0)), char2(10, -3));
  assert_eq!(simd::to_char(double3(10.0, -3.0, -9.5)), char3(10, -3, -9));
  assert_eq!(simd::to_char(double4(10.0, -3.0, -9.5, 400.0)), char4(10, -3, -9, -112));

  assert_eq!(simd::to_char(char2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char(char3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char(char4(10, -3, -9, 0)), char4(10, -3, -9, 0));

  assert_eq!(simd::to_char(uchar2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char(uchar3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char(uchar4(10, 3, 9, 200)), char4(10, 3, 9, -56));

  assert_eq!(simd::to_char(short2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char(short3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char(short4(10, -3, -9, 400)), char4(10, -3, -9, -112));

  assert_eq!(simd::to_char(ushort2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char(ushort3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char(ushort4(10, 3, 9, 200)), char4(10, 3, 9, -56));

  assert_eq!(simd::to_char(int2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char(int3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char(int4(10, -3, -9, 400)), char4(10, -3, -9, -112));

  assert_eq!(simd::to_char(uint2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char(uint3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char(uint4(10, 3, 9, 200)), char4(10, 3, 9, -56));

  assert_eq!(simd::to_char(long2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char(long3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char(long4(10, -3, -9, 400)), char4(10, -3, -9, -112));

  assert_eq!(simd::to_char(ulong2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char(ulong3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char(ulong4(10, 3, 9, 200)), char4(10, 3, 9, -56));
}

#[test]
fn to_uchar() {
  assert_eq!(simd::to_uchar(float2(10.0, -3.0)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(float3(10.0, -3.0, -9.5)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(float4(10.0, -3.0, -9.5, 400.0)), uchar4(10, 253, 247, 144));

  assert_eq!(simd::to_uchar(double2(10.0, -3.0)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(double3(10.0, -3.0, -9.5)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(double4(10.0, -3.0, -9.5, 400.0)), uchar4(10, 253, 247, 144));

  assert_eq!(simd::to_uchar(char2(10, -3)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(char3(10, -3, -9)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(char4(10, -3, -9, 0)), uchar4(10, 253, 247, 0));

  assert_eq!(simd::to_uchar(uchar2(10, 253)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(uchar3(10, 253, 247)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(uchar4(10, 253, 247, 200)), uchar4(10, 253, 247, 200));

  assert_eq!(simd::to_uchar(short2(10, -3)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(short3(10, -3, -9)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(short4(10, -3, -9, 400)), uchar4(10, 253, 247, 144));

  assert_eq!(simd::to_uchar(ushort2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar(ushort3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar(ushort4(10, 3, 9, 400)), uchar4(10, 3, 9, 144));

  assert_eq!(simd::to_uchar(int2(10, -3)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(int3(10, -3, -9)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(int4(10, -3, -9, 400)), uchar4(10, 253, 247, 144));

  assert_eq!(simd::to_uchar(uint2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar(uint3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar(uint4(10, 3, 9, 400)), uchar4(10, 3, 9, 144));

  assert_eq!(simd::to_uchar(long2(10, -3)), uchar2(10, 253));
  assert_eq!(simd::to_uchar(long3(10, -3, -9)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar(long4(10, -3, -9, 400)), uchar4(10, 253, 247, 144));

  assert_eq!(simd::to_uchar(ulong2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar(ulong3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar(ulong4(10, 3, 9, 400)), uchar4(10, 3, 9, 144));
}

#[test]
fn to_char_sat() {
  assert_eq!(simd::to_char_sat(float2(10.0, -3.0)), char2(10, -3));
  assert_eq!(simd::to_char_sat(float3(10.0, -3.0, -9.5)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(float4(10.0, -3.0, -9.5, 400.0)), char4(10, -3, -9, 127));

  assert_eq!(simd::to_char_sat(double2(10.0, -3.0)), char2(10, -3));
  assert_eq!(simd::to_char_sat(double3(10.0, -3.0, -9.5)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(double4(10.0, -3.0, -9.5, 400.0)), char4(10, -3, -9, 127));

  assert_eq!(simd::to_char_sat(char2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char_sat(char3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(char4(10, -3, -9, 0)), char4(10, -3, -9, 0));

  assert_eq!(simd::to_char_sat(uchar2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char_sat(uchar3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char_sat(uchar4(10, 3, 9, 200)), char4(10, 3, 9, 127));

  assert_eq!(simd::to_char_sat(short2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char_sat(short3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(short4(10, -3, -9, 400)), char4(10, -3, -9, 127));

  assert_eq!(simd::to_char_sat(ushort2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char_sat(ushort3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char_sat(ushort4(10, 3, 9, 200)), char4(10, 3, 9, 127));

  assert_eq!(simd::to_char_sat(int2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char_sat(int3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(int4(10, -3, -9, 400)), char4(10, -3, -9, 127));

  assert_eq!(simd::to_char_sat(uint2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char_sat(uint3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char_sat(uint4(10, 3, 9, 200)), char4(10, 3, 9, 127));

  assert_eq!(simd::to_char_sat(long2(10, -3)), char2(10, -3));
  assert_eq!(simd::to_char_sat(long3(10, -3, -9)), char3(10, -3, -9));
  assert_eq!(simd::to_char_sat(long4(10, -3, -9, 400)), char4(10, -3, -9, 127));

  assert_eq!(simd::to_char_sat(ulong2(10, 3)), char2(10, 3));
  assert_eq!(simd::to_char_sat(ulong3(10, 3, 9)), char3(10, 3, 9));
  assert_eq!(simd::to_char_sat(ulong4(10, 3, 9, 200)), char4(10, 3, 9, 127));
}

#[test]
fn to_uchar_sat() {
  assert_eq!(simd::to_uchar_sat(float2(10.0, -3.0)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(float3(10.0, -3.0, -9.5)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(float4(10.0, -3.0, -9.5, 400.0)), uchar4(10, 0, 0, 255));

  assert_eq!(simd::to_uchar_sat(double2(10.0, -3.0)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(double3(10.0, -3.0, -9.5)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(double4(10.0, -3.0, -9.5, 400.0)), uchar4(10, 0, 0, 255));

  assert_eq!(simd::to_uchar_sat(char2(10, -3)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(char3(10, -3, -9)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(char4(10, -3, -9, 0)), uchar4(10, 0, 0, 0));

  assert_eq!(simd::to_uchar_sat(uchar2(10, 253)), uchar2(10, 253));
  assert_eq!(simd::to_uchar_sat(uchar3(10, 253, 247)), uchar3(10, 253, 247));
  assert_eq!(simd::to_uchar_sat(uchar4(10, 253, 247, 200)), uchar4(10, 253, 247, 200));

  assert_eq!(simd::to_uchar_sat(short2(10, -3)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(short3(10, -3, -9)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(short4(10, -3, -9, 400)), uchar4(10, 0, 0, 255));

  assert_eq!(simd::to_uchar_sat(ushort2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar_sat(ushort3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar_sat(ushort4(10, 3, 9, 400)), uchar4(10, 3, 9, 255));

  assert_eq!(simd::to_uchar_sat(int2(10, -3)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(int3(10, -3, -9)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(int4(10, -3, -9, 400)), uchar4(10, 0, 0, 255));

  assert_eq!(simd::to_uchar_sat(uint2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar_sat(uint3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar_sat(uint4(10, 3, 9, 400)), uchar4(10, 3, 9, 255));

  assert_eq!(simd::to_uchar_sat(long2(10, -3)), uchar2(10, 0));
  assert_eq!(simd::to_uchar_sat(long3(10, -3, -9)), uchar3(10, 0, 0));
  assert_eq!(simd::to_uchar_sat(long4(10, -3, -9, 400)), uchar4(10, 0, 0, 255));

  assert_eq!(simd::to_uchar_sat(ulong2(10, 3)), uchar2(10, 3));
  assert_eq!(simd::to_uchar_sat(ulong3(10, 3, 9)), uchar3(10, 3, 9));
  assert_eq!(simd::to_uchar_sat(ulong4(10, 3, 9, 400)), uchar4(10, 3, 9, 255));
}

#[test]
fn to_short() {
  assert_eq!(simd::to_short(float2(10.0, -3.0)), short2(10, -3));
  assert_eq!(simd::to_short(float3(10.0, -3.0, -9.5)), short3(10, -3, -9));
  assert_eq!(simd::to_short(float4(10.0, -3.0, -9.5, 40000.0)), short4(10, -3, -9, -25536));
  
  assert_eq!(simd::to_short(double2(10.0, -3.0)), short2(10, -3));
  assert_eq!(simd::to_short(double3(10.0, -3.0, -9.5)), short3(10, -3, -9));
  assert_eq!(simd::to_short(double4(10.0, -3.0, -9.5, 40000.0)), short4(10, -3, -9, -25536));
  
  assert_eq!(simd::to_short(char2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short(char3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short(char4(10, -3, -9, 0)), short4(10, -3, -9, 0));

  assert_eq!(simd::to_short(uchar2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short(uchar3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short(uchar4(10, 3, 9, 200)), short4(10, 3, 9, 200));

  assert_eq!(simd::to_short(short2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short(short3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short(short4(10, -3, -9, 200)), short4(10, -3, -9, 200));

  assert_eq!(simd::to_short(ushort2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short(ushort3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short(ushort4(10, 3, 9, 40000)), short4(10, 3, 9, -25536));

  assert_eq!(simd::to_short(int2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short(int3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short(int4(10, -3, -9, 40000)), short4(10, -3, -9, -25536));

  assert_eq!(simd::to_short(uint2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short(uint3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short(uint4(10, 3, 9, 40000)), short4(10, 3, 9, -25536));

  assert_eq!(simd::to_short(long2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short(long3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short(long4(10, -3, -9, 40000)), short4(10, -3, -9, -25536));

  assert_eq!(simd::to_short(ulong2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short(ulong3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short(ulong4(10, 3, 9, 40000)), short4(10, 3, 9, -25536));
}

#[test]
fn to_ushort() {
  assert_eq!(simd::to_ushort(float2(10.0, -3.0)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(float3(10.0, -3.0, -9.5)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(float4(10.0, -3.0, -9.5, 400.0)), ushort4(10, 65533, 65527, 400));

  assert_eq!(simd::to_ushort(double2(10.0, -3.0)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(double3(10.0, -3.0, -9.5)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(double4(10.0, -3.0, -9.5, 400.0)), ushort4(10, 65533, 65527, 400));

  assert_eq!(simd::to_ushort(char2(10, -3)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(char3(10, -3, -9)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(char4(10, -3, -9, 0)), ushort4(10, 65533, 65527, 0));

  assert_eq!(simd::to_ushort(uchar2(10, 253)), ushort2(10, 253));
  assert_eq!(simd::to_ushort(uchar3(10, 253, 247)), ushort3(10, 253, 247));
  assert_eq!(simd::to_ushort(uchar4(10, 253, 247, 200)), ushort4(10, 253, 247, 200));

  assert_eq!(simd::to_ushort(short2(10, -3)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(short3(10, -3, -9)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(short4(10, -3, -9, 400)), ushort4(10, 65533, 65527, 400));

  assert_eq!(simd::to_ushort(ushort2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort(ushort3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort(ushort4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));

  assert_eq!(simd::to_ushort(int2(10, -3)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(int3(10, -3, -9)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(int4(10, -3, -9, 400)), ushort4(10, 65533, 65527, 400));

  assert_eq!(simd::to_ushort(uint2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort(uint3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort(uint4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));

  assert_eq!(simd::to_ushort(long2(10, -3)), ushort2(10, 65533));
  assert_eq!(simd::to_ushort(long3(10, -3, -9)), ushort3(10, 65533, 65527));
  assert_eq!(simd::to_ushort(long4(10, -3, -9, 400)), ushort4(10, 65533, 65527, 400));

  assert_eq!(simd::to_ushort(ulong2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort(ulong3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort(ulong4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));
}

#[test]
fn to_short_sat() {
  assert_eq!(simd::to_short_sat(float2(10.0, -3.0)), short2(10, -3));
  assert_eq!(simd::to_short_sat(float3(10.0, -3.0, -9.5)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(float4(10.0, -3.0, -9.5, 40000.0)), short4(10, -3, -9, 32767));

  assert_eq!(simd::to_short_sat(double2(10.0, -3.0)), short2(10, -3));
  assert_eq!(simd::to_short_sat(double3(10.0, -3.0, -9.5)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(double4(10.0, -3.0, -9.5, 40000.0)), short4(10, -3, -9, 32767));

  assert_eq!(simd::to_short_sat(char2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short_sat(char3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(char4(10, -3, -9, 0)), short4(10, -3, -9, 0));

  assert_eq!(simd::to_short_sat(uchar2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short_sat(uchar3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short_sat(uchar4(10, 3, 9, 200)), short4(10, 3, 9, 200));

  assert_eq!(simd::to_short_sat(short2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short_sat(short3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(short4(10, -3, -9, 400)), short4(10, -3, -9, 400));

  assert_eq!(simd::to_short_sat(ushort2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short_sat(ushort3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short_sat(ushort4(10, 3, 9, 400)), short4(10, 3, 9, 400));

  assert_eq!(simd::to_short_sat(int2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short_sat(int3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(int4(10, -3, -9, 400)), short4(10, -3, -9, 400));

  assert_eq!(simd::to_short_sat(uint2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short_sat(uint3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short_sat(uint4(10, 3, 9, 200)), short4(10, 3, 9, 200));

  assert_eq!(simd::to_short_sat(long2(10, -3)), short2(10, -3));
  assert_eq!(simd::to_short_sat(long3(10, -3, -9)), short3(10, -3, -9));
  assert_eq!(simd::to_short_sat(long4(10, -3, -9, 400)), short4(10, -3, -9, 400));

  assert_eq!(simd::to_short_sat(ulong2(10, 3)), short2(10, 3));
  assert_eq!(simd::to_short_sat(ulong3(10, 3, 9)), short3(10, 3, 9));
  assert_eq!(simd::to_short_sat(ulong4(10, 3, 9, 200)), short4(10, 3, 9, 200));
}

#[test]
fn to_ushort_sat() {
  assert_eq!(simd::to_ushort_sat(float2(10.0, -3.0)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(float3(10.0, -3.0, -9.5)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(float4(10.0, -3.0, -9.5, 400.0)), ushort4(10, 0, 0, 400));

  assert_eq!(simd::to_ushort_sat(double2(10.0, -3.0)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(double3(10.0, -3.0, -9.5)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(double4(10.0, -3.0, -9.5, 400.0)), ushort4(10, 0, 0, 400));

  assert_eq!(simd::to_ushort_sat(char2(10, -3)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(char3(10, -3, -9)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(char4(10, -3, -9, 0)), ushort4(10, 0, 0, 0));

  assert_eq!(simd::to_ushort_sat(uchar2(10, 253)), ushort2(10, 253));
  assert_eq!(simd::to_ushort_sat(uchar3(10, 253, 247)), ushort3(10, 253, 247));
  assert_eq!(simd::to_ushort_sat(uchar4(10, 253, 247, 200)), ushort4(10, 253, 247, 200));

  assert_eq!(simd::to_ushort_sat(short2(10, -3)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(short3(10, -3, -9)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(short4(10, -3, -9, 400)), ushort4(10, 0, 0, 400));

  assert_eq!(simd::to_ushort_sat(ushort2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort_sat(ushort3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort_sat(ushort4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));

  assert_eq!(simd::to_ushort_sat(int2(10, -3)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(int3(10, -3, -9)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(int4(10, -3, -9, 400)), ushort4(10, 0, 0, 400));

  assert_eq!(simd::to_ushort_sat(uint2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort_sat(uint3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort_sat(uint4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));

  assert_eq!(simd::to_ushort_sat(long2(10, -3)), ushort2(10, 0));
  assert_eq!(simd::to_ushort_sat(long3(10, -3, -9)), ushort3(10, 0, 0));
  assert_eq!(simd::to_ushort_sat(long4(10, -3, -9, 400)), ushort4(10, 0, 0, 400));

  assert_eq!(simd::to_ushort_sat(ulong2(10, 3)), ushort2(10, 3));
  assert_eq!(simd::to_ushort_sat(ulong3(10, 3, 9)), ushort3(10, 3, 9));
  assert_eq!(simd::to_ushort_sat(ulong4(10, 3, 9, 400)), ushort4(10, 3, 9, 400));
}

#[test]
fn to_int() {
  assert_eq!(simd::to_int(float2(10.0, -3.0)), int2(10, -3));
  assert_eq!(simd::to_int(float3(10.0, -3.0, -9.5)), int3(10, -3, -9));
  assert_eq!(simd::to_int(float4(10.0, -3.0, -9.5, 40000.0)), int4(10, -3, -9, 40000));
  
  assert_eq!(simd::to_int(double2(10.0, -3.0)), int2(10, -3));
  assert_eq!(simd::to_int(double3(10.0, -3.0, -9.5)), int3(10, -3, -9));
  assert_eq!(simd::to_int(double4(10.0, -3.0, -9.5, 40000.0)), int4(10, -3, -9, 40000));
  
  assert_eq!(simd::to_int(char2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int(char3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int(char4(10, -3, -9, 0)), int4(10, -3, -9, 0));

  assert_eq!(simd::to_int(uchar2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int(uchar3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int(uchar4(10, 3, 9, 200)), int4(10, 3, 9, 200));

  assert_eq!(simd::to_int(short2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int(short3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int(short4(10, -3, -9, 200)), int4(10, -3, -9, 200));

  assert_eq!(simd::to_int(ushort2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int(ushort3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int(ushort4(10, 3, 9, 40000)), int4(10, 3, 9, 40000));

  assert_eq!(simd::to_int(int2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int(int3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int(int4(10, -3, -9, 40000)), int4(10, -3, -9, 40000));

  assert_eq!(simd::to_int(uint2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int(uint3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int(uint4(10, 3, 9, 40000)), int4(10, 3, 9, 40000));

  assert_eq!(simd::to_int(long2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int(long3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int(long4(10, -3, -9, 40000)), int4(10, -3, -9, 40000));

  assert_eq!(simd::to_int(ulong2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int(ulong3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int(ulong4(10, 3, 9, 40000)), int4(10, 3, 9, 40000));
}

#[test]
fn to_uint() {
  assert_eq!(simd::to_uint(float2(10.0, -3.0)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(float3(10.0, -3.0, -9.5)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(float4(10.0, -3.0, -9.5, 400.0)), uint4(10, 4294967293, 4294967287, 400));

  assert_eq!(simd::to_uint(double2(10.0, -3.0)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(double3(10.0, -3.0, -9.5)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(double4(10.0, -3.0, -9.5, 400.0)), uint4(10, 4294967293, 4294967287, 400));

  assert_eq!(simd::to_uint(char2(10, -3)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(char3(10, -3, -9)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(char4(10, -3, -9, 0)), uint4(10, 4294967293, 4294967287, 0));

  assert_eq!(simd::to_uint(uchar2(10, 253)), uint2(10, 253));
  assert_eq!(simd::to_uint(uchar3(10, 253, 247)), uint3(10, 253, 247));
  assert_eq!(simd::to_uint(uchar4(10, 253, 247, 200)), uint4(10, 253, 247, 200));

  assert_eq!(simd::to_uint(short2(10, -3)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(short3(10, -3, -9)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(short4(10, -3, -9, 400)), uint4(10, 4294967293, 4294967287, 400));

  assert_eq!(simd::to_uint(ushort2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint(ushort3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint(ushort4(10, 3, 9, 400)), uint4(10, 3, 9, 400));

  assert_eq!(simd::to_uint(int2(10, -3)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(int3(10, -3, -9)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(int4(10, -3, -9, 400)), uint4(10, 4294967293, 4294967287, 400));

  assert_eq!(simd::to_uint(uint2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint(uint3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint(uint4(10, 3, 9, 400)), uint4(10, 3, 9, 400));

  assert_eq!(simd::to_uint(long2(10, -3)), uint2(10, 4294967293));
  assert_eq!(simd::to_uint(long3(10, -3, -9)), uint3(10, 4294967293, 4294967287));
  assert_eq!(simd::to_uint(long4(10, -3, -9, 400)), uint4(10, 4294967293, 4294967287, 400));

  assert_eq!(simd::to_uint(ulong2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint(ulong3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint(ulong4(10, 3, 9, 400)), uint4(10, 3, 9, 400));
}

#[test]
fn to_int_sat() {
  assert_eq!(simd::to_int_sat(float2(10.0, -3.0)), int2(10, -3));
  assert_eq!(simd::to_int_sat(float3(10.0, -3.0, -9.5)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(float4(10.0, -3.0, -9.5, 40000.0)), int4(10, -3, -9, 40000));

  assert_eq!(simd::to_int_sat(double2(10.0, -3.0)), int2(10, -3));
  assert_eq!(simd::to_int_sat(double3(10.0, -3.0, -9.5)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(double4(10.0, -3.0, -9.5, 40000.0)), int4(10, -3, -9, 40000));

  assert_eq!(simd::to_int_sat(char2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int_sat(char3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(char4(10, -3, -9, 0)), int4(10, -3, -9, 0));

  assert_eq!(simd::to_int_sat(uchar2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int_sat(uchar3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int_sat(uchar4(10, 3, 9, 200)), int4(10, 3, 9, 200));

  assert_eq!(simd::to_int_sat(short2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int_sat(short3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(short4(10, -3, -9, 400)), int4(10, -3, -9, 400));

  assert_eq!(simd::to_int_sat(ushort2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int_sat(ushort3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int_sat(ushort4(10, 3, 9, 400)), int4(10, 3, 9, 400));

  assert_eq!(simd::to_int_sat(int2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int_sat(int3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(int4(10, -3, -9, 400)), int4(10, -3, -9, 400));

  assert_eq!(simd::to_int_sat(uint2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int_sat(uint3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int_sat(uint4(10, 3, 9, 200)), int4(10, 3, 9, 200));

  assert_eq!(simd::to_int_sat(long2(10, -3)), int2(10, -3));
  assert_eq!(simd::to_int_sat(long3(10, -3, -9)), int3(10, -3, -9));
  assert_eq!(simd::to_int_sat(long4(10, -3, -9, 400)), int4(10, -3, -9, 400));

  assert_eq!(simd::to_int_sat(ulong2(10, 3)), int2(10, 3));
  assert_eq!(simd::to_int_sat(ulong3(10, 3, 9)), int3(10, 3, 9));
  assert_eq!(simd::to_int_sat(ulong4(10, 3, 9, 200)), int4(10, 3, 9, 200));
}

#[test]
fn to_uint_sat() {
  assert_eq!(simd::to_uint_sat(float2(10.0, -3.0)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(float3(10.0, -3.0, -9.5)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(float4(10.0, -3.0, -9.5, 400.0)), uint4(10, 0, 0, 400));

  assert_eq!(simd::to_uint_sat(double2(10.0, -3.0)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(double3(10.0, -3.0, -9.5)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(double4(10.0, -3.0, -9.5, 400.0)), uint4(10, 0, 0, 400));

  assert_eq!(simd::to_uint_sat(char2(10, -3)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(char3(10, -3, -9)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(char4(10, -3, -9, 0)), uint4(10, 0, 0, 0));

  assert_eq!(simd::to_uint_sat(uchar2(10, 253)), uint2(10, 253));
  assert_eq!(simd::to_uint_sat(uchar3(10, 253, 247)), uint3(10, 253, 247));
  assert_eq!(simd::to_uint_sat(uchar4(10, 253, 247, 200)), uint4(10, 253, 247, 200));

  assert_eq!(simd::to_uint_sat(short2(10, -3)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(short3(10, -3, -9)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(short4(10, -3, -9, 400)), uint4(10, 0, 0, 400));

  assert_eq!(simd::to_uint_sat(ushort2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint_sat(ushort3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint_sat(ushort4(10, 3, 9, 400)), uint4(10, 3, 9, 400));

  assert_eq!(simd::to_uint_sat(int2(10, -3)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(int3(10, -3, -9)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(int4(10, -3, -9, 400)), uint4(10, 0, 0, 400));

  assert_eq!(simd::to_uint_sat(uint2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint_sat(uint3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint_sat(uint4(10, 3, 9, 400)), uint4(10, 3, 9, 400));

  assert_eq!(simd::to_uint_sat(long2(10, -3)), uint2(10, 0));
  assert_eq!(simd::to_uint_sat(long3(10, -3, -9)), uint3(10, 0, 0));
  assert_eq!(simd::to_uint_sat(long4(10, -3, -9, 400)), uint4(10, 0, 0, 400));

  assert_eq!(simd::to_uint_sat(ulong2(10, 3)), uint2(10, 3));
  assert_eq!(simd::to_uint_sat(ulong3(10, 3, 9)), uint3(10, 3, 9));
  assert_eq!(simd::to_uint_sat(ulong4(10, 3, 9, 400)), uint4(10, 3, 9, 400));
}

#[test]
fn to_long() {
  assert_eq!(simd::to_long(float2(10.0, -3.0)), long2(10, -3));
  assert_eq!(simd::to_long(float3(10.0, -3.0, -9.5)), long3(10, -3, -9));
  assert_eq!(simd::to_long(float4(10.0, -3.0, -9.5, 40000.0)), long4(10, -3, -9, 40000));
  
  assert_eq!(simd::to_long(double2(10.0, -3.0)), long2(10, -3));
  assert_eq!(simd::to_long(double3(10.0, -3.0, -9.5)), long3(10, -3, -9));
  assert_eq!(simd::to_long(double4(10.0, -3.0, -9.5, 40000.0)), long4(10, -3, -9, 40000));
  
  assert_eq!(simd::to_long(char2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long(char3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long(char4(10, -3, -9, 0)), long4(10, -3, -9, 0));

  assert_eq!(simd::to_long(uchar2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long(uchar3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long(uchar4(10, 3, 9, 200)), long4(10, 3, 9, 200));

  assert_eq!(simd::to_long(short2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long(short3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long(short4(10, -3, -9, 200)), long4(10, -3, -9, 200));

  assert_eq!(simd::to_long(ushort2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long(ushort3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long(ushort4(10, 3, 9, 40000)), long4(10, 3, 9, 40000));

  assert_eq!(simd::to_long(int2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long(int3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long(int4(10, -3, -9, 40000)), long4(10, -3, -9, 40000));

  assert_eq!(simd::to_long(uint2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long(uint3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long(uint4(10, 3, 9, 40000)), long4(10, 3, 9, 40000));

  assert_eq!(simd::to_long(long2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long(long3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long(long4(10, -3, -9, 40000)), long4(10, -3, -9, 40000));

  assert_eq!(simd::to_long(ulong2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long(ulong3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long(ulong4(10, 3, 9, 40000)), long4(10, 3, 9, 40000));
}

#[test]
fn to_ulong() {
  assert_eq!(simd::to_ulong(float2(10.0, -3.0)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(float3(10.0, -3.0, -9.5)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(float4(10.0, -3.0, -9.5, 400.0)), ulong4(10, 18446744073709551613, 18446744073709551607, 400));

  assert_eq!(simd::to_ulong(double2(10.0, -3.0)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(double3(10.0, -3.0, -9.5)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(double4(10.0, -3.0, -9.5, 400.0)), ulong4(10, 18446744073709551613, 18446744073709551607, 400));

  assert_eq!(simd::to_ulong(char2(10, -3)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(char3(10, -3, -9)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(char4(10, -3, -9, 0)), ulong4(10, 18446744073709551613, 18446744073709551607, 0));

  assert_eq!(simd::to_ulong(uchar2(10, 253)), ulong2(10, 253));
  assert_eq!(simd::to_ulong(uchar3(10, 253, 247)), ulong3(10, 253, 247));
  assert_eq!(simd::to_ulong(uchar4(10, 253, 247, 200)), ulong4(10, 253, 247, 200));

  assert_eq!(simd::to_ulong(short2(10, -3)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(short3(10, -3, -9)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(short4(10, -3, -9, 400)), ulong4(10, 18446744073709551613, 18446744073709551607, 400));

  assert_eq!(simd::to_ulong(ushort2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong(ushort3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong(ushort4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));

  assert_eq!(simd::to_ulong(int2(10, -3)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(int3(10, -3, -9)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(int4(10, -3, -9, 400)), ulong4(10, 18446744073709551613, 18446744073709551607, 400));

  assert_eq!(simd::to_ulong(uint2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong(uint3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong(uint4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));

  assert_eq!(simd::to_ulong(long2(10, -3)), ulong2(10, 18446744073709551613));
  assert_eq!(simd::to_ulong(long3(10, -3, -9)), ulong3(10, 18446744073709551613, 18446744073709551607));
  assert_eq!(simd::to_ulong(long4(10, -3, -9, 400)), ulong4(10, 18446744073709551613, 18446744073709551607, 400));

  assert_eq!(simd::to_ulong(ulong2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong(ulong3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong(ulong4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));
}

#[test]
fn to_long_sat() {
  assert_eq!(simd::to_long_sat(float2(10.0, -3.0)), long2(10, -3));
  assert_eq!(simd::to_long_sat(float3(10.0, -3.0, -9.5)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(float4(10.0, -3.0, -9.5, 40000.0)), long4(10, -3, -9, 40000));

  assert_eq!(simd::to_long_sat(double2(10.0, -3.0)), long2(10, -3));
  assert_eq!(simd::to_long_sat(double3(10.0, -3.0, -9.5)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(double4(10.0, -3.0, -9.5, 40000.0)), long4(10, -3, -9, 40000));

  assert_eq!(simd::to_long_sat(char2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long_sat(char3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(char4(10, -3, -9, 0)), long4(10, -3, -9, 0));

  assert_eq!(simd::to_long_sat(uchar2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long_sat(uchar3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long_sat(uchar4(10, 3, 9, 200)), long4(10, 3, 9, 200));

  assert_eq!(simd::to_long_sat(short2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long_sat(short3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(short4(10, -3, -9, 400)), long4(10, -3, -9, 400));

  assert_eq!(simd::to_long_sat(ushort2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long_sat(ushort3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long_sat(ushort4(10, 3, 9, 400)), long4(10, 3, 9, 400));

  assert_eq!(simd::to_long_sat(int2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long_sat(int3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(int4(10, -3, -9, 400)), long4(10, -3, -9, 400));

  assert_eq!(simd::to_long_sat(uint2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long_sat(uint3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long_sat(uint4(10, 3, 9, 200)), long4(10, 3, 9, 200));

  assert_eq!(simd::to_long_sat(long2(10, -3)), long2(10, -3));
  assert_eq!(simd::to_long_sat(long3(10, -3, -9)), long3(10, -3, -9));
  assert_eq!(simd::to_long_sat(long4(10, -3, -9, 400)), long4(10, -3, -9, 400));

  assert_eq!(simd::to_long_sat(ulong2(10, 3)), long2(10, 3));
  assert_eq!(simd::to_long_sat(ulong3(10, 3, 9)), long3(10, 3, 9));
  assert_eq!(simd::to_long_sat(ulong4(10, 3, 9, 200)), long4(10, 3, 9, 200));
}

#[test]
fn to_ulong_sat() {
  assert_eq!(simd::to_ulong_sat(float2(10.0, -3.0)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(float3(10.0, -3.0, -9.5)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(float4(10.0, -3.0, -9.5, 400.0)), ulong4(10, 0, 0, 400));

  assert_eq!(simd::to_ulong_sat(double2(10.0, -3.0)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(double3(10.0, -3.0, -9.5)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(double4(10.0, -3.0, -9.5, 400.0)), ulong4(10, 0, 0, 400));

  assert_eq!(simd::to_ulong_sat(char2(10, -3)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(char3(10, -3, -9)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(char4(10, -3, -9, 0)), ulong4(10, 0, 0, 0));

  assert_eq!(simd::to_ulong_sat(uchar2(10, 253)), ulong2(10, 253));
  assert_eq!(simd::to_ulong_sat(uchar3(10, 253, 247)), ulong3(10, 253, 247));
  assert_eq!(simd::to_ulong_sat(uchar4(10, 253, 247, 200)), ulong4(10, 253, 247, 200));

  assert_eq!(simd::to_ulong_sat(short2(10, -3)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(short3(10, -3, -9)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(short4(10, -3, -9, 400)), ulong4(10, 0, 0, 400));

  assert_eq!(simd::to_ulong_sat(ushort2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong_sat(ushort3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong_sat(ushort4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));

  assert_eq!(simd::to_ulong_sat(int2(10, -3)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(int3(10, -3, -9)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(int4(10, -3, -9, 400)), ulong4(10, 0, 0, 400));

  assert_eq!(simd::to_ulong_sat(uint2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong_sat(uint3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong_sat(uint4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));

  assert_eq!(simd::to_ulong_sat(long2(10, -3)), ulong2(10, 0));
  assert_eq!(simd::to_ulong_sat(long3(10, -3, -9)), ulong3(10, 0, 0));
  assert_eq!(simd::to_ulong_sat(long4(10, -3, -9, 400)), ulong4(10, 0, 0, 400));

  assert_eq!(simd::to_ulong_sat(ulong2(10, 3)), ulong2(10, 3));
  assert_eq!(simd::to_ulong_sat(ulong3(10, 3, 9)), ulong3(10, 3, 9));
  assert_eq!(simd::to_ulong_sat(ulong4(10, 3, 9, 400)), ulong4(10, 3, 9, 400));
}

#[test]
fn to_float() {
  assert_eq!(simd::to_float(float2(10.0, -3.0)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(float3(10.0, -3.0, -9.5)), float3(10.0, -3.0, -9.5));
  assert_eq!(simd::to_float(float4(10.0, -3.0, -9.5, 400.0)), float4(10.0, -3.0, -9.5, 400.0));

  assert_eq!(simd::to_float(double2(10.0, -3.0)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(double3(10.0, -3.0, -9.5)), float3(10.0, -3.0, -9.5));
  assert_eq!(simd::to_float(double4(10.0, -3.0, -9.5, 400.0)), float4(10.0, -3.0, -9.5, 400.0));

  assert_eq!(simd::to_float(char2(10, -3)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(char3(10, -3, -9)), float3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_float(char4(10, -3, -9, 0)), float4(10.0, -3.0, -9.0, 0.0));

  assert_eq!(simd::to_float(uchar2(10, 3)), float2(10.0, 3.0));
  assert_eq!(simd::to_float(uchar3(10, 3, 9)), float3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_float(uchar4(10, 3, 9, 200)), float4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_float(short2(10, -3)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(short3(10, -3, -9)), float3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_float(short4(10, -3, -9, 400)), float4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_float(ushort2(10, 3)), float2(10.0, 3.0));
  assert_eq!(simd::to_float(ushort3(10, 3, 9)), float3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_float(ushort4(10, 3, 9, 200)), float4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_float(int2(10, -3)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(int3(10, -3, -9)), float3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_float(int4(10, -3, -9, 400)), float4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_float(uint2(10, 3)), float2(10.0, 3.0));
  assert_eq!(simd::to_float(uint3(10, 3, 9)), float3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_float(uint4(10, 3, 9, 200)), float4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_float(long2(10, -3)), float2(10.0, -3.0));
  assert_eq!(simd::to_float(long3(10, -3, -9)), float3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_float(long4(10, -3, -9, 400)), float4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_float(ulong2(10, 3)), float2(10.0, 3.0));
  assert_eq!(simd::to_float(ulong3(10, 3, 9)), float3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_float(ulong4(10, 3, 9, 200)), float4(10.0, 3.0, 9.0, 200.0));
}

#[test]
fn to_double() {
  assert_eq!(simd::to_double(float2(10.0, -3.0)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(float3(10.0, -3.0, -9.5)), double3(10.0, -3.0, -9.5));
  assert_eq!(simd::to_double(float4(10.0, -3.0, -9.5, 400.0)), double4(10.0, -3.0, -9.5, 400.0));

  assert_eq!(simd::to_double(double2(10.0, -3.0)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(double3(10.0, -3.0, -9.5)), double3(10.0, -3.0, -9.5));
  assert_eq!(simd::to_double(double4(10.0, -3.0, -9.5, 400.0)), double4(10.0, -3.0, -9.5, 400.0));

  assert_eq!(simd::to_double(char2(10, -3)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(char3(10, -3, -9)), double3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_double(char4(10, -3, -9, 0)), double4(10.0, -3.0, -9.0, 0.0));

  assert_eq!(simd::to_double(uchar2(10, 3)), double2(10.0, 3.0));
  assert_eq!(simd::to_double(uchar3(10, 3, 9)), double3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_double(uchar4(10, 3, 9, 200)), double4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_double(short2(10, -3)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(short3(10, -3, -9)), double3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_double(short4(10, -3, -9, 400)), double4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_double(ushort2(10, 3)), double2(10.0, 3.0));
  assert_eq!(simd::to_double(ushort3(10, 3, 9)), double3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_double(ushort4(10, 3, 9, 200)), double4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_double(int2(10, -3)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(int3(10, -3, -9)), double3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_double(int4(10, -3, -9, 400)), double4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_double(uint2(10, 3)), double2(10.0, 3.0));
  assert_eq!(simd::to_double(uint3(10, 3, 9)), double3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_double(uint4(10, 3, 9, 200)), double4(10.0, 3.0, 9.0, 200.0));

  assert_eq!(simd::to_double(long2(10, -3)), double2(10.0, -3.0));
  assert_eq!(simd::to_double(long3(10, -3, -9)), double3(10.0, -3.0, -9.0));
  assert_eq!(simd::to_double(long4(10, -3, -9, 400)), double4(10.0, -3.0, -9.0, 400.0));

  assert_eq!(simd::to_double(ulong2(10, 3)), double2(10.0, 3.0));
  assert_eq!(simd::to_double(ulong3(10, 3, 9)), double3(10.0, 3.0, 9.0));
  assert_eq!(simd::to_double(ulong4(10, 3, 9, 200)), double4(10.0, 3.0, 9.0, 200.0));
}
