use std::time::Instant;

#[derive(Debug, PartialEq)]
pub enum TimerState {
    Idle,
    Running {
        started_at: Instant,
        accumulated_seconds: u64
    },
    Paused {
        accumulated_seconds: u64
    }
}

pub enum TimerEvent {
    Play,
    Stop,
    Pause,
    Resume
}

impl TimerState {
    pub fn transition(self, event: TimerEvent) -> Self {
        match (self, event) {
            (TimerState::Idle, TimerEvent::Play) => TimerState::Running {
                started_at: Instant::now(),
                accumulated_seconds: 0
            },
            
            (TimerState::Running {
                started_at,
                accumulated_seconds
            }, TimerEvent::Pause) => TimerState::Paused {
                accumulated_seconds: started_at.elapsed().as_secs() + accumulated_seconds
            },
            
            (TimerState::Paused {
                accumulated_seconds
            }, TimerEvent::Resume) => TimerState::Running {
                started_at: Instant::now(),
                accumulated_seconds
            },
            
            (_, TimerEvent::Stop) => TimerState::Idle,
            (state, _) => state
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case::idle(TimerState::Idle)]
    #[case::running(TimerState::Running {
        started_at: Instant::now(),
        accumulated_seconds: 10
    })]
    #[case::paused(TimerState::Paused {
        accumulated_seconds: 10
    })]
    fn stop_transitions_to_idle(#[case] state: TimerState) {
        // Act
        let result = state.transition(TimerEvent::Stop);

        // Assert
        assert_eq!(result, TimerState::Idle)
    }    
}