mod app;
mod appinit;
mod pages;
mod store;
mod event_bus;
mod event_bus_two;
mod event_bus_account;

use wasm_bindgen::prelude::*;

// use app::App;
// use yewdux::prelude::WithDispatch;

// pub struct DataState {
//     name: String,
//     age: u8
// }

// impl DataState {
//     fn GetName (&self) -> String {
//         // String::from("test")
//         // self.name
//     }
// }

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    
    // let account = DataState {
    //     name: String::from("Indo"),
    //     age: 33
    // };
    
    // yew::start_app::<WithDispatch<App>>();
    yew::start_app::<app::App>();
    // yew::start_app::<appinit::AppInit>();



    // // RUST DOM

    // // Use `web_sys`'s global `window` function to get a handle on the global
    // // window object.
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    // // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_text_content(Some("Hello from Rust!"));

    // body.append_child(&val)?;

    Ok(())
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


#[wasm_bindgen]
pub fn addElementP() -> Result<(), JsValue> {
    // RUST DOM

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    Ok(())
}