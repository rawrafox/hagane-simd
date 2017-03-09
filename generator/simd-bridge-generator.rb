#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = [
      { size: 1, max_width: 16, type: 'i8', opencl: 'char', kind: %i(signed integer), bool: "char" },
      { size: 1, max_width: 16, type: 'u8', opencl: 'uchar', kind: %i(unsigned integer), bool: "char" },
      { size: 2, max_width: 16, type: 'i16', opencl: "short", kind: %i(signed integer), bool: "short" },
      { size: 2, max_width: 16, type: 'u16', opencl: "ushort", kind: %i(unsigned integer), bool: "short" },
      { size: 4, max_width: 16, type: 'i32', opencl: "int", kind: %i(signed integer), bool: "int" },
      { size: 4, max_width: 16, type: 'u32', opencl: "uint", kind: %i(unsigned integer), bool: "int" },
      { size: 4, max_width: 16, type: 'f32', opencl: "float", kind: %i(float), bool: "int", max_matrix_size: 4 },
      { size: 8, max_width: 16, type: 'i64', opencl: "long", kind: %i(signed integer), bool: "long" },
      { size: 8, max_width: 16, type: 'u64', opencl: "ulong", kind: %i(unsigned integer), bool: "long"  },
      { size: 8, max_width: 16, type: 'f64', opencl: "double", kind: %i(float), bool: "long", max_matrix_size: 4 }
    ]

    TYPES_BY_NAME = TYPES.map { |x| [x[:opencl], x] }.to_h

    WIDTHS = [2, 3, 4, 8, 16]

    def self.generate(path)
      FileUtils.mkdir_p(path)

      files = []

      TYPES.select { |attributes| attributes[:max_matrix_size] }.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)

        max_matrix_size = attributes.fetch(:max_matrix_size)

        (2 .. max_matrix_size).each do |i|
          vector_name = "#{type}#{i}"

          (2 .. max_matrix_size).each do |j|
            io = StringIO.new
            o = Bridge::Output.new(io)

            name = "#{type}#{j}x#{i}"
            content = (["pub #{vector_name}"] * j).join(", ")

            transpose_vector_name = "#{type}#{j}"
            transpose_name = "#{type}#{i}x#{j}"

            o.puts("use std;", pad: true)
            o.puts("use ::*;")

            if i == j
              if ["f32", "f64"].include?(scalar)
                typecode = { "f32" => "f", "f64" => "d" }.fetch(scalar)

                o.block("extern", pad: true) do
                  o.puts("fn __invert_#{typecode}#{i}(a: #{name}) -> #{name};")
                end
              end
            end

            o.block("impl std::ops::Add for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn add(self, other: Self) -> Self") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "self.#{k} + other.#{k}" }.join(", ")});")
              end
            end

            o.block("impl std::ops::Sub for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn sub(self, other: Self) -> Self") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "self.#{k} - other.#{k}" }.join(", ")});")
              end
            end

            o.block("impl std::ops::Mul<#{transpose_name}> for #{name}", pad: true) do |o|
              o.puts("type Output = #{type}#{i}x#{i};")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn mul(self, other: #{transpose_name}) -> Self::Output") do |o|
                o.puts("return self.dot(other);")
              end
            end

            o.block("impl std::ops::Mul<#{transpose_vector_name}> for #{name}", pad: true) do |o|
              o.puts("type Output = #{vector_name};")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn mul(self, other: #{transpose_vector_name}) -> Self::Output") do |o|
                o.puts("return self.dot(other);")
              end
            end

            o.block("impl std::ops::Mul<#{scalar}> for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn mul(self, other: #{scalar}) -> Self") do |o|
                o.puts("let a = #{vector_name}::broadcast(other);")
                o.puts
                o.puts("return #{name}(#{j.times.map { |k| "a * self.#{k}" }.join(", ")});")
              end
            end

            o.block("impl Dot<#{transpose_name}> for #{name}", pad: true) do |o|
              o.puts("type DotProduct = #{type}#{i}x#{i};")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn dot(self, other: #{transpose_name}) -> Self::DotProduct") do |o|
                o.puts("return #{type}#{i}x#{i}(#{i.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
              end
            end

            o.block("impl Dot<#{transpose_vector_name}> for #{name}", pad: true) do |o|
              o.puts("type DotProduct = #{vector_name};")
              o.puts
              o.puts("#[inline(always)]")
              o.block("fn dot(self, other: #{transpose_vector_name}) -> Self::DotProduct") do |o|
                o.puts("return #{j.times.map { |k| "self.#{k} * other.#{k}" }.join(" + ")};")
              end
            end

            o.block("impl PartialEq for #{name}", pad: true) do |o|
              o.puts("#[inline]")
              o.block("fn eq(&self, other: &#{name}) -> bool") do |o|
                o.puts("return (#{j.times.map { |k| "self.#{k}.eq(other.#{k})" }.join(" & ")}).all()")
              end
            end

            o.block("impl #{name}", pad: true) do
              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn from_columns(#{j.times.map { |k| "c#{k}: #{vector_name}" }.join(", ")}) -> #{name}") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "c#{k}" }.join(", ")});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn from_rows(#{i.times.map { |k| "r#{k}: #{transpose_vector_name}" }.join(", ")}) -> #{name}") do |o|
                o.puts("return #{transpose_name}(#{i.times.map { |k| "r#{k}" }.join(", ")}).transpose();")
              end

              if i == j
                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn identity() -> #{name}") do |o|
                  identity = j.times.map { |k| "#{vector_name}(#{([0.0] * i).tap { |ary| ary[k] = 1.0 }.join(", ")})" }.join(", ")

                  o.puts("return #{name}(#{identity});")
                end
              end

              if i == 4 && j == 4
                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn from_scale(scale: #{scalar}) -> #{name}") do |o|
                  x = (j - 1).times.map { |k| "#{vector_name}(#{(["0.0"] * i).tap { |ary| ary[k] = "scale" }.join(", ")})" }.join(", ")
                  x += ", #{vector_name}(0.0, 0.0, 0.0, 1.0)"

                  o.puts("return #{name}(#{x});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn from_translation(x: #{scalar}, y: #{scalar}, z: #{scalar}) -> #{name}") do |o|
                  x = (j - 1).times.map { |k| "#{vector_name}(#{(["0.0"] * i).tap { |ary| ary[k] = "1.0" }.join(", ")})" }.join(", ")
                  x += ", #{vector_name}(x, y, z, 1.0)"

                  o.puts("return #{name}(#{x});")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn from_euler_angles(roll: #{scalar}, pitch: #{scalar}, yaw: #{scalar}) -> #{name}") do |o|
                  o.puts("let (sr, cr) = roll.sin_cos();", pad: true)
                  o.puts("let (sp, cp) = pitch.sin_cos();")
                  o.puts("let (sy, cy) = yaw.sin_cos();")

                  o.puts("return #{name}(", pad: true)
                  o.puts("  #{vector_name}(cy * cp, sy * cp, -sp, 0.0),")
                  o.puts("  #{vector_name}(cy * sp * sr - sy * cr, sy * sp * sr + cy * cr, cp * sr, 0.0),")
                  o.puts("  #{vector_name}(cy * sp * cr + sy * sr, sy * sp * cr - cy * sr, cp * cr, 0.0),")
                  o.puts("  #{vector_name}(0.0, 0.0, 0.0, 1.0)")
                  o.puts(");")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn look_at(eye: #{type}3, center: #{type}3, up: #{type}3) -> #{name}") do |o|
                  o.puts("let z = (center - eye).normalize();", pad: true)
                  o.puts("let x = up.cross(z).normalize();")
                  o.puts("let y = z.cross(x);")

                  o.puts("let p = #{vector_name}(x.0, y.0, z.0, 0.0);", pad: true)
                  o.puts("let q = #{vector_name}(x.1, y.1, z.1, 0.0);")
                  o.puts("let r = #{vector_name}(x.2, y.2, z.2, 0.0);")
                  o.puts("let s = #{vector_name}(-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0);")

                  o.puts("return #{name}(p, q, r, s);", pad: true)
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn perspective(width: #{scalar}, height: #{scalar}, near: #{scalar}, far: #{scalar}) -> #{name}") do |o|
                  o.puts("let z_near = 2.0 * near;", pad: true)
                  o.puts("let z_far = far / (far - near);")

                  o.puts("let p = #{vector_name}(z_near / width, 0.0, 0.0, 0.0);", pad: true)
                  o.puts("let q = #{vector_name}(0.0, z_near / height, 0.0, 0.0);")
                  o.puts("let r = #{vector_name}(0.0, 0.0, z_far, 1.0);")
                  o.puts("let s = #{vector_name}(0.0, 0.0, -near * z_far, 0.0);")

                  o.puts("return #{name}(p, q, r, s);", pad: true)
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn perspective_fov(fov_y: #{scalar}, aspect: #{scalar}, near: #{scalar}, far: #{scalar}) -> #{name}") do |o|
                  o.puts("let y_scale = 1.0 / (0.5 * fov_y).tan();", pad: true)
                  o.puts("let x_scale = y_scale / aspect;")
                  o.puts("let z_scale = far / (far - near);")

                  o.puts("let p = #{vector_name}(x_scale, 0.0, 0.0, 0.0);", pad: true)
                  o.puts("let q = #{vector_name}(0.0, y_scale, 0.0, 0.0);")
                  o.puts("let r = #{vector_name}(0.0, 0.0, z_scale, 1.0);")
                  o.puts("let s = #{vector_name}(0.0, 0.0, -near * z_scale, 0.0);")

                  o.puts("return #{name}(p, q, r, s);", pad: true)
                end
                
                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn orthographic(left: #{scalar}, right: #{scalar}, bottom: #{scalar}, top: #{scalar}, near: #{scalar}, far: #{scalar}) -> #{name}") do |o|
                  o.puts("let s_length = 1.0 / (right - left);", pad: true)
                  o.puts("let s_height = 1.0 / (top - bottom);")
                  o.puts("let s_depth = 1.0 / (far - near);")

                  o.puts("let p = #{vector_name}(2.0 * s_length, 0.0, 0.0, 0.0);", pad: true)
                  o.puts("let q = #{vector_name}(0.0, 2.0 * s_height, 0.0, 0.0);")
                  o.puts("let r = #{vector_name}(0.0, 0.0, s_depth, 0.0);")
                  o.puts("let s = #{vector_name}(0.0, 0.0, -near * s_depth, 1.0);")

                  o.puts("return #{name}(p, q, r, s);", pad: true)
                end
                
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn linear_combination(a: #{scalar}, x: #{name}, b: #{scalar}, y: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts("let b = #{vector_name}::broadcast(b);")

                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k} + b * y.#{k}" }.join(", ")});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn transpose(self) -> #{transpose_name}") do |o|
                i.times do |k|
                  o.puts("let c#{k} = #{transpose_vector_name}(#{j.times.map { |l| "(self.#{l}).#{k}" }.join(", ")});")
                end
                o.puts
                o.puts("return #{transpose_name}(#{i.times.map { |k| "c#{k}" }.join(", ")});")
              end

              # TODO: matrix_determinant

              if i == j && ["f32", "f64"].include?(scalar)
                typecode = { "f32" => "f", "f64" => "d" }.fetch(scalar)

                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn inverse(self) -> #{name}") do |o|
                  o.puts("return unsafe { __invert_#{typecode}#{i}(self) };")
                end
              end

              # matrix_multiply is expressed via the `Dot` trait

              # TODO: o.puts("#[inline(always)]", pad: true)
              # o.block("pub fn almost_equal_elements(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end

              # TODO: o.puts("#[inline(always)]", pad: true)
              # o.block("pub fn almost_equal_elements_relative(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end
            end

            files << ["#{path}/matrix/matrix_#{name}.rs", io.string]
          end
        end
      end

      files
    end

    def self.conversion(o, in_ty, out_ty, width, saturate: false)
      in_ty = TYPES_BY_NAME.fetch(in_ty)
      out_ty = TYPES_BY_NAME.fetch(out_ty)

      in_scalar = "#{in_ty.fetch(:type)}"
      in_type = "#{in_ty.fetch(:opencl)}"
      in_name = "#{in_type}#{width}"
      in_size = in_ty.fetch(:size)
      in_kind = in_ty.fetch(:kind)

      out_scalar = "#{out_ty.fetch(:type)}"
      out_type = "#{out_ty.fetch(:opencl)}"
      out_name = "#{out_type}#{width}"
      out_size = out_ty.fetch(:size)
      out_kind = out_ty.fetch(:kind)

      if saturate
        o.puts("#[inline(always)]", pad: true)
        o.block("fn to_#{out_type}_sat(self) -> #{out_name}") do |o|
          min = "Self::broadcast(std::#{out_scalar}::MIN as #{in_scalar})"
          max = "Self::broadcast(std::#{out_scalar}::MAX as #{in_scalar})"

          if in_scalar == out_scalar
            o.puts("return self;")
          elsif in_kind == out_kind && in_size < out_size
            o.puts("return #{in_name}::to_#{out_type}(self);")
          elsif in_kind.include?(:signed) && out_kind.include?(:unsigned) && in_size <= out_size
            o.puts("return #{in_name}::to_#{out_type}(self.max(Self::from(0)));")
          elsif in_kind.include?(:unsigned)
            o.puts("return #{in_name}::to_#{out_type}(self.min(#{max}));")
          else
            o.puts("return #{in_name}::to_#{out_type}(self.clamp(#{min}, #{max}));")
          end
        end
      else
        if width == 3 && !in_kind.include?(:float) && in_size < out_size # TODO: Fix this compiler bug
          o.puts("#[inline(always)]", pad: true)
          o.block("fn to_#{out_type}(self) -> #{out_name}") do |o|
            result = width.times.map { |i| "self.#{i} as #{out_scalar}"}

            o.puts("return #{out_name}(#{result.join(", ")});")
          end
        end
      end
    end
  end
end
