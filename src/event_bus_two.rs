use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    UpdateDataOne(String),
}

pub struct EventBusTwo {
    link: AgentLink<EventBusTwo>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBusTwo {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::UpdateDataOne(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}