use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use yew::services::ConsoleService;

#[derive(Default, Clone)]
pub struct State {
    count: u32,
    title: String,
}

pub struct DispatchComponent {
    /// Our local version of state.
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
}

pub enum Msg {
    /// Message to receive new state.
    State(Rc<State>),
}

impl Component for DispatchComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Create Dispatch with a bridge that receives new state.
        let dispatch = Dispatch::bridge_state(link.callback(Msg::State));
        // Magically increment our counter for this example.
        // NOTE: Changes aren't immediate! We won't see new state until we receive it in our update
        // method.
        // dispatch.reduce(|s| s.count += 1);

        // let check = Rc::new( State {
        //     title: String::from("check title"),
        //     count: 1
        // });

        // ConsoleService::info("check title in create");
        // ConsoleService::info(&check.title);
        // ConsoleService::info(&check.count.to_string());

        // dispatch.send(msg: impl Into<<Self::Store as Store>::Input>)

        Self {
            dispatch,
            state: Rc::new( State {
                count: 2,
                title: String::from("Title")
            })
            // state: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::info("state is updated");
        // let s: &str = &self.state.title.clone();
        // ConsoleService::info(s);
        match msg {
            Msg::State(state) => {
                // Receive new state and re-render.
                // ConsoleService::info("received new state");
                // ConsoleService::info(&self.state.title.clone());
                // ConsoleService::info(&state.count.to_string());
                // ConsoleService::info(&state.title.clone());
                self.state = state;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let count = self.state.count;
        // We can modify state with callbacks too!
        let incr = self.dispatch.reduce_callback(|s| {
            // let s: &str = &s.title.clone();
            // ConsoleService::info("checking title from incr");
            // ConsoleService::info(&s.title.clone());
            // ConsoleService::info(&s.count.to_string());
            // s.title = s.title.clone();
            s.title = String::from("Updated from view");
            s.count += 1;
        });

        // ConsoleService::info("checking title");
        // ConsoleService::info(&self.state.title.clone());
        // ConsoleService::info(&self.state.count.to_string());

        html! {
            <>
            <h4>{ "Using Dispatch" }</h4>
            <p>{ &self.state.title.clone() }</p>
            <p>{ "Testing" }</p>
            <h1>{ count }</h1>
            <button onclick=incr>{"+1"}</button>
            </>
        }
    }
}



// QUESTION
// CANNOT MAKE DEFAULT STATE
// COUNT WILL ALWAYS START FROM 0