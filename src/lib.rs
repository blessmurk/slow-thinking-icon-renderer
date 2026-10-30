use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct SlowThinkingIcon { rotation_y: f32 }
#[wasm_bindgen]
impl SlowThinkingIcon {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self { Self { rotation_y: 0.0 } }
    pub fn update(&mut self, dt: f32, depth: f32) {
        let speed = if depth > 0.7 { 90.0 } else { 45.0 };
        self.rotation_y = (self.rotation_y + speed * dt) % 360.0;
    }
}
