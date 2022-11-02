use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/vanillajs.js")]
extern "C" {
    #[wasm_bindgen(js_name = Main)]
    pub type Main;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Main;

    #[wasm_bindgen(method, js_class = "Main", js_name = deleteAll)]
    pub fn delete_all(this: &Main, index: u32);

    #[wasm_bindgen(method, js_class = "Main", js_name = selectAll)]
    pub fn select_all(this: &Main, index: u32);

    #[wasm_bindgen(method, js_class = "Main", js_name = run)]
    pub fn run(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = runLots)]
    pub fn run_lots(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = clear)]
    pub fn clear(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = swapLots)]
    pub fn swap_lots(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = selectLots)]
    pub fn select_lots(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = deleteLots)]
    pub fn delete_lots(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = add)]
    pub fn add(this: &Main);

    #[wasm_bindgen(method, js_class = "Main", js_name = update)]
    pub fn update(this: &Main);
}
