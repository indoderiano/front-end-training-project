use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;
use yew::services::ConsoleService;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    LoadAccount,
    UpdateAccount(Person)
}

pub struct EventBusAccount {
    link: AgentLink<EventBusAccount>,
    subscribers: HashSet<HandlerId>,
    account: Person
}

impl Agent for EventBusAccount {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = Person;

    fn create(link: AgentLink<Self>) -> Self {
        let data  = Person {
            name: String::from("Indo"),
            age: 33
        };

        Self {
            link,
            subscribers: HashSet::new(),
            account: data
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::LoadAccount => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, self.account.clone());
                }
            },
            Request::UpdateAccount(acc) => {
                self.account = acc.clone();
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, acc.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        ConsoleService::info("a component is connected");
        // ConsoleService::info(format!("{}", id.raw_id()));
        // let dis = id.to_string();
        // ConsoleService::info(format!("{}", &dis[..]));

        
        // After a component is connected, data is shared immediately
        self.link.respond(id.clone(), self.account.clone());
        
        
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}