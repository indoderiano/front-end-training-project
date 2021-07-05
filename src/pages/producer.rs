use crate::event_bus::{EventBus, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::prelude::*;
use yew::services::ConsoleService;

pub enum Msg {
    Clicked,
}

pub struct Producer {
    link: ComponentLink<Producer>,
    event_bus: Dispatcher<EventBus>,
}

impl Component for Producer {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            link,
            event_bus: EventBus::dispatcher(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // usage
        ConsoleService::info("this is console log");
        match msg {
            Msg::Clicked => {
                self.event_bus
                    .send(Request::UpdateDataOne("Message received".to_owned()));
                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Clicked)>
                { "PRESS ME" }
            </button>
        }
    }
}
