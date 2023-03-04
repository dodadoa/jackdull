use log::{debug, error, warn};

use self::actions::Actions;
use self::state::AppState;
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    state: AppState,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Typing(c) => {
                    self.state.add_char(c);
                    AppReturn::Continue
                },
                Action::BackwardDeleteChar => {
                    self.state.remove_char();
                    AppReturn::Continue
                },
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
        self.state.increase_duaration_tick();
        AppReturn::Continue
    }

    pub async fn tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub async fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialized(&mut self) {
        self.actions = vec![
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
            Action::BackwardDeleteChar
        ]
        .into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn load_text(&mut self, text: String) {
        self.state.set_to_type(text)
    }
}
