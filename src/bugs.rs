// This seems to not generate new/something when js_name is used.

#[wasm_bindgen(js_name = B)]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(constructor)]
    pub fn new() -> A {
    	A
    }

    #[wasm_bindgen(js_name = choose)]
	pub fn something(&mut self) {
		// nothing
	}
}