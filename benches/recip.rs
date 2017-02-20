#![feature(cfg_target_feature, link_llvm_intrinsics, platform_intrinsics, simd_ffi, test)]

extern crate hagane_simd;
extern crate test;

use hagane_simd::*;
use test::*;

#[cfg(target_feature = "sse")]
extern "platform-intrinsic" {
  fn x86_mm_rcp_ps(x: float4) -> float4;
}

#[bench]
fn bench_div(b: &mut Bencher) {
  let n = black_box(float4(0.0, 1.0, 5.0, 9.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      sum = sum + 1.0 / sum;
    }

    sum
  });
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_apple_recip(b: &mut Bencher) {
  let n = black_box(float4(0.0, 1.0, 5.0, 9.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rcp_ps(sum) };

      let r  = r * (2.0 - sum.eq(float4::broadcast(0.0)).bitselect(sum, float4::broadcast(-std::f32::INFINITY)) * r);

      sum = sum + r;
    }

    sum
  })
}


#[cfg(target_feature = "sse")]
#[bench]
fn bench_apple_simplified_recip(b: &mut Bencher) {
  let n = black_box(float4(0.0, 1.0, 5.0, 9.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rcp_ps(sum) };

      let r  = r * (2.0 - sum * r);

      sum = sum + r;
    }

    sum
  })
}

#[cfg(target_feature = "sse")]
#[bench]
fn bench_intel_rpcps_nr1_recip(b: &mut Bencher) {
  let n = black_box(float4(0.0, 1.0, 5.0, 9.0));

  b.iter(|| {
    let mut sum = n;

    for _ in 0 .. 10000 {
      let r = unsafe { x86_mm_rcp_ps(sum) };
      let r = r * ((r + r) - ((r * r) * sum));

      sum = sum + r;
    }

    sum
  })
}
