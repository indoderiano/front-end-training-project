use crate::event_bus_two::EventBusTwo;
use yew::agent::Bridged;
use yew::prelude::*;


pub enum Msg {
    NewMessage(String),
}

pub struct SubscriberTwo {
    message: String,
    _producer: Box<dyn Bridge<EventBusTwo>>,
}

impl Component for SubscriberTwo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            message: "No message yet.".to_owned(),
            _producer: EventBusTwo::bridge(link.callback(Msg::NewMessage)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMessage(s) => {
                self.message = s + " signed";
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <strong><p>{ "Subscriber Two" }</p></strong>
                <p>{ &self.message }</p>
            </>
        }
    }
}