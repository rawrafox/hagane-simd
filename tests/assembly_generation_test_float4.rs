extern crate hagane_simd;

use hagane_simd::*;

#[macro_use] mod macros;

#[inline(never)]
fn abs(x: float4) -> float4 {
  return x.abs();
}

#[inline(never)]
fn add(x: float4, y: float4) -> float4 {
  return x + y;
}

#[inline(never)]
fn sub(x: float4, y: float4) -> float4 {
  return x - y;
}

#[inline(never)]
fn mul(x: float4, y: float4) -> float4 {
  return x * y;
}

#[inline(never)]
fn div(x: float4, y: float4) -> float4 {
  return x / y;
}

#[inline(never)]
fn recip(x: float4) -> float4 {
  return x.recip();
}

#[inline(never)]
fn rsqrt(x: float4) -> float4 {
  return x.rsqrt();
}

#[inline(never)]
fn add_mul(x: float4, y: float4, z: float4) -> float4 {
  return x.add_mul(y, z);
}

#[inline(never)]
fn dot(x: float4, y: float4) -> f32 {
  return x.dot(y);
}

#[inline(never)]
fn max(x: float4, y: float4) -> float4 {
  return x.max(y);
}

#[inline(never)]
fn min(x: float4, y: float4) -> float4 {
  return x.min(y);
}

#[test]
fn test() {
  let a = float4(1.0, 2.0, 3.0, 4.0);
  let b = float4(2.0, 3.0, 4.0, 5.0);
  let c = float4(-1.0, -2.0, -3.0, -4.0);
  let z = float4(0.0, 0.0, 0.0, 0.0);

  assert_eq!(abs(a), a);
  assert_eq!(abs(b), b);
  assert_eq!(abs(c), a);

  assert_eq!(dot(a, b), 40.0);
  assert_eq!(dot(b, a), 40.0);

  assert_eq!(add(a, b), float4(3.0, 5.0, 7.0, 9.0));
  assert_eq!(add(b, a), float4(3.0, 5.0, 7.0, 9.0));

  assert_eq!(sub(a, b), float4(-1.0, -1.0, -1.0, -1.0));
  assert_eq!(sub(b, a), float4(1.0, 1.0, 1.0, 1.0));

  assert_eq!(mul(a, b), float4(2.0, 6.0, 12.0, 20.0));
  assert_eq!(mul(b, a), float4(2.0, 6.0, 12.0, 20.0));

  assert_eq!(div(a, b), float4(0.5, 2.0 / 3.0, 0.75, 0.8));
  assert_eq!(div(b, a), float4(2.0, 1.5, 4.0 / 3.0, 5.0 / 4.0));

  assert_near_f32!(recip(a), 1.0 / a, 1);
  assert_near_f32!(recip(b), 1.0 / b, 1);
  assert_near_f32!(recip(c), 1.0 / c, 1);
  assert_near_f32!(recip(z), 1.0 / z, 0);

  assert_near_f32!(rsqrt(a), 1.0 / a.sqrt(), 1);
  assert_near_f32!(rsqrt(b), 1.0 / b.sqrt(), 1);
  assert_near_f32!(rsqrt(z), 1.0 / z.sqrt(), 0);

  assert_eq!(add_mul(a, b, c), float4(-1.0, -4.0, -9.0, -16.0));
  assert_eq!(add_mul(b, a, c), float4(1.0, -1.0, -5.0, -11.0));

  assert_eq!(max(a, b), b);
  assert_eq!(max(b, a), b);

  assert_eq!(min(a, b), a);
  assert_eq!(min(b, a), a);
}
