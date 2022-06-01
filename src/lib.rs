use deno_bindgen::deno_bindgen;

#[deno_bindgen]
fn say_hello() -> String {
    String::from("Hello, world!")
}
