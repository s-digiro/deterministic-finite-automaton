use std::collections::HashSet;
use std::hash::Hash;

pub struct DFA<StateType, AlphabetType> {
    states: HashSet<StateType>,
    alphabet: HashSet<AlphabetType>,
    transition: Box<dyn Fn(StateType, AlphabetType) -> StateType>,
    start: StateType,
    accept: HashSet<StateType>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status<S> {
    Accept(S),
    Reject(S),
}

impl<S, A> DFA<S, A>
where
    S: Eq + Hash + Clone,
    A: Eq + Hash + Clone,
{
    pub fn new(
        states: impl IntoIterator<Item = S>,
        alphabet: impl IntoIterator<Item = A>,
        transition: impl Fn(S, A) -> S + 'static,
        start: S,
        accept: impl IntoIterator<Item = S>,
    ) -> DFA<S, A> {
        DFA {
            states: HashSet::from_iter(states.into_iter()),
            alphabet: HashSet::from_iter(alphabet.into_iter()),
            transition: Box::new(transition),
            start,
            accept: HashSet::from_iter(accept.into_iter()),
        }
    }

    pub fn states(&self) -> &HashSet<S> {
        &self.states
    }

    pub fn alphabet(&self) -> &HashSet<A> {
        &self.alphabet
    }

    pub fn transition_function(&self) -> &Box<dyn Fn(S, A) -> S> {
        &self.transition
    }

    pub fn start_state(&self) -> &S {
        &self.start
    }

    pub fn accept_states(&self) -> &HashSet<S> {
        &self.accept
    }

    pub fn input(&self, s: impl IntoIterator<Item = A>) -> Status<S> {
        let mut state = self.start.clone();

        for c in s.into_iter() {
            state = (self.transition)(state, c);
        }

        if self.accept.contains(&state) {
            Status::Accept(state)
        } else {
            Status::Reject(state)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_dfa() {
        let dfa = DFA::new(
            [1, 2, 3],
            ['a', 'b'],
            |state, input| match (state, input) {
                (1, 'a') => 2,
                _ => 3,
            },
            1,
            [2],
        );

        assert!(dfa.states() == &HashSet::from([1, 2, 3]));
        assert!(dfa.alphabet() == &HashSet::from(['a', 'b']));
        assert!(*dfa.start_state() == 1);
        assert!(dfa.accept_states() == &HashSet::from([2]));
    }

    #[test]
    fn run_inputs() {
        let dfa = DFA::new(
            [0, 1, 2],
            ['0', '1'],
            |state, input| match (state, input) {
                (0, '0') => 0,
                (0, '1') => 1,
                (1, '0') => 2,
                (1, '1') => 0,
                (2, '0') => 1,
                (2, '1') => 2,
                _ => panic!("invalid state"),
            },
            0,
            [0],
        );

        assert!(dfa.input("0".chars()) == Status::Accept(0));
        assert!(dfa.input("1".chars()) == Status::Reject(1));
        assert!(dfa.input("11".chars()) == Status::Accept(0));
        assert!(dfa.input("10".chars()) == Status::Reject(2));
        assert!(dfa.input("1001".chars()) == Status::Accept(0));
    }
}
