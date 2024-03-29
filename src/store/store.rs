use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Clone)]
pub struct State {
    pub count: u32,
}

pub enum CounterInput {
    /// Increment count by one.
    Increment,
}

pub enum CounterOutput {
    /// Output the current count but doubled.
    Doubled(u32),
}

pub struct CounterStore {
    state: Rc<State>,
    link: StoreLink<Self>,
}

impl Store for CounterStore {
    type Model = State;
    type Message = ();
    type Input = CounterInput;
    type Output = CounterOutput;

    fn new(link: StoreLink<Self>) -> Self {
        Self {
            link,
            state: Rc::new(State { count: 0 }),
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) -> Changed {
        let state = Rc::make_mut(&mut self.state);
        match msg {
            CounterInput::Increment => {
                state.count += 1;
                // Response with current count doubled.
                self.link
                    .respond(who, CounterOutput::Doubled(state.count * 2));
            }
        }

        true
    }
}