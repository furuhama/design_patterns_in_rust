// State Pattern
//
// Electronic Dices::
// Push button -> Power on, Start spinning
// Push button -> Stop spinning (The number will fix)
// Push button -> Power off, Back to first state

use std::collections::HashMap;

pub fn state() {
    let mut hmap = HashMap::new();
    hmap.insert(StateDice::PowerOn, Box::new(StatePowerOn) as Box<State>);
    hmap.insert(StateDice::StopDice, Box::new(StateStop) as Box<State>);
    hmap.insert(StateDice::PowerOff, Box::new(StatePowerOff) as Box<State>);

    let hmap = &hmap;

    let mut context = StateContext::new();

    context.press_button(hmap);
    assert_eq!(context.number, None);
    assert_eq!(context.current_state, StateDice::StopDice);

    context.press_button(hmap);
    assert_eq!(context.number, Some(4));
    assert_eq!(context.current_state, StateDice::PowerOff);

    context.press_button(hmap);
    assert_eq!(context.number, Some(4));
    assert_eq!(context.current_state, StateDice::PowerOn);
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum StateDice {
    PowerOn,
    StopDice,
    PowerOff,
}

trait State {
    fn on_press_button(&self, &mut StateContext);
}

struct StatePowerOn;
impl State for StatePowerOn {
    fn on_press_button(&self, context: &mut StateContext) {
        println!("Power on & The dice starts spinning.");

        context.set_state(StateDice::StopDice);
    }
}

struct StateStop;
impl State for StateStop {
    fn on_press_button(&self, context: &mut StateContext) {
        println!("The Dice stopped.");

        context.set_dice_number(4);

        context.set_state(StateDice::PowerOff);
    }
}

struct StatePowerOff;
impl State for StatePowerOff {
    fn on_press_button(&self, context: &mut StateContext) {
        println!("Power off.");

        context.set_state(StateDice::PowerOn);
    }
}

#[derive(Debug)]
struct StateContext {
    number: Option<u8>,
    current_state: StateDice,
}

impl StateContext {
    fn new() -> StateContext {
        StateContext {
            number: None,
            current_state: StateDice::PowerOn,
        }
    }

    fn set_state(&mut self, s: StateDice) {
        self.current_state = s;
    }

    fn set_dice_number(&mut self, n: u8) {
        self.number = Some(n);
    }

    fn press_button<'a>(&mut self, hmap: &HashMap<StateDice, Box<State + 'a>>) {
        let b = hmap.get(&self.current_state).unwrap();
        b.on_press_button(self);
    }
}
