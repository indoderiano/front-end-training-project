use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

#[derive(Default, Clone)]
pub struct State {
    count: u32,
}

pub struct DispatchPropsComponent {
    dispatch: DispatchProps<BasicStore<State>>,
}

impl Component for DispatchPropsComponent {
    type Message = ();
    type Properties = DispatchProps<BasicStore<State>>;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
        let onclick = self.dispatch.reduce_callback(|s| s.count += 1);
        html! {
            <>
            <h4>{ "Using DispatchProps" }</h4>
            <h1>{ count }</h1>
            <button onclick=onclick>{"+1"}</button>
            </>
        }
    }
}

// pub fn main() {
//     // IMPORTANT: Don't forget to wrap your component in `WithDispatch` or it will panic!
//     yew::start_app::<WithDispatch<App>>();
// }