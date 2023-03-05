use std::time::Duration;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_tick: u64,
        typed_text: String,
        to_type: String
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_tick = 0;
        let typed_text = "".to_owned();
        let to_type = "".to_owned();
        Self::Initialized {
            duration,
            counter_tick,
            typed_text,
            to_type
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn increase_duaration_tick(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            *duration = duration.checked_add(Duration::from_secs(1)).unwrap();
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Self::Initialized { typed_text, to_type, .. } = self {
            typed_text == to_type
        } else {
            false
        }
    }

    pub fn is_time_over(&self) -> bool {
        if let Self::Initialized { duration, .. } = self {
            *duration >= Duration::from_secs(60)
        } else {
            false
        }
    }

    pub fn typed_text(&self) -> Option<String> {
        if let Self::Initialized { typed_text, .. } = self {
            Some(typed_text.to_owned())
        } else {
            None
        }
    }

    pub fn to_type(&self) -> Option<String> {
        if let Self::Initialized { to_type, .. } = self {
            Some(to_type.to_owned())
        } else {
            None
        }
    }

    pub fn set_to_type(&mut self, to_type: String) {
        if let Self::Initialized { to_type: to_type_mut, .. } = self {
            *to_type_mut = to_type;
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

    pub fn set_message_timeup(&mut self) {
        if let Self::Initialized { to_type, .. } = self {
            *to_type = "Time is up!".to_owned();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
