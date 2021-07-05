use crate::event_bus_account::{EventBusAccount, Person, Request};
use yew::agent::{Bridged, Dispatched, Dispatcher};
use yew::prelude::*;
// use yew::services::ConsoleService;

pub enum Msg {
    LoadData(Person),
    EditData
}

pub struct SubscriberAccount {
    message: String,
    _producer: Box<dyn Bridge<EventBusAccount>>,
    account: Person,
    event_bus: Dispatcher<EventBusAccount>,
    link: ComponentLink<SubscriberAccount>,
}

impl Component for SubscriberAccount {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // EventBus::dispatcher().send(Request::UpdateDataOne("Message created".to_owned()));
        // link.send_message(Msg::CreateMessage);

        // ConsoleService::info("this is console log from subscriber");


        Self {
            message: "No message yet.".to_owned(),
            _producer: EventBusAccount::bridge(link.callback(Msg::LoadData)),
            account: Person {
                name: String::from("no name"),
                age: 0
            },
            event_bus: EventBusAccount::dispatcher(),
            link
        }
        
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::NewMessage(acc) => {
            //     // self.message = s + " signed";
            //     true
            // },
            // Msg::CreateMessage => {
            //     EventBusAccount::dispatcher().send(Request::UpdateDataOne("Message created".to_owned()));
            //     true
            // }
            Msg::LoadData(acc) => {
                self.account = acc;
                true
            },
            Msg::EditData => {
                let data_changed = Person {
                    name: String::from("Superman"),
                    age: 35
                };
                self.event_bus
                    .send(Request::UpdateAccount(data_changed));
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
                <strong><p>{ "Subscriber Account" }</p></strong>
                <p>{ &self.message }</p>
                <p>{ &self.account.name }</p>
                <p>{ &self.account.age }</p>
                <button onclick=self.link.callback(|_| Msg::EditData)>
                    { "PRESS TO CHANGE DATA" }
                </button>
            </>
        }
    }
}