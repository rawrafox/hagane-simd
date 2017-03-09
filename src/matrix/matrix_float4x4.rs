use std;
use ::*;

extern {
  fn __invert_f4(a: float4x4) -> float4x4;
}

impl std::ops::Add for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return float4x4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return float4x4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul<float4x4> for float4x4 {
  type Output = float4x4;

  #[inline(always)]
  fn mul(self, other: float4x4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<float4> for float4x4 {
  type Output = float4;

  #[inline(always)]
  fn mul(self, other: float4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f32> for float4x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f32) -> Self {
    let a = float4::broadcast(other);

    return float4x4(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<float4x4> for float4x4 {
  type DotProduct = float4x4;

  #[inline(always)]
  fn dot(self, other: float4x4) -> Self::DotProduct {
    return float4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<float4> for float4x4 {
  type DotProduct = float4;

  #[inline(always)]
  fn dot(self, other: float4) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl PartialEq for float4x4 {
  #[inline]
  fn eq(&self, other: &float4x4) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2) & self.3.eq(other.3)).all()
  }
}

impl float4x4 {
  #[inline(always)]
  pub fn from_columns(c0: float4, c1: float4, c2: float4, c3: float4) -> float4x4 {
    return float4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn from_rows(r0: float4, r1: float4, r2: float4, r3: float4) -> float4x4 {
    return float4x4(r0, r1, r2, r3).transpose();
  }

  #[inline(always)]
  pub fn identity() -> float4x4 {
    return float4x4(float4(1.0, 0.0, 0.0, 0.0), float4(0.0, 1.0, 0.0, 0.0), float4(0.0, 0.0, 1.0, 0.0), float4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn from_scale(scale: f32) -> float4x4 {
    return float4x4(float4(scale, 0.0, 0.0, 0.0), float4(0.0, scale, 0.0, 0.0), float4(0.0, 0.0, scale, 0.0), float4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn from_translation(x: f32, y: f32, z: f32) -> float4x4 {
    return float4x4(float4(1.0, 0.0, 0.0, 0.0), float4(0.0, 1.0, 0.0, 0.0), float4(0.0, 0.0, 1.0, 0.0), float4(x, y, z, 1.0));
  }

  #[inline(always)]
  pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> float4x4 {
    let (sr, cr) = roll.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let (sy, cy) = yaw.sin_cos();

    return float4x4(
      float4(cy * cp, sy * cp, -sp, 0.0),
      float4(cy * sp * sr - sy * cr, sy * sp * sr + cy * cr, cp * sr, 0.0),
      float4(cy * sp * cr + sy * sr, sy * sp * cr - cy * sr, cp * cr, 0.0),
      float4(0.0, 0.0, 0.0, 1.0)
    );
  }

  #[inline(always)]
  pub fn look_at(origin: float3, target: float3, up: float3) -> float4x4 {
    let z = (target - origin).normalize();
    let x = up.cross(z).normalize();
    let y = z.cross(x);
    println!("{:?} {:?} {:?} {:?}", up, z, up.cross(z), up.cross(z).normalize());

    return float4x4(
      float4(x.0, y.0, z.0, 0.0),
      float4(x.1, y.1, z.1, 0.0),
      float4(x.2, y.2, z.2, 0.0),
      float4(-x.dot(origin), -y.dot(origin), -z.dot(origin), 1.0),
    );
  }

  #[inline(always)]
  pub fn perspective(aspect_ratio: f32, fov_y: f32, z_near: f32, z_far: f32) -> float4x4 {
    let fov = 1.0 / ((fov_y / 2.0).tan() * aspect_ratio);
    let distance = z_near - z_far;

    let x = (z_near + z_far) / distance;
    let y = 2.0 * z_near * z_far / distance;
    return float4x4(
      float4(fov, 0.0, 0.0,  0.0),
      float4(0.0, fov, 0.0,  0.0),
      float4(0.0, 0.0,   x, -1.0),
      float4(0.0, 0.0,   y,  0.0)
    );
  }

  #[inline(always)]
  pub fn linear_combination(a: f32, x: float4x4, b: f32, y: float4x4) -> float4x4 {
    let a = float4::broadcast(a);
    let b = float4::broadcast(b);
    return float4x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> float4x4 {
    let c0 = float4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = float4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = float4((self.0).2, (self.1).2, (self.2).2, (self.3).2);
    let c3 = float4((self.0).3, (self.1).3, (self.2).3, (self.3).3);

    return float4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn inverse(self) -> float4x4 {
    return unsafe { __invert_f4(self) };
  }
}
