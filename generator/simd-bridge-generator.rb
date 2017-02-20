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

            if i == j # TODO: Implement this for i != j
              o.block("impl std::ops::Mul for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline(always)]")
                o.block("fn mul(self, other: Self) -> Self") do |o|
                  o.puts("return self.dot(other);")
                end
              end

              o.block("impl std::ops::Mul<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline(always)]")
                o.block("fn mul(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return self.dot(other);")
                end
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

            if i == j
              o.block("impl Dot<#{name}> for #{name}", pad: true) do |o|
                o.puts("type DotProduct = #{name};")
                o.puts
                o.puts("#[inline(always)]")
                o.block("fn dot(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl Dot<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type DotProduct = #{vector_name};")
                o.puts
                o.puts("#[inline(always)]")
                o.block("fn dot(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return #{j.times.map { |k| "self.#{k} * other.#{k}" }.join(" + ")};")
                end
              end
            end

            o.block("impl #{name}", pad: true) do
              if i == j
                o.puts("#[inline(always)]", pad: true)
                o.block("pub fn identity(self) -> #{name}") do |o|
                  identity = j.times.map { |k| "#{vector_name}(#{([0.0] * i).tap { |ary| ary[k] = 1.0 }.join(", ")})" }.join(", ")

                  o.puts("return #{name}(#{identity});")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn linear_combination(a: #{scalar}, x: #{name}, b: #{scalar}, y: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts("let b = #{vector_name}::broadcast(b);")

                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k} + b * y.#{k}" }.join(", ")});")
              end

              transpose_vector_name = "#{type}#{j}"
              transpose_matrix_name = "#{type}#{i}x#{j}"

              o.puts("#[inline(always)]", pad: true)
              o.block("pub fn transpose(self) -> #{transpose_matrix_name}") do |o|
                i.times do |k|
                  o.puts("let c#{k} = #{transpose_vector_name}(#{j.times.map { |l| "(self.#{l}).#{k}" }.join(", ")});")
                end
                o.puts
                o.puts("return #{transpose_matrix_name}(#{i.times.map { |k| "c#{k}" }.join(", ")});")
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
              #o.block("pub fn equal(x: #{name}, y: #{name}) -> bool") do |o|
              #end

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
