# deterministic-finite-automaton
Rust implementation of a deterministic finite automaton

[Wikipedia Link](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)

## usage:
```
// Accepts strings that contain 1 twice
let states = [0, 1, 2];
let alphabet = [0, 1];
let transition_fn = |s, c| match (s, c) {
    (0, 0) => 0,
    (0, 1) => 1,
    (1, 0) => 1,
    (1, 1) => 2,
    (2, 0) => 2,
    (2, 1) => 2,
    _ => panic!("Invalid (state, char)"),
};
let start_state = 0
let accept_states = [2];

let dfa = DFA::new(
    states,
    alphabet,
    transition_fn,
    start_state,
    accept_states,
);

assert!(dfa.input([1, 1, 0]) State::Accept(2));
assert!(dfa.input([0]) State::Reject(0));
```
