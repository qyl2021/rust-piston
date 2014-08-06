
extern crate piston;
extern crate event;

use piston::{
    Update,
    UpdateArgs,
};
use event::{
    Action,
    Cursor,
    Sequence,
    StartState,
    Wait,
    WhenAll,
    While,
};

/// Some test actions.
pub enum TestActions {
    /// Increment accumulator.
    Inc,
    /// Decrement accumulator.
    Dec,
}

impl StartState<()> for TestActions {
    fn start_state(&self) {}
}

// A test state machine that can increment and decrement.
fn exec(mut acc: u32, dt: f64, cursor: &mut Cursor<TestActions, ()>) -> u32 {
    cursor.update(&Update(UpdateArgs { dt: dt }), |dt, action, _| {
        match *action {
            Inc => { acc += 1; (event::Success, dt) },
            Dec => { acc -= 1; (event::Success, dt) },
        }
    });
    acc
}

// Each action that terminates immediately
// consumes a time of 0.0 seconds.
// This makes it possible to execute one action
// after another without delay or waiting for next update.
fn print_2() {
    let a: u32 = 0;
    let seq = Sequence(vec![Action(Inc), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 0.0, &mut cursor);
    assert_eq!(a, 2);
}

// If you wait the exact amount before to execute an action,
// it will execute. This behavior makes it easy to predict
// when an action will run.
fn wait_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 1.0, &mut cursor);
    assert_eq!(a, 1);
}

// When we execute half the time and then the other half,
// then the action should be executed.
fn wait_half_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 0);
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 1);
}

// A sequence of wait events is the same as one wait event.
fn wait_two_waits() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(0.5), Wait(0.5), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 1.0, &mut cursor);
    assert_eq!(a, 1);
}

// Increase counter ten times.
fn loop_ten_times() {
    let a: u32 = 0;
    let rep = While(box Wait(50.0), vec![Wait(0.5), Action(Inc), Wait(0.5)]);
    let mut cursor = rep.to_cursor();
    let a = exec(a, 10.0, &mut cursor);
    assert_eq!(a, 10);
}

fn when_all_wait() {
    let a: u32 = 0;
    let all = Sequence(vec![
            // Wait in parallel.
            WhenAll(vec![Wait(0.5), Wait(1.0)]),
            Action(Inc)
        ]);
    let mut cursor = all.to_cursor();
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 0);
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 1);
}

fn main() {
    print_2();
    wait_sec();
    wait_half_sec();
    wait_two_waits();
    loop_ten_times();
    when_all_wait();
}


