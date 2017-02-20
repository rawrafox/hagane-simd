#![feature(cfg_target_feature, link_llvm_intrinsics, platform_intrinsics, simd_ffi, test)]

extern crate hagane_simd;
extern crate test;

use hagane_simd::*;
use test::*;

#[cfg(target_feature = "sse")]
extern "platform-intrinsic" {
  fn x86_mm_sqrt_ps(x: float4) -> float4;
  fn x86_mm_rsqrt_ps(x: float4) -> float4;
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_sqrt_plus_div(b: &mut Bencher) {
  let n = black_box(float4(1.0, 3.0, 5.0, 7.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let sqrt = unsafe { x86_mm_sqrt_ps(sum) };

      sum = sum + 1.0 / sqrt;
    }

    sum
  });
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_intel_rsqrt(b: &mut Bencher) {
  let n = black_box(float4(1.0, 3.0, 5.0, 7.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rsqrt_ps(sum) };

      sum = sum - 0.5 * (r * (3.0 - r * r * sum));
    }

    sum
  });
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_apple_rsqrt(b: &mut Bencher) {
  let n = black_box(float4(1.0, 3.0, 5.0, 7.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rsqrt_ps(sum) };
      let r = r * (1.5 - 0.5 * sum.eq(float4::broadcast(0.0)).bitselect(sum, float4::broadcast(-std::f32::INFINITY)) * r * r);

      sum = sum + r;
    }

    sum
  });
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_apple_rsqrt_simplified(b: &mut Bencher) {
  let n = black_box(float4(1.0, 3.0, 5.0, 7.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rsqrt_ps(sum) };
      let r = r * (1.5 - 0.5 * sum * r * r);

      sum = sum + r;
    }

    sum
  });
}
