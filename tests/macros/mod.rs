macro_rules! assert_near_f32 {
  ($given:expr , $expected:expr, $error:expr) => ({
    let a = $given;
    let b = $expected;
    let e = $error;

    let ulp = (a.to_boolean() - b.to_boolean()).abs();
    if ulp.gt(broadcast(e)).any() {
      assert!(e < $error, "given: {:?}, expected: {:?}, ulp: {:?}", a, b, ulp);
    }
  })
}
