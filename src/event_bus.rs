use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;
// use yew::services::ConsoleService;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    UpdateDataOne(String),
    // LoadAccount(Person)
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new()
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::UpdateDataOne(s) => {
                for sub in self.subscribers.iter() {

                    // ConsoleService::info("this is console log");
                    // println!("asdf");
                    // greet("in event bus");
                    // println!("{:?}", sub);
                    // greet(sub);
                    // println!("{}", std::any::type_name::<T>());
                    // let x = sub.to_string();
                    // greet(x);
                    // greet(format!("{}",sub.to_string()));
                    self.link.respond(*sub, s.clone());
                }
            },
            // Request::LoadAccount(d) => {
            //     for sub in self.subscribers.iter() {
            //         self.link.respond(*sub, d);
            //     }
            // }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}