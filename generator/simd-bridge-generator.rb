#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = [
      { size: 1, max_width: 4, type: 'i8', opencl: 'char', kind: %i(signed integer), bool: "char" },
      # { size: 1, max_width: 4, type: 'u8', opencl: 'uchar', unsigned: true },
      { size: 2, max_width: 4, type: 'i16', opencl: "short", kind: %i(signed integer), bool: "short" },
      # { size: 2, max_width: 4, type: 'u16', opencl: "ushort", unsigned: true },
      { size: 4, max_width: 4, type: 'i32', opencl: "int", kind: %i(signed integer), bool: "int" },
      # { size: 4, max_width: 4, type: 'u32', opencl: "uint", unsigned: true },
      { size: 4, max_width: 4, type: 'f32', opencl: "float", kind: %i(float), bool: "int", max_matrix_size: 4 },
      { size: 8, max_width: 4, type: 'i64', opencl: "long", kind: %i(signed integer), bool: "long" },
      # { size: 8, max_width: 4, type: 'u64', opencl: "ulong", unsigned: true },
      { size: 8, max_width: 4, type: 'f64', opencl: "double", kind: %i(float), bool: "long", max_matrix_size: 4 }
    ]
    
    TYPES_BY_NAME = TYPES.map { |x| [x[:opencl], x] }.to_h

    WIDTHS = [1, 2, 3, 4, 8, 16]

    def self.generate(path)
      FileUtils.mkdir_p(path)

      files = []

      TYPES.each do |attributes|
        scalar = attributes.fetch(:type)
        type = attributes.fetch(:opencl)
        kind = attributes.fetch(:kind)
        bool = attributes.fetch(:bool)

        WIDTHS.select { |x| x <= attributes.fetch(:max_width) }.each do |width|
          io = StringIO.new
          o = Bridge::Output.new(io)

          name = "#{type}#{width}"
          bool_name = "#{bool}#{width}"

          if width == 1
            o.puts("pub type #{name} = #{attributes.fetch(:type)};", pad: true)
            o.puts("pub type vector_#{name} = #{name};")
          else
            content = (["pub #{attributes.fetch(:type)}"] * width).join(", ")

            o.puts("use std;", pad: true)
            o.puts("use ::*;")

            o.puts("#[repr(C)]", pad: true)
            o.puts("#[repr(simd)]")
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
            o.puts("pub type vector_#{name} = #{name};")

            o.block("extern \"platform-intrinsic\"", pad: true) do
              o.puts("fn simd_add<T>(x: T, y: T) -> T;", pad: true)
              o.puts("fn simd_sub<T>(x: T, y: T) -> T;")
              o.puts("fn simd_mul<T>(x: T, y: T) -> T;")
              o.puts("fn simd_div<T>(x: T, y: T) -> T;")

              if kind.include?(:integer)
                o.puts("fn simd_shl<T>(x: T, y: T) -> T;", pad: true)
                o.puts("fn simd_shr<T>(x: T, y: T) -> T;")

                o.puts("fn simd_and<T>(x: T, y: T) -> T;", pad: true)
                o.puts("fn simd_or<T>(x: T, y: T) -> T;")
                o.puts("fn simd_xor<T>(x: T, y: T) -> T;")
              end

              o.puts("fn simd_eq<T, U>(x: T, y: T) -> U;", pad: true)
              o.puts("fn simd_ne<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_lt<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_le<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_gt<T, U>(x: T, y: T) -> U;")
              o.puts("fn simd_ge<T, U>(x: T, y: T) -> U;")

              o.puts("fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;", pad: true)
              o.puts("fn simd_extract<T, E>(x: T, i: u32) -> E;")
            end

            o.block("impl std::ops::Index<u32> for #{name}", pad: true) do |o|
              o.puts("type Output = #{scalar};")
              o.puts
              o.puts("#[inline]")
              o.block("fn index(&self, index: u32) -> &#{scalar}") do |o|
                o.puts("return unsafe { simd_extract(self, index) };")
              end
            end

            %w(add sub mul div).each do |op|
              o.block("impl std::ops::#{op.capitalize} for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: Self) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, other) };")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                  o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                end
              end

              o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                end
              end
            end

            if kind.include?(:integer)
              %w(and or xor).each do |op|
                o.block("impl std::ops::Bit#{op.capitalize} for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: Self) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, other) };")
                  end
                end

                o.block("impl std::ops::Bit#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: #{scalar}) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                  end
                end

                o.block("impl std::ops::Bit#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                  o.puts("type Output = #{name};")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn bit#{op}(self, other: #{name}) -> #{name}") do |o|
                    o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                  end
                end
              end

              %w(shl shr).each do |op|
                o.block("impl std::ops::#{op.capitalize}<#{name}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: Self) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, other) };")
                  end
                end

                o.block("impl std::ops::#{op.capitalize}<#{scalar}> for #{name}", pad: true) do |o|
                  o.puts("type Output = Self;")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: #{scalar}) -> Self") do |o|
                    o.puts("return unsafe { simd_#{op}(self, #{name}::broadcast(other)) };")
                  end
                end

                o.block("impl std::ops::#{op.capitalize}<#{name}> for #{scalar}", pad: true) do |o|
                  o.puts("type Output = #{name};")
                  o.puts
                  o.puts("#[inline]")
                  o.block("fn #{op}(self, other: #{name}) -> #{name}") do |o|
                    o.puts("return unsafe { simd_#{op}(#{name}::broadcast(self), other) };")
                  end
                end
              end

              o.block("impl std::ops::Not for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn not(self) -> Self") do |o|
                  o.puts("return self ^ #{name}::broadcast(-1);")
                end
              end
            end

            o.block("impl PartialEq for #{name}", pad: true) do |o|
              o.puts("#[inline]", pad: true)
              o.block("fn eq(&self, other: &Self) -> bool") do |o|
                o.puts("return #{bool_name}::all(#{name}::eq(*self, *other));")
              end

              o.puts("#[inline]", pad: true)
              o.block("fn ne(&self, other: &Self) -> bool") do |o|
                o.puts("return #{bool_name}::all(#{name}::ne(*self, *other));")
              end
            end

            o.block("impl Dot for #{name}", pad: true) do |o|
              o.puts("type Output = #{scalar};")
              o.puts
              o.puts("#[inline]")
              o.block("fn dot(self, other: #{name}) -> #{scalar}") do |o|
                o.puts("return #{name}::reduce_add(self * other);")
              end
            end

            o.block("impl #{name}", pad: true) do |o|
              o.puts("#[inline]", pad: true)
              o.block("pub fn bitcast<T>(x: T) -> #{name}") do |o|
                o.puts("assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());")
                o.puts
                o.puts("return unsafe { std::mem::transmute_copy(&x) };")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn broadcast(x: #{scalar}) -> #{name}") do |o|
                o.puts("return #{name}(#{(["x"] * width).join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn extract(self, i: u32) -> #{scalar}") do |o|
                o.puts("return unsafe { simd_extract(self, i) };")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn replace(self, i: u32, x: #{scalar}) -> #{name}") do |o|
                o.puts("return unsafe { simd_insert(self, i, x) };")
              end

              # Comparison

              %w(eq ne lt le gt ge).each do |op|
                o.puts("#[inline]", pad: true)
                o.block("pub fn #{op}(x: #{name}, y: #{name}) -> #{bool_name}") do |o|
                  o.puts("return unsafe { simd_#{op}(x, y) };")
                end
              end

              # Common

              if kind.include?(:signed)
                o.puts("#[inline]", pad: true)
                o.block("pub fn abs(x: #{name}) -> #{name}") do |o|
                  o.puts("let mask = x >> #{attributes.fetch(:size) * 8 - 1};")

                  o.puts("return (x ^ mask) - mask;")
                end
              elsif kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn abs(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.abs()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end
              end

              if kind.include?(:integer)
                o.puts("#[inline]", pad: true)
                o.block("pub fn max(x: #{name}, y: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitselect(x, y, #{name}::gt(y, x));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn min(x: #{name}, y: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitselect(x, y, #{name}::lt(y, x));")
                end
              elsif kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn max(x: #{name}, y: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.max(y.#{i})" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn min(x: #{name}, y: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.min(y.#{i})" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn clamp(x: #{name}, min: #{name}, max: #{name}) -> #{name}") do |o|
                o.puts("return #{name}::min(#{name}::max(x, min), max);")
              end

              if kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn sign(x: #{name}) -> #{name}") do |o|
                  o.puts("let (zero, one) = (#{name}::broadcast(0.0), #{name}::broadcast(1.0));")

                  o.puts("return #{name}::bitselect(#{name}::copysign(one, x), zero, #{name}::eq(x, zero) | #{name}::ne(x, x));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn mix(x: #{name}, y: #{name}, t: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.signum()" }.join(", ")

                  o.puts("return x + t * (y - x);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn recip(x: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}::broadcast(1.0) / x;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn rsqrt(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "1.0 / x.#{i}.sqrt()" }.join(", ")

                  o.puts("return #{name}::broadcast(1.0) / #{name}::sqrt(x);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn fract(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.fract()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn step(edge: #{name}, x: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitselect(#{name}::broadcast(1.0), #{name}::broadcast(0.0), #{name}::lt(x, edge));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn smoothstep(edge0: #{name}, edge1: #{name}, x: #{name}) -> #{name}") do |o|
                  o.puts("let t = #{name}::clamp((x - edge0) / (edge1 - edge0), #{name}::broadcast(0.0), #{name}::broadcast(1.0));")
                  o.puts
                  o.puts("return t * t * (3.0 - 2.0 * t);")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn reduce_add(x: #{name}) -> #{scalar}") do |o|
                case width
                when 2
                  o.puts("return x.0 + x.1;")
                when 3
                  o.puts("return x.0 + x.1 + x.2;")
                else
                  o.puts("return #{type}#{width / 2}::reduce_add(x.lo() + x.hi());")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn reduce_min(x: #{name}) -> #{scalar}") do |o|
                case width
                when 2
                  if kind.include?(:float)
                    o.puts("return x.0.min(x.1);")
                  else
                    o.puts("return std::cmp::min(x.0, x.1);")
                  end
                when 3
                  if kind.include?(:float)
                    o.puts("return x.2.min(#{type}2::reduce_min(x.lo()));")
                  else
                    o.puts("return std::cmp::min(#{type}2::reduce_min(x.lo()), x.2);")
                  end
                else
                  o.puts("return #{type}#{width / 2}::reduce_min(#{type}#{width / 2}::min(x.lo(), x.hi()));")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn reduce_max(x: #{name}) -> #{scalar}") do |o|
                case width
                when 2
                  if kind.include?(:float)
                    o.puts("return x.0.max(x.1);")
                  else
                    o.puts("return std::cmp::max(x.0, x.1);")
                  end
                when 3
                  if kind.include?(:float)
                    o.puts("return x.2.max(#{type}2::reduce_max(x.lo()));")
                  else
                    o.puts("return std::cmp::max(#{type}2::reduce_max(x.lo()), x.2);")
                  end
                else
                  o.puts("return #{type}#{width / 2}::reduce_max(#{type}#{width / 2}::max(x.lo(), x.hi()));")
                end
              end

              # Math

              if kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn copysign(x: #{name}, y: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitselect(y, x, #{bool_name}::broadcast(std::#{TYPES_BY_NAME[bool][:type]}::MAX));")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn sqrt(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.sqrt()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn ceil(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.ceil()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn floor(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.floor()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                # TODO: Blocked by libstd
                # o.puts("#[inline]", pad: true)
                # o.block("pub fn rint(x: #{name}) -> #{name}") do |o|
                #   result = width.times.map { |i| "x.#{i}.rint()" }.join(", ")
                # 
                #   o.puts("return #{name}(#{result});")
                # end

                o.puts("#[inline]", pad: true)
                o.block("pub fn trunc(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.trunc()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn sin(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.sin()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn cos(x: #{name}) -> #{name}") do |o|
                  result = width.times.map { |i| "x.#{i}.cos()" }.join(", ")

                  o.puts("return #{name}(#{result});")
                end

              end

              # Logic

              if kind.include?(:integer)
                constant = kind.include?(:signed) ? "std::#{scalar}::MIN" : "std::#{scalar}::MAX"

                o.puts("#[inline]", pad: true)
                o.block("pub fn all(x: #{name}) -> bool") do |o|
                  result = width.times.map { |i| "x.#{i}" }.join(" & ")

                  o.puts("return (#{result}) & #{constant} != 0;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn any(x: #{name}) -> bool") do |o|
                  result = width.times.map { |i| "x.#{i}" }.join(" | ")

                  o.puts("return (#{result}) & #{constant} != 0;")
                end
              end

              if kind.include?(:float)
                o.puts("#[inline]", pad: true)
                o.block("pub fn select(x: #{name}, y: #{name}, z: #{bool_name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitselect(x, y, z >> #{attributes.fetch(:size) * 8 - 1});")
                end
              end

              if kind.include?(:signed)
                o.puts("#[inline]", pad: true)
                o.block("pub fn bitselect(x: #{name}, y: #{name}, z: #{bool_name}) -> #{name}") do |o|
                  o.puts("return (x & !z) | (y & z);")
                end
              else
                o.puts("#[inline]", pad: true)
                o.block("pub fn bitselect(x: #{name}, y: #{name}, z: #{bool_name}) -> #{name}") do |o|
                  o.puts("return #{name}::bitcast(#{bool_name}::bitselect(#{bool_name}::bitcast(x), #{bool_name}::bitcast(y), z));")
                end
              end

              # Geometry

              # if kind.include?(:float)
              #   o.puts("#[inline]", pad: true)
              #   o.block("pub fn length_squared(self) -> #{scalar}") do |o|
              #     o.puts("return self.dot(self);")
              #   end
              #
              #   o.puts("#[inline]", pad: true)
              #   o.block("pub fn normalize(self) -> #{name}") do |o|
              #     o.puts("return self / self.length_squared().sqrt();") # TODO: Use some rsqrt approximation
              #   end
              # end


              # Swizzles

              case width
              when 2
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return self.0;")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return self.1;")
                end
              when 3
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.0, self.1);")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{type}2") do |o|
                  o.puts("return #{type}2(self.2, 0#{".0" if kind.include?(:float)});")
                end
              else
                o.puts("#[inline]", pad: true)
                o.block("pub fn lo(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{i}"}.join(", ")});")
                end

                o.puts("#[inline]", pad: true)
                o.block("pub fn hi(self) -> #{type}#{width / 2}") do |o|
                  o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{width / 2 + i}"}.join(", ")});")
                end
              end
            end
          end

          files << ["#{path}/type_#{name}.rs", io.string]
        end
      end

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

            o.puts("#[repr(C)]", pad: true)
            o.puts("#[derive(Copy, Clone, Debug)]")
            o.puts("pub struct #{name}(#{content});")
            o.puts("pub type matrix_#{name} = #{name};")

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
              o.puts("#[inline]")
              o.block("fn add(self, other: Self) -> Self") do |o|
                o.puts("return #{name}::add(self, other);")
              end
            end

            o.block("impl std::ops::Sub for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline]")
              o.block("fn sub(self, other: Self) -> Self") do |o|
                o.puts("return #{name}::sub(self, other);")
              end
            end

            if kind.include?(:float) && i == j # TODO: Implement this for i != j
              o.block("impl std::ops::Mul for #{name}", pad: true) do |o|
                o.puts("type Output = Self;")
                o.puts
                o.puts("#[inline]")
                o.block("fn mul(self, other: Self) -> Self") do |o|
                  o.puts("return self.dot(other);")
                end
              end

              o.block("impl std::ops::Mul<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn mul(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return self.dot(other);")
                end
              end
            end

            o.block("impl std::ops::Mul<#{scalar}> for #{name}", pad: true) do |o|
              o.puts("type Output = Self;")
              o.puts
              o.puts("#[inline]")
              o.block("fn mul(self, other: #{scalar}) -> Self") do |o|
                o.puts("return #{name}::scale(other, self);")
              end
            end

            if kind.include?(:float) && i == j
              o.block("impl Dot for #{name}", pad: true) do |o|
                o.puts("type Output = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl Dot<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type Output = #{vector_name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{vector_name}) -> #{vector_name}") do |o|
                  o.puts("return #{j.times.map { |k| "self.#{k} * other.#{k}" }.join(" + ")};")
                end
              end
            end

            o.block("impl #{name}", pad: true) do
              if i == j
                o.puts("#[inline]", pad: true)
                o.block("pub fn identity(self) -> #{name}") do |o|
                  identity = j.times.map { |k| "#{vector_name}(#{([0.0] * i).tap { |ary| ary[k] = 1.0 }.join(", ")})" }.join(", ")

                  o.puts("return #{name}(#{identity});")
                end
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn scale(a: #{scalar}, x: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts
                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn linear_combination(a: #{scalar}, x: #{name}, b: #{scalar}, y: #{name}) -> #{name}") do |o|
                o.puts("let a = #{vector_name}::broadcast(a);")
                o.puts("let b = #{vector_name}::broadcast(b);")

                o.puts("return #{name}(#{j.times.map { |k| "a * x.#{k} + b * y.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn add(x: #{name}, y: #{name}) -> #{name}") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "x.#{k} + y.#{k}" }.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn sub(x: #{name}, y: #{name}) -> #{name}") do |o|
                o.puts("return #{name}(#{j.times.map { |k| "x.#{k} - y.#{k}" }.join(", ")});")
              end

              transpose_vector_name = "#{type}#{j}"
              transpose_matrix_name = "#{type}#{i}x#{j}"

              o.puts("#[inline]", pad: true)
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

                o.puts("#[inline]", pad: true)
                o.block("pub fn inverse(self) -> #{name}") do |o|
                  o.puts("return unsafe { __invert_#{typecode}#{i}(self) };")
                end
              end

              # matrix_multiply is expressed via the `Dot` trait

              # TODO: o.puts("#[inline]", pad: true)
              #o.block("pub fn equal(x: #{name}, y: #{name}) -> bool") do |o|
              #end

              # TODO: o.puts("#[inline]", pad: true)
              # o.block("pub fn almost_equal_elements(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end

              # TODO: o.puts("#[inline]", pad: true)
              # o.block("pub fn almost_equal_elements_relative(x: #{name}, y: #{name}, tolerance: #{scalar}) -> bool") do |o|
              # end
            end

            files << ["#{path}/type_#{name}.rs", io.string]
          end
        end
      end

      files
    end
  end
end
