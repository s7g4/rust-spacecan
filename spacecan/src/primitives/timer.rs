extern crate alloc;

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    NotStarted,
    AlreadyRunning,
}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerError::NotStarted => write!(f, "Timer not started"),
            TimerError::AlreadyRunning => write!(f, "Timer already running"),
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    start_time: Option<u32>,
    duration: u32,
    running: bool,
}

impl Timer {
    pub fn new(duration: u32) -> Self {
        Timer {
            start_time: None,
            duration,
            running: false,
        }
    }

    pub fn start(&mut self, current_time: u32) -> Result<(), TimerError> {
        if self.running {
            return Err(TimerError::AlreadyRunning);
        }

        self.start_time = Some(current_time);
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.start_time = None;
    }

    pub fn is_expired(&self, current_time: u32) -> bool {
        if !self.running {
            return false;
        }

        if let Some(start) = self.start_time {
            current_time.saturating_sub(start) >= self.duration
        } else {
            false
        }
    }

    pub fn remaining_time(&self, current_time: u32) -> Option<u32> {
        if !self.running {
            return None;
        }

        if let Some(start) = self.start_time {
            let elapsed = current_time.saturating_sub(start);
            if elapsed >= self.duration {
                Some(0)
            } else {
                Some(self.duration - elapsed)
            }
        } else {
            None
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn restart(&mut self, current_time: u32) -> Result<(), TimerError> {
        self.stop();
        self.start(current_time)
    }
}
