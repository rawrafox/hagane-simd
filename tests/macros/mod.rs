macro_rules! assert_near_f32_scalar {
  ($given:expr , $expected:expr, $error:expr) => ({
    let a = $given;
    let b = $expected as f32;
    let e = $error;

    if b.is_finite() {
      if a.is_nan() {
        assert!(false, "given: {:?}, expected: {:?}", a, b);
      } else if a.is_infinite() {
        assert!(false, "given: {:?}, expected: {:?}", a, b);
      }

      let ulp = unsafe { std::mem::transmute::<f32, i32>((a - b).abs()) };

      assert!(ulp <= e, "given: {:?}, expected: {:?}, ulp: {:?}", a, b, ulp);
    } else if b.is_nan() {
      assert!(a.is_nan(), "given: {:?}, expected {:?}", a, b)
    } else if b.is_infinite() {
      assert!(a == b, "given: {:?}, expected {:?}", a, b)
    } else {
      panic!("Should never get here.")
    }
  })
}

macro_rules! assert_near_f32 {
  ($given:expr , $expected:expr, $error:expr) => ({
    let a = $given;
    let b = $expected;
    let e = $error;

    let ulp1 = (a.to_boolean() - b.to_boolean()).abs();
    let ulp2 = (a - b).abs().to_boolean();

    if ulp1.gt(broadcast(e)).any() || ulp1.lt(broadcast(0)).any() {
      assert!(ulp2.le(broadcast(e)).all(), "given: {:?}, expected: {:?}, ulp: {:?} / {:?}", a, b, ulp1, ulp2);
    }
  })
}
