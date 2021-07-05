use crate::event_bus_two::{EventBusTwo, Request};
use yew::agent::{Dispatched, Dispatcher};
use yew::prelude::*;

pub enum Msg {
    Clicked,
}

pub struct ProducerTwo {
    link: ComponentLink<ProducerTwo>,
    event_bus: Dispatcher<EventBusTwo>,
}

impl Component for ProducerTwo {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            event_bus: EventBusTwo::dispatcher(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                { "PRESS ME Two" }
            </button>
        }
    }
}
