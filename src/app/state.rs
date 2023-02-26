use std::time::Duration;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        typed_text: String
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let typed_text = "".to_owned();
        Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            typed_text
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn typed_text(&self) -> Option<String> {
        if let Self::Initialized { typed_text, .. } = self {
            Some(typed_text.to_owned())
        } else {
            None
        }
    }

    pub fn add_char(&mut self, c: &char) {
        if let Self::Initialized { typed_text, .. } = self {
            typed_text.push(*c);
        }
    }

    pub fn remove_char(&mut self) {
        if let Self::Initialized { typed_text, .. } = self {
            typed_text.pop();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
