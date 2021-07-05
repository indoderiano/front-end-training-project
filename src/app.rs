use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;
// use yew::services::ConsoleService;
use yewdux::prelude::WithDispatch;

use crate::pages::{
    home::Home,
    details::Details,
    subscriber::Subscriber,
    subscribertwo::SubscriberTwo,
    producer::Producer,
    producertwo::ProducerTwo,
    
    subscriber_account::SubscriberAccount,

    yewdux::dispatch_component::DispatchComponent,
    yewdux::dispatch_props::DispatchPropsComponent,
    yewdux::reducer_component::ReducerComponent,
};

#[derive(Switch, Clone)]
enum Route {
    #[to = "/details"]
    Details,
    #[to = "/"]
    Home,
}

pub struct App {
    // console: ConsoleService,
}

pub enum Msg {}


// pub struct DataState {
//     name: String,
//     age: u8
// }

// let a = DataState {
//     name: String::from("Indo"),
//     age: 33
// };

impl Component for App {
    type Message = Msg;
    type Properties = ();
    
    // type Properties = DataState {
    //     name: String::from("indo"),
    //     age: 33
    // };

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        println!("testing yew");
        // let account = DataState{
        //     name: String::from("indo"),
        //     age: 33,
        // };
        App {
            // console: ConsoleService{}
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // self.console.debug("Using console");
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::Home => html! {<Home/>},
            Route::Details => html! {<Details/>},
        });
        type Anchor = RouterAnchor<Route>;
        html! {
            <div>
                <p>{ "Hello world!" }</p>
                <Anchor route=Route::Home classes="item">
                  {"Home"}
                </Anchor>
                <Anchor route=Route::Details classes="item">
                  {"Details"}
                </Anchor>
                <main>
                    <Router<Route, ()> render=render/>
                </main>


                <p></p>
                <Subscriber/>
                <p></p>
                <Producer/>

                <p></p>
                <SubscriberTwo/>
                <p></p>
                <ProducerTwo/>
                
                <h1>{"Account"}</h1>
                <SubscriberAccount/>
                // <p>{self::Properties}</p>
                // <p>{"Name: ", }

                <p></p>
                <h1>{"Yewdux"}</h1>
                <DispatchComponent/>
                <WithDispatch<DispatchPropsComponent>/>
                <p>{"Reducer"}</p>
                <WithDispatch<ReducerComponent>/>
            </div>
        }
    }
}



// pub struct DataState {
//     name: String,
//     age: u8
// }

// let account = DataState {
//     name: String::from("Indo"),
//     age: 33
// };