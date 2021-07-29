use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::store::{
    CounterInput,
    CounterOutput,
    CounterStore,
    State
};

pub enum Msg {
    Output(CounterOutput),
    State(Rc<State>),
    TriggerDispatchIncrement,
}

pub struct StoreGlobal {
    dispatch: Dispatch<CounterStore>,
    state: Rc<State>,
    link: ComponentLink<Self>
}

impl Component for StoreGlobal {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = {
            let on_state = link.callback(Msg::State);
            let on_output = link.callback(Msg::Output);

            Dispatch::bridge(on_state, on_output)
        };
        // Magically increment counter by one for this example.
        dispatch.send(CounterInput::Increment);

        Self {
            dispatch,
            state: Rc::new(State { count: 0 }),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
            Msg::Output(msg) => match msg {
                CounterOutput::Doubled(n) => {
                    println!("Count doubled would be: {}", n);
                    false
                }
            },
            Msg::TriggerDispatchIncrement => {
                self.dispatch.send(CounterInput::Increment);
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let count = self.state.count;
        let onclick = self.dispatch.callback(|_| CounterInput::Increment);
        let triggerdispatch = self.link.callback(|_| Msg::TriggerDispatchIncrement);
        html! {
            <>
            <h1>{ count }</h1>
            <button onclick=onclick>{"+1"}</button>
            <button onclick=triggerdispatch>{"Trigger Dispatch"}</button>
            </>
        }
    }
}

