use std::time::Duration;

use crate::io::file::TypingFileDisplay;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_tick: u64,
        typed_text: String,
        to_type: String,
        words_count: u32,
        typing_information: TypingFileDisplay,
    },
    Menu,
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_tick = 0;
        let typed_text = "".to_owned();
        let to_type = "".to_owned();
        let words_count = 0;
        let typing_information = TypingFileDisplay {
            from: "".to_owned(),
            content: "".to_owned(),
            url: "".to_owned(),
            words_count: 0,
        };

        Self::Initialized {
            duration,
            counter_tick,
            typed_text,
            to_type,
            words_count,
            typing_information,
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
        if let Self::Initialized {
            typed_text,
            to_type,
            ..
        } = self
        {
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

    pub fn to_type(&self) -> Option<Vec<&str>> {
        if let Self::Initialized { to_type, .. } = self {
            Some(to_type.split("/").collect::<Vec<&str>>())
        } else {
            None
        }
    }

    pub fn set_to_type(&mut self, to_type: String) {
        if let Self::Initialized {
            to_type: to_type_mut,
            ..
        } = self
        {
            *to_type_mut = to_type;
        }
    }

    pub fn typing_information(&self) -> Option<TypingFileDisplay> {
        if let Self::Initialized {
            typing_information, ..
        } = self
        {
            Some(typing_information.to_owned())
        } else {
            None
        }
    }

    pub fn set_typing_information(&mut self, typing_information: TypingFileDisplay) {
        if let Self::Initialized {
            typing_information: typing_information_mut,
            ..
        } = self
        {
            *typing_information_mut = typing_information;
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

    pub fn set_message_finished(&mut self) {
        if let Self::Initialized {
            to_type,
            duration,
            words_count,
            ..
        } = self
        {
            let result_text = format!(
                "Finished! Your speed is {:?} WPM",
                60 * *words_count as u64 / duration.as_secs()
            );
            *to_type = result_text;
        }
    }

    pub fn stop_timer(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            *duration = Duration::from_secs(1);
        }
    }

    pub fn set_words_count(&mut self, wc: u32) {
        if let Self::Initialized { words_count, .. } = self {
            *words_count = wc
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
