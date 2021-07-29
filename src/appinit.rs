// use yew::prelude::*;
// use yewdux::prelude::*;
// use crate::app::App;
// use crate::store::reducer::{
//     AppDispatch,
//     Action,
// };
// use yewtil::NeqAssign;

// pub struct AppInit {
//     dispatch: AppDispatch,
// }

// pub enum Msg {}

// impl Component for AppInit {
//     type Message = Msg;
//     type Properties = AppDispatch;

//     fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
//         AppInit {
//             dispatch
//         }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
//         // false
//         self.dispatch.neq_assign(dispatch)
//     }

//     fn view(&self) -> Html {
//         html! {
//             <>
//                 <WithDispatch<App>/>
//             </>
//         }
//     }
// }
