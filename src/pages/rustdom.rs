use yew::prelude::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn addElement() -> Result<(), JsValue> {
    // RUST DOM

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_text_content(Some("Hello from Rust!"));

    // body.append_child(&val)?;


    
    // SCROLL TO ID
    // document.getElementById("divFirst").scrollIntoView();
    let element_home = document.get_element_by_id("home").expect("should have element id");

    element_home.scroll_into_view();

    Ok(())
}


pub struct RustDom {
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddElement,
}

impl Component for RustDom {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        RustDom {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddElement => {
                addElement();
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button
                    onclick=self.link.callback(|_| Msg::AddElement)
                >
                    { "click me to add element" }
                </button>
            </div>
        }
    }
}