# Slow Thinking Icon - Spec
Goals: 60fps, <2ms draw, <5MB VRAM, cross-OS Vulkan, on-device WebLLM
Aim: Visualize AI slow thinking - hourglass sand = token streaming
Architecture: Rust SDF -> WASM -> wgpu (WebGPU/Dawn or Vulkan/ash + MoltenVK)
3D: 360° Y in 8s + 10° X sine tilt, quaternion in WASM push constants
Test: visual regression, Vulkan validation, RenderDoc, cross-OS matrix
