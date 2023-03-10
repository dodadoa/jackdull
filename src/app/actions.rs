use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::key::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Typing(char),
    BackwardDeleteChar,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 57] = [
            Action::Quit,
            Action::Typing('a'),
            Action::Typing('b'),
            Action::Typing('c'),
            Action::Typing('d'),
            Action::Typing('e'),
            Action::Typing('f'),
            Action::Typing('g'),
            Action::Typing('h'),
            Action::Typing('i'),
            Action::Typing('j'),
            Action::Typing('k'),
            Action::Typing('l'),
            Action::Typing('m'),
            Action::Typing('n'),
            Action::Typing('o'),
            Action::Typing('p'),
            Action::Typing('q'),
            Action::Typing('r'),
            Action::Typing('s'),
            Action::Typing('t'),
            Action::Typing('u'),
            Action::Typing('v'),
            Action::Typing('w'),
            Action::Typing('x'),
            Action::Typing('y'),
            Action::Typing('z'),
            Action::Typing('A'),
            Action::Typing('B'),
            Action::Typing('C'),
            Action::Typing('D'),
            Action::Typing('E'),
            Action::Typing('F'),
            Action::Typing('G'),
            Action::Typing('H'),
            Action::Typing('I'),
            Action::Typing('J'),
            Action::Typing('K'),
            Action::Typing('L'),
            Action::Typing('M'),
            Action::Typing('N'),
            Action::Typing('O'),
            Action::Typing('P'),
            Action::Typing('Q'),
            Action::Typing('R'),
            Action::Typing('S'),
            Action::Typing('T'),
            Action::Typing('U'),
            Action::Typing('V'),
            Action::Typing('W'),
            Action::Typing('X'),
            Action::Typing('Y'),
            Action::Typing('Z'),
            Action::Typing(' '),
            Action::Typing('.'),
            Action::Typing('\''),
            Action::BackwardDeleteChar,
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Ctrl('c')],
            Action::Typing('a') => &[Key::Char('a')],
            Action::Typing('b') => &[Key::Char('b')],
            Action::Typing('c') => &[Key::Char('c')],
            Action::Typing('d') => &[Key::Char('d')],
            Action::Typing('e') => &[Key::Char('e')],
            Action::Typing('f') => &[Key::Char('f')],
            Action::Typing('g') => &[Key::Char('g')],
            Action::Typing('h') => &[Key::Char('h')],
            Action::Typing('i') => &[Key::Char('i')],
            Action::Typing('j') => &[Key::Char('j')],
            Action::Typing('k') => &[Key::Char('k')],
            Action::Typing('l') => &[Key::Char('l')],
            Action::Typing('m') => &[Key::Char('m')],
            Action::Typing('n') => &[Key::Char('n')],
            Action::Typing('o') => &[Key::Char('o')],
            Action::Typing('p') => &[Key::Char('p')],
            Action::Typing('q') => &[Key::Char('q')],
            Action::Typing('r') => &[Key::Char('r')],
            Action::Typing('s') => &[Key::Char('s')],
            Action::Typing('t') => &[Key::Char('t')],
            Action::Typing('u') => &[Key::Char('u')],
            Action::Typing('v') => &[Key::Char('v')],
            Action::Typing('w') => &[Key::Char('w')],
            Action::Typing('x') => &[Key::Char('x')],
            Action::Typing('y') => &[Key::Char('y')],
            Action::Typing('z') => &[Key::Char('z')],
            Action::Typing(' ') => &[Key::Char(' ')],
            Action::Typing('A') => &[Key::Char('A')],
            Action::Typing('B') => &[Key::Char('B')],
            Action::Typing('C') => &[Key::Char('C')],
            Action::Typing('D') => &[Key::Char('D')],
            Action::Typing('E') => &[Key::Char('E')],
            Action::Typing('F') => &[Key::Char('F')],
            Action::Typing('G') => &[Key::Char('G')],
            Action::Typing('H') => &[Key::Char('H')],
            Action::Typing('I') => &[Key::Char('I')],
            Action::Typing('J') => &[Key::Char('J')],
            Action::Typing('K') => &[Key::Char('K')],
            Action::Typing('L') => &[Key::Char('L')],
            Action::Typing('M') => &[Key::Char('M')],
            Action::Typing('N') => &[Key::Char('N')],
            Action::Typing('O') => &[Key::Char('O')],
            Action::Typing('P') => &[Key::Char('P')],
            Action::Typing('Q') => &[Key::Char('Q')],
            Action::Typing('R') => &[Key::Char('R')],
            Action::Typing('S') => &[Key::Char('S')],
            Action::Typing('T') => &[Key::Char('T')],
            Action::Typing('U') => &[Key::Char('U')],
            Action::Typing('V') => &[Key::Char('V')],
            Action::Typing('W') => &[Key::Char('W')],
            Action::Typing('X') => &[Key::Char('X')],
            Action::Typing('Y') => &[Key::Char('Y')],
            Action::Typing('Z') => &[Key::Char('Z')],
            Action::Typing('.') => &[Key::Char('.')],
            Action::Typing('\'') => &[Key::Char('\'')],
            Action::BackwardDeleteChar => &[Key::Backspace],
            _ => panic!("should not reach"),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Typing('a') => "a",
            Action::Typing('b') => "b",
            Action::Typing('c') => "c",
            Action::Typing('d') => "d",
            Action::Typing('e') => "e",
            Action::Typing('f') => "f",
            Action::Typing('g') => "g",
            Action::Typing('h') => "h",
            Action::Typing('i') => "i",
            Action::Typing('j') => "j",
            Action::Typing('k') => "k",
            Action::Typing('l') => "l",
            Action::Typing('m') => "m",
            Action::Typing('n') => "n",
            Action::Typing('o') => "o",
            Action::Typing('p') => "p",
            Action::Typing('q') => "q",
            Action::Typing('r') => "r",
            Action::Typing('s') => "s",
            Action::Typing('t') => "t",
            Action::Typing('u') => "u",
            Action::Typing('v') => "v",
            Action::Typing('w') => "w",
            Action::Typing('x') => "x",
            Action::Typing('y') => "y",
            Action::Typing('z') => "z",
            Action::Typing('A') => "A",
            Action::Typing('B') => "B",
            Action::Typing('C') => "C",
            Action::Typing('D') => "D",
            Action::Typing('E') => "E",
            Action::Typing('F') => "F",
            Action::Typing('G') => "G",
            Action::Typing('H') => "H",
            Action::Typing('I') => "I",
            Action::Typing('J') => "J",
            Action::Typing('K') => "K",
            Action::Typing('L') => "L",
            Action::Typing('M') => "M",
            Action::Typing('N') => "N",
            Action::Typing('O') => "O",
            Action::Typing('P') => "P",
            Action::Typing('Q') => "Q",
            Action::Typing('R') => "R",
            Action::Typing('S') => "S",
            Action::Typing('T') => "T",
            Action::Typing('U') => "U",
            Action::Typing('V') => "V",
            Action::Typing('W') => "W",
            Action::Typing('X') => "X",
            Action::Typing('Y') => "Y",
            Action::Typing('Z') => "Z",
            Action::Typing(' ') => " ",
            Action::Typing('.') => ".",
            Action::Typing('\'') => "\'",

            Action::BackwardDeleteChar => "remove char",
            _ => panic!("should not reach"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        Self(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_action_by_key() {
        let actions: Actions = vec![Action::Quit].into();
        let result = actions.find(Key::Ctrl('c'));
        assert_eq!(result, Some(&Action::Quit));
    }

    #[test]
    fn should_find_action_by_key_not_found() {
        let actions: Actions = vec![Action::Quit].into();
        let result = actions.find(Key::Alt('w'));
        assert_eq!(result, None);
    }

    #[test]
    fn should_create_actions_from_vec() {
        let _actions: Actions = vec![Action::Quit].into();
    }

    #[test]
    #[should_panic]
    fn should_panic_when_create_actions_conflict_key() {
        let _actions: Actions = vec![Action::Quit, Action::Quit].into();
    }
}
