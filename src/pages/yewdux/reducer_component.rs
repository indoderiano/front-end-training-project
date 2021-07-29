use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

pub enum Action {
    Increment,
}

#[derive(Clone)]
pub struct Counter {
    count: u64,
}

impl Reducer for Counter {
    type Action = Action;

    fn new() -> Self {
        Self { count: 2 }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::Increment => {
                self.count += 1;
                true
            }
        }
    }
}

pub type AppDispatch = DispatchProps<ReducerStore<Counter>>;

pub struct ReducerComponent {
    dispatch: AppDispatch,
}

impl Component for ReducerComponent {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // Magically increment counter for this example.
        // dispatch.send(Action::Increment);

        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let count = self.dispatch.state().count;
        let increment = self.dispatch.callback(|_| Action::Increment);
        html! {
            <>
                <h1>{ count }</h1>
                <button onclick=increment>{"+1"}</button>
            </>
        }
    }
}