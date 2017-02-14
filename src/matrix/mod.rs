use ::*;

mod matrix_float2x2;
mod matrix_float3x2;
mod matrix_float4x2;
mod matrix_float2x3;
mod matrix_float3x3;
mod matrix_float4x3;
mod matrix_float2x4;
mod matrix_float3x4;
mod matrix_float4x4;

mod matrix_double2x2;
mod matrix_double3x2;
mod matrix_double4x2;
mod matrix_double2x3;
mod matrix_double3x3;
mod matrix_double4x3;
mod matrix_double2x4;
mod matrix_double3x4;
mod matrix_double4x4;

declare_matrix!(float2x2, float3x2, float4x2, float2);
declare_matrix!(float2x3, float3x3, float4x3, float3);
declare_matrix!(float2x4, float3x4, float4x4, float4);

declare_matrix!(double2x2, double3x2, double4x2, double2);
declare_matrix!(double2x3, double3x3, double4x3, double3);
declare_matrix!(double2x4, double3x4, double4x4, double4);
