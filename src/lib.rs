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