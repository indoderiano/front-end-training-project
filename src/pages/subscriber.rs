use crate::event_bus::{EventBus, Request};
use yew::agent::{Bridged};
use yew::prelude::*;
use yew::services::ConsoleService;

pub enum Msg {
    NewMessage(String),
    CreateMessage,
}

pub struct Subscriber {
    message: String,
    _producer: Box<dyn Bridge<EventBus>>,
    // event_bus: Dispatcher<EventBus>,
}

impl Component for Subscriber {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // EventBus::dispatcher().send(Request::UpdateDataOne("Message created".to_owned()));
        link.send_message(Msg::CreateMessage);

        ConsoleService::info("this is console log from subscriber");

        Self {
            message: "No message yet.".to_owned(),
            _producer: EventBus::bridge(link.callback(Msg::NewMessage)),
            // event_bus: EventBus::dispatcher(),
        }
        
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMessage(s) => {
                self.message = s + " signed";
                true
            },
            Msg::CreateMessage => {
                EventBus::dispatcher().send(Request::UpdateDataOne("Message created".to_owned()));
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
                <strong><p>{ "Subscriber" }</p></strong>
                <p>{ &self.message }</p>
            </>
        }
    }
}