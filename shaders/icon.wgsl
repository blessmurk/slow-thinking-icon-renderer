@group(0) @binding(0) var<uniform> rot: mat4x4<f32>;
@vertex fn vs(@location(0) p: vec3<f32>) -> @builtin(position) vec4<f32> { return rot * vec4(p,1.0); }
@fragment fn fs() -> @location(0) vec4<f32> { return vec4(0.29,0.48,1.0,1.0); }
