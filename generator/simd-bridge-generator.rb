#!/usr/bin/env ruby

$: << "#{__dir__}/lib"

require "bridge/output"

module Bridge
  class SIMD
    TYPES = [
      { size: 1, max_width: 4, type: 'i8', opencl: 'char', kind: %i(signed integer), bool: "char" },
      { size: 1, max_width: 4, type: 'u8', opencl: 'uchar', kind: %i(unsigned integer), bool: "char" },
      { size: 2, max_width: 4, type: 'i16', opencl: "short", kind: %i(signed integer), bool: "short" },
      { size: 2, max_width: 4, type: 'u16', opencl: "ushort", kind: %i(unsigned integer), bool: "short" },
      { size: 4, max_width: 4, type: 'i32', opencl: "int", kind: %i(signed integer), bool: "int" },
      { size: 4, max_width: 4, type: 'u32', opencl: "uint", kind: %i(unsigned integer), bool: "int" },
      { size: 4, max_width: 4, type: 'f32', opencl: "float", kind: %i(float), bool: "int", max_matrix_size: 4 },
      { size: 8, max_width: 4, type: 'i64', opencl: "long", kind: %i(signed integer), bool: "long" },
      { size: 8, max_width: 4, type: 'u64', opencl: "ulong", kind: %i(unsigned integer), bool: "long"  },
      { size: 8, max_width: 4, type: 'f64', opencl: "double", kind: %i(float), bool: "long", max_matrix_size: 4 }
    ]

    TYPES_BY_NAME = TYPES.map { |x| [x[:opencl], x] }.to_h

    WIDTHS = [2, 3, 4, 8, 16]

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

          o.puts("use std;", pad: true)
          o.puts("use ::*;")
          o.puts("use ::simd::*;")

          o.block("extern \"platform-intrinsic\"", pad: true) do
            if kind.include?(:integer)
              o.puts("fn simd_shl<T>(x: T, y: T) -> T;", pad: true)
              o.puts("fn simd_shr<T>(x: T, y: T) -> T;")

              o.puts("fn simd_and<T>(x: T, y: T) -> T;", pad: true)
              o.puts("fn simd_or<T>(x: T, y: T) -> T;")
              o.puts("fn simd_xor<T>(x: T, y: T) -> T;")
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
                if kind.include?(:unsigned)
                  o.puts("return self ^ std::#{scalar}::MAX;")
                else
                  o.puts("return self ^ -1;")
                end
              end
            end
          end

          o.block("impl PartialEq for #{name}", pad: true) do |o|
            o.puts("#[inline]", pad: true)
            o.block("fn eq(&self, other: &Self) -> bool") do |o|
              o.puts("return simd::eq(*self, *other).all();")
            end

            o.puts("#[inline]", pad: true)
            o.block("fn ne(&self, other: &Self) -> bool") do |o|
              o.puts("return simd::ne(*self, *other).all();")
            end
          end

          o.block("impl simd::Vector for #{name}", pad: true) do |o|
            o.puts("type Scalar = #{scalar};", pad: true)
            o.puts("type Boolean = #{bool_name};")

            o.puts("type CharVector = char#{width};", pad: true)
            o.puts("type ShortVector = short#{width};")
            o.puts("type IntVector = int#{width};")
            o.puts("type LongVector = long#{width};")

            o.puts("type UCharVector = uchar#{width};", pad: true)
            o.puts("type UShortVector = ushort#{width};")
            o.puts("type UIntVector = uint#{width};")
            o.puts("type ULongVector = ulong#{width};")

            o.puts("type FloatVector = float#{width};", pad: true)
            o.puts("type DoubleVector = double#{width};")

            o.puts("#[inline(always)]", pad: true)
            o.block("fn abs(self) -> Self") do |o|
              if kind.include?(:signed)
                o.puts("let mask = self >> #{attributes.fetch(:size) * 8 - 1};")

                o.puts("return (self ^ mask) - mask;")
              elsif kind.include?(:float)
                o.puts("return simd::bitselect(#{bool_name}::broadcast(std::#{TYPES_BY_NAME[bool][:type]}::MAX), #{name}::broadcast(0.0), self);")
              else
                o.puts("return self;")
              end
            end

            o.puts("#[inline(always)]", pad: true)
            o.block("fn max(self, other: Self) -> Self") do |o|
              if kind.include?(:float)
                result = width.times.map { |i| "self.#{i}.max(other.#{i})" }.join(", ")

                o.puts("return #{name}(#{result});")
              else
                o.puts("return simd::bitselect(simd::gt(other, self), self, other);")
              end
            end

            o.puts("#[inline(always)]", pad: true)
            o.block("fn min(self, other: Self) -> Self") do |o|
              if kind.include?(:float)
                result = width.times.map { |i| "self.#{i}.min(other.#{i})" }.join(", ")

                o.puts("return #{name}(#{result});")
              else
                o.puts("return simd::bitselect(simd::lt(other, self), self, other);")
              end
            end

            o.puts("#[inline(always)]", pad: true)
            o.block("fn reduce_add(self) -> Self::Scalar") do |o|
              case width
              when 2
                o.puts("return self.0 + self.1;")
              when 3
                o.puts("return self.0 + self.1 + self.2;")
              else
                o.puts("return simd::reduce_add(self.lo() + self.hi());")
              end
            end

            o.puts("#[inline(always)]", pad: true)
            o.block("fn reduce_min(self) -> Self::Scalar") do |o|
              case width
              when 2
                if kind.include?(:float)
                  o.puts("return self.0.min(self.1);")
                else
                  o.puts("return std::cmp::min(self.0, self.1);")
                end
              when 3
                if kind.include?(:float)
                  o.puts("return self.2.min(simd::reduce_min(self.lo()));")
                else
                  o.puts("return std::cmp::min(simd::reduce_min(self.lo()), self.2);")
                end
              else
                o.puts("return simd::reduce_min(simd::min(self.lo(), self.hi()));")
              end
            end

            o.puts("#[inline(always)]", pad: true)
            o.block("fn reduce_max(self) -> Self::Scalar") do |o|
              case width
              when 2
                if kind.include?(:float)
                  o.puts("return self.0.max(self.1);")
                else
                  o.puts("return std::cmp::max(self.0, self.1);")
                end
              when 3
                if kind.include?(:float)
                  o.puts("return self.2.max(simd::reduce_max(self.lo()));")
                else
                  o.puts("return std::cmp::max(simd::reduce_max(self.lo()), self.2);")
                end
              else
                o.puts("return simd::reduce_max(simd::max(self.lo(), self.hi()));")
              end
            end

            self.conversion(o, type, "char", width)
            self.conversion(o, type, "char", width, saturate: true)
            self.conversion(o, type, "uchar", width)
            self.conversion(o, type, "uchar", width, saturate: true)
            self.conversion(o, type, "short", width)
            self.conversion(o, type, "short", width, saturate: true)
            self.conversion(o, type, "ushort", width)
            self.conversion(o, type, "ushort", width, saturate: true)
            self.conversion(o, type, "int", width)
            self.conversion(o, type, "int", width, saturate: true)
            self.conversion(o, type, "uint", width)
            self.conversion(o, type, "uint", width, saturate: true)
            self.conversion(o, type, "float", width)
            self.conversion(o, type, "long", width)
            self.conversion(o, type, "long", width, saturate: true)
            self.conversion(o, type, "ulong", width)
            self.conversion(o, type, "ulong", width, saturate: true)
            self.conversion(o, type, "double", width)
          end

          if kind.include?(:float) && [2, 3].include?(width)
            o.block("impl simd::Cross for #{name}", pad: true) do |o|
              o.puts("type CrossProduct = #{type}3;", pad: true)

              o.puts("#[inline(always)]", pad: true)
              o.block("fn cross(self, other: Self) -> Self::CrossProduct") do |o|
                if width == 2
                  o.puts("return #{type}3(0.0, 0.0, self.0 * other.1 - self.1 * other.0);")
                else
                  o.puts("let a = self * #{name}(other.2, other.1, other.0) - #{name}(self.2, self.1, self.0) * other;")
                  o.puts
                  o.puts("return #{name}(a.2, a.1, a.0);")
                end
              end
            end
          end

          o.block("impl simd::Dot for #{name}", pad: true) do |o|
            o.puts("type DotProduct = #{scalar};", pad: true)

            o.puts("#[inline(always)]")
            o.block("fn dot(self, other: Self) -> Self::DotProduct") do |o|
              o.puts("return simd::reduce_add(self * other);")
            end
          end
          
          if kind.include?(:float)
            o.block("impl simd::Float for #{name}", pad: true) do |o|
              o.puts("#[inline(always)]", pad: true)
              o.block("fn copysign(self, magnitude: Self) -> Self") do |o|
                o.puts("return simd::bitselect(#{bool_name}::broadcast(std::#{TYPES_BY_NAME[bool][:type]}::MAX), magnitude, self);")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn sign(self) -> Self") do |o|
                o.puts("let (zero, one) = (#{name}::broadcast(0.0), #{name}::broadcast(1.0));")
                o.puts
                o.puts("return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn sqrt(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.sqrt()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn recip(self) -> Self") do |o|
                o.puts("return 1.0 / self;")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn rsqrt(self) -> Self") do |o|
                o.puts("return self.sqrt().recip();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn fract(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.fract()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn ceil(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.ceil()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn floor(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.floor()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn trunc(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.trunc()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn mix(self, a: Self, b: Self) -> Self") do |o|
                o.puts("return a + self * (b - a);")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn step(self, edge: Self) -> Self") do |o|
                o.puts("return simd::bitselect(simd::lt(self, edge), #{name}::broadcast(1.0), #{name}::broadcast(0.0));")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn smoothstep(self, edge0: Self, edge1: Self) -> Self") do |o|
                o.puts("let t = simd::clamp((self - edge0) / (edge1 - edge0), #{name}::broadcast(0.0), #{name}::broadcast(1.0));")
                o.puts
                o.puts("return t * t * (3.0 - 2.0 * t);")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn sin(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.sin()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn cos(self) -> Self") do |o|
                result = width.times.map { |i| "self.#{i}.cos()" }.join(", ")

                o.puts("return #{name}(#{result});")
              end
            end
          end

          if kind.include?(:float)
            o.block("impl simd::Geometry for #{name}", pad: true) do |o|
              o.puts("#[inline(always)]", pad: true)
              o.block("fn project(self, onto: Self) -> Self") do |o|
                o.puts("return (self.dot(onto) / onto.dot(onto)) * onto;")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn length(self) -> Self::Scalar") do |o|
                o.puts("return self.length_squared().sqrt();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn length_squared(self) -> Self::Scalar") do |o|
                o.puts("return self.dot(self);")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn norm_one(self) -> Self::Scalar") do |o|
                o.puts("return self.abs().reduce_add();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn norm_inf(self) -> Self::Scalar") do |o|
                o.puts("return self.abs().reduce_max();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn distance(self, other: Self) -> Self::Scalar") do |o|
                o.puts("return (self - other).length();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn distance_squared(self, other: Self) -> Self::Scalar") do |o|
                o.puts("return (self - other).length_squared();")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn normalize(self) -> Self") do |o|
                o.puts("return self * simd::rsqrt(#{name}::broadcast(self.length_squared()));")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn reflect(self, n: Self) -> Self") do |o|
                o.puts("return self - 2.0 * self.dot(n) * n;")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn refract(self, n: Self, eta: Self::Scalar) -> Self") do |o|
                o.puts("let dp = self.dot(n);", pad: true)
                o.puts("let k = 1.0 - eta * eta * (1.0 - dp * dp);")

                o.puts("return if k >= 0.0 { eta * self - (eta * dp + k.sqrt()) } else { #{name}::broadcast(0.0) };", pad: true)
              end
            end
          end

          if kind.include?(:integer)
            o.block("impl simd::Integer for #{name}", pad: true) do |o|
              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_and(self) -> Self::Scalar") do |o|
                case width
                when 2
                  o.puts("return self.0 & self.1")
                when 3
                  o.puts("return self.0 & self.1 & self.2")
                else
                  o.puts("return (self.lo() & self.hi()).reduce_and();")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_or(self) -> Self::Scalar") do |o|
                case width
                when 2
                  o.puts("return self.0 | self.1")
                when 3
                  o.puts("return self.0 | self.1 | self.2")
                else
                  o.puts("return (self.lo() | self.hi()).reduce_or();")
                end
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn reduce_xor(self) -> Self::Scalar") do |o|
                case width
                when 2
                  o.puts("return self.0 ^ self.1")
                when 3
                  o.puts("return self.0 ^ self.1 ^ self.2")
                else
                  o.puts("return (self.lo() ^ self.hi()).reduce_xor();")
                end
              end

              constant = kind.include?(:signed) ? "std::#{scalar}::MIN" : "0x8#{"0" * (attributes.fetch(:size) * 2 - 1)}"

              o.puts("#[inline(always)]", pad: true)
              o.block("fn all(self) -> bool") do |o|
                o.puts("return self.reduce_and() & #{constant} != 0;")
              end

              o.puts("#[inline(always)]", pad: true)
              o.block("fn any(self) -> bool") do |o|
                o.puts("return self.reduce_or() & #{constant} != 0;")
              end
            end
          end

          if kind.include?(:signed)
            TYPES.select { |t| t.fetch(:bool) == type }.each do |other|
              other_name = "#{other.fetch(:opencl)}#{width}"

              o.block("impl simd::Select<#{other_name}> for #{name}", pad: true) do |o|
                o.puts("#[inline(always)]", pad: true)
                o.block("fn select(self, a: #{other_name}, b: #{other_name}) -> #{other_name}") do |o|
                  o.puts("return (self >> #{attributes.fetch(:size) * 8 - 1}).bitselect(a, b);")
                end

                o.puts("#[inline(always)]", pad: true)
                o.block("fn bitselect(self, a: #{other_name}, b: #{other_name}) -> #{other_name}") do |o|
                  if name == other_name
                    o.puts("return (a & !self) | (b & self);")
                  else
                    o.puts("return #{other_name}::bitcast(self.bitselect(#{bool_name}::bitcast(a), #{bool_name}::bitcast(b)));")
                  end
                end
              end
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
            o.block("pub fn broadcast(x: #{scalar}) -> Self") do |o|
              o.puts("return #{name}(#{(["x"] * width).join(", ")});")
            end

            # Swizzles

            case width
            when 2
              o.puts("#[inline]", pad: true)
              o.block("pub fn lo(self) -> #{scalar}") do |o|
                o.puts("return self.0;")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn hi(self) -> #{scalar}") do |o|
                o.puts("return self.1;")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn odd(self) -> #{scalar}") do |o|
                o.puts("return self.1;")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn even(self) -> #{scalar}") do |o|
                o.puts("return self.0;")
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

              o.puts("#[inline]", pad: true)
              o.block("pub fn odd(self) -> #{type}2") do |o|
                o.puts("return #{type}2(self.1, 0#{".0" if kind.include?(:float)});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn even(self) -> #{type}2") do |o|
                o.puts("return #{type}2(self.0, self.2);")
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

              o.puts("#[inline]", pad: true)
              o.block("pub fn odd(self) -> #{type}#{width / 2}") do |o|
                o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{2 * i + 1}"}.join(", ")});")
              end

              o.puts("#[inline]", pad: true)
              o.block("pub fn even(self) -> #{type}#{width / 2}") do |o|
                o.puts("return #{type}#{width / 2}(#{(width / 2).times.map { |i| "self.#{2 * i}"}.join(", ")});")
              end
            end
          end

          files << ["#{path}/impl_#{name}.rs", io.string]
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
            o.puts("use ::simd::*;") if i == j

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
              o.block("impl simd::Dot for #{name}", pad: true) do |o|
                o.puts("type DotProduct = #{name};")
                o.puts
                o.puts("#[inline]")
                o.block("fn dot(self, other: #{name}) -> #{name}") do |o|
                  o.puts("return #{name}(#{j.times.map { |k| "self.dot(other.#{k})" }.join(", ")});")
                end
              end

              o.block("impl simd::Dot<#{vector_name}> for #{name}", pad: true) do |o|
                o.puts("type DotProduct = #{vector_name};")
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

              # matrix_multiply is expressed via the `simd::Dot` trait

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

            files << ["#{path}/impl_#{name}.rs", io.string]
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
          min = "#{in_name}::broadcast(std::#{out_scalar}::MIN as #{in_scalar})"
          max = "#{in_name}::broadcast(std::#{out_scalar}::MAX as #{in_scalar})"

          if in_scalar == out_scalar
            o.puts("return self;")
          elsif in_kind == out_kind && in_size < out_size
            o.puts("return #{in_name}::to_#{out_type}(self);")
          elsif in_kind.include?(:signed) && out_kind.include?(:unsigned) && in_size <= out_size
            o.puts("return #{in_name}::to_#{out_type}(simd::max(self, #{in_name}::broadcast(0)));")
          elsif in_kind.include?(:unsigned)
            o.puts("return #{in_name}::to_#{out_type}(simd::min(self, #{max}));")
          else
            o.puts("return #{in_name}::to_#{out_type}(simd::clamp(self, #{min}, #{max}));")
          end
        end
      else
        if width == 3 && !in_kind.include?(:float) && in_size < out_size # TODO: Fix this compiler bug
          o.puts("#[inline(always)]", pad: true)
          o.block("fn to_#{out_type}(self) -> #{out_name}") do |o|
            o.puts("return #{out_name}(self.0 as #{out_scalar}, self.1 as #{out_scalar}, self.2 as #{out_scalar});")
          end
        end
      end
    end
  end
end
