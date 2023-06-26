use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, Date};

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(message: &str);
}

#[wasm_bindgen(raw_module = "../index.js")]
extern "C" {
    fn addIt(m: i32, n: i32) -> i32;
}

#[wasm_bindgen]
pub fn call_js_func() -> i32 {
    // log_str("hello, javascript!")
    addIt(1, 2)
}

#[wasm_bindgen]
pub struct User {
    name: String,
    age: u32
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }

    pub fn print_user(&self) {
        log(format!("name is : {}, age is : {}", self.name, self.age).as_str());
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

#[wasm_bindgen]
pub fn update_message(selector: &str, message: &str) {
    let window = web_sys::window().expect("Failed to load window");
    let document = window.document().expect("Failed to load document");
    let element = document
        .query_selector(selector)
        .expect("Failed to load element");

    if let Some(element) = element {
        element.set_inner_html(message);
    } else {
        panic!("failed to set inner html")
    }
}

fn console_log(message: String) {
    web_sys::console::log_1(&message.into());
}

#[wasm_bindgen]
pub fn print_values(js_number: i32, js_boolean: bool, js_uint8_array: &[u8], js_number_array: Vec<i32>) {
    console_log(format!("js number: {}", js_number));
    console_log(format!("js boolean: {}", js_boolean));

    for item in js_uint8_array {
        console_log(format!("js Uint8Array item: {}", item));
    }

    for item in js_number_array {
        console_log(format!("js number array item: {}", item));
    }
}

#[wasm_bindgen]
pub fn print_js_value(val: JsValue) {
    console_log(format!("{:?}", val));
}

#[wasm_bindgen]
pub fn only_return_error_when_result(count: i32) -> Result<(), JsError> {
    if count > 10 {
        Ok(())
    } else {
        Err(JsError::new("count < 10"))
    }
}

#[wasm_bindgen]
pub fn return_all_when_result(count: i32) -> Result<i32, JsError> {
    if count > 10 {
        Ok(count + 10)
    } else {
        Err(JsError::new("count < 10"))
    }
}

#[wasm_bindgen]
pub fn print_uint8_array(js_arr: Uint8Array) -> Uint8Array {
    // new Uint8Array
    let mut arr = Uint8Array::new_with_length(3);

    // Uint8Array -> vec
    for (index, item) in js_arr.to_vec().iter().enumerate() {
        console_log(format!("{} - the item in js_arr: {}", index, item));
    }

    // Avoid type conversion
    // Use the method of the type itself
    for index in 0..js_arr.length() {
        console_log(format!("{} - the item in js_arr: {}", index, js_arr.get_index(index)));
    }

    // vec -> Uint8Array
    let vec = vec![1, 2, 3];
    let arr2 = Uint8Array::from(vec.as_slice());
    arr = arr2.clone();

    // Use the method of the type itself
    arr.set_index(0, 100);

    arr
}

#[wasm_bindgen]
pub fn return_time() -> f64 {
    let date = Date::new_0();
    date.get_time()
}
