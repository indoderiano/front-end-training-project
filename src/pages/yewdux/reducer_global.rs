use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

use crate::store::reducer::{
    AppDispatch,
    Action,
};

pub enum Msg {
    TriggerDispatchIncrement,
}

pub struct ReducerGlobal {
    dispatch: AppDispatch,
    link: ComponentLink<Self>,
}

impl Component for ReducerGlobal {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Magically increment counter for this example.
        dispatch.send(Action::Increment);

        Self { 
            dispatch,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TriggerDispatchIncrement => {
                self.dispatch.send(Action::Increment);
                true
            }
        }
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let count = self.dispatch.state().count;
        let increment = self.dispatch.callback(|_| Action::Increment);
        let triggerdispatch = self.link.callback(|_| Msg::TriggerDispatchIncrement);
        html! {
            <>
                <h1>{ count }</h1>
                <button onclick=increment>{"+1"}</button>
                <button onclick=triggerdispatch>{"Trigger Dispatch"}</button>
            </>
        }
    }
}