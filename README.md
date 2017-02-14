# Hagane SIMD vector library #

This is a crate that provides an SIMD vector library for Rust and specifically the ObjC bridge `hagane`, it's nowhere near as polished or complete as the language support in Clang / OpenCL but it's intended to be a start to get us out and be productive at least.

The main goal is to be ABI compatible with the Apple standard library `simd` for use on macOS and iOS. Right now it only builds on Apple platforms since we depend on `libSystem`, but the goal is to replace those bits as soon as possible.

## Build the source ##

Right now you can regenerate the source code from the Ruby script via `rake`, this is a temporary measure until the point I am happy with the output and try to write optimisations. Then we'll fossilise the code and remove the generator.

# TODO #

This is a TODO list, a check means that it is "done".

 - [ ] Vectors
   - [ ] `half` type (blocked on compiler)
   - [ ] `rint` function (blocked on stdlib)
   - [ ] Vector and scalars constructors (blocked on compiler)
   - [ ] Vector and vector constructors (blocked on compiler)
   - [ ] Arbitrary mix constructors (blocked on compiler)
   - [ ] Swizzling (partially blocked on compiler)
     - [ ] `xyzw` specifier
     - [ ] `rgba` specifier
     - [ ] `s0123456789abcdef` specifier
     - [ ] `s0123456789ABCDEF` specifier
   - [ ] Constants
   - [ ] Extended Math Functions (From OpenCL)
     - [ ] `acos`
     - [ ] `acosh`
     - [ ] `acospi`
     - [ ] `asin`
     - [ ] `asinh`
     - [ ] `asinpi`
     - [ ] `atan`
     - [ ] `atan2`
     - [ ] `atanh`
     - [ ] `atanpi`
     - [ ] `atan2pi`
     - [ ] `cbrt`
     - [ ] `cosh`
     - [ ] `cospi`
     - [ ] `erfc`
     - [ ] `erf`
     - [ ] `exp`
     - [ ] `exp2`
     - [ ] `exp10`
     - [ ] `expm1`
     - [ ] `fabs`
     - [ ] `fdim`
     - [ ] `fma`
     - [ ] `fmod`
     - [ ] `fract`
     - [ ] `frexp`
     - [ ] `frexp` (vector / scalar)
     - [ ] `hypot`
     - [ ] `ilogb`
     - [ ] `ldexp`
     - [ ] `lgamma`
     - [ ] `lgamma_r`
     - [ ] `log`
     - [ ] `log2`
     - [ ] `log10`
     - [ ] `log1p`
     - [ ] `logb`
     - [ ] `maxmag`
     - [ ] `minmag`
     - [ ] `modf`
     - [ ] `nan`
     - [ ] `nextafter`
     - [ ] `pow`
     - [ ] `pown`
     - [ ] `powr`
     - [ ] `remainder`
     - [ ] `remquo`
     - [ ] `rootn`
     - [ ] `rsqrt`
     - [ ] `sincos`
     - [ ] `sinh`
     - [ ] `sinpi`
     - [ ] `tan`
     - [ ] `tanh`
     - [ ] `tanpi`
     - [ ] `tgamma`
 - [ ] Matrices
   - [ ] Constructors
     - [x] Basic
     - [ ] From columns
     - [ ] From rows
     - [ ] From diagonal
   - [ ] Functions
     - [ ] `equal`
     - [ ] `almost_equal_elements`
     - [ ] `almost_equal_elements_relative`
