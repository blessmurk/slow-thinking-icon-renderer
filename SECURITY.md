# WASL Security Checklists
- WASM-01..12: bounds, fuel metering 100ms, stack guard, CSP wasm-unsafe-eval SRI, WASI deny, no eval, cargo audit, reproducible, zero mem, SBOM
- VK-01..10: spirv-val, no non-semantic info, pipeline cache HMAC, descriptor bounds, zero GPU mem, RenderDoc DEBUG only, MoltenVK pin, VkResult check, limit texture, wgpu sandbox
- LLM-01..10: SHA256 pinned, no remote code GGUF, prompt injection guard, rate limit 50 tok/s, disable tools, on-device only, PII scrub, Worker COOP/COEP, KV 512MB cap, CSP HF only
- APP-01..08: OWASP ASVS L2, npm/cargo audit, signed binaries, SBOM CycloneDX, code signing, no inline scripts, secrets env, no GPU dumps logs
