extern crate hagane_simd as simd;

use simd::*;

#[test]
fn test_rem() {
  assert_eq!(float2(4.0, -4.0) % float2(3.0, 3.0) , float2(1.0, -1.0));
  assert_eq!(float3(4.0, -4.0, -3.5) % float3(3.0, 3.0, -3.0), float3(1.0, -1.0, -0.5));
  assert_eq!(float4(4.0, -4.0, -3.5, 3.5) % float4(3.0, 3.0, -3.0, -3.0), float4(1.0, -1.0, -0.5, 0.5));

  // assert_eq!(abs(double2(10.0, -3.0)), double2(10.0, 3.0));
  // assert_eq!(abs(double3(10.0, -3.0, -9.5)), double3(10.0, 3.0, 9.5));
  // assert_eq!(abs(double4(10.0, -3.0, -9.5, 0.0)), double4(10.0, 3.0, 9.5, 0.0));
  // 
  // assert_eq!(abs(char2(10, -3)), char2(10, 3));
  // assert_eq!(abs(char3(10, -3, -9)), char3(10, 3, 9));
  // assert_eq!(abs(char4(10, -3, -9, 0)), char4(10, 3, 9, 0));
  // 
  // assert_eq!(abs(short2(10, -3)), short2(10, 3));
  // assert_eq!(abs(short3(10, -3, -9)), short3(10, 3, 9));
  // assert_eq!(abs(short4(10, -3, -9, 0)), short4(10, 3, 9, 0));
  // 
  // assert_eq!(abs(int2(10, -3)), int2(10, 3));
  // assert_eq!(abs(int3(10, -3, -9)), int3(10, 3, 9));
  // assert_eq!(abs(int4(10, -3, -9, 0)), int4(10, 3, 9, 0));
  // 
  // assert_eq!(abs(long2(10, -3)), long2(10, 3));
  // assert_eq!(abs(long3(10, -3, -9)), long3(10, 3, 9));
  // assert_eq!(abs(long4(10, -3, -9, 0)), long4(10, 3, 9, 0));
}
