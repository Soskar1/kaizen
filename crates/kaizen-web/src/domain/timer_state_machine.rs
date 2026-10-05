// Need to use web_time instead of std, because Instant from std panics in web app
// https://crates.io/crates/web-time
use web_time::Instant;

#[derive(Debug, PartialEq, Clone)]
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
        self.transition_at(event, Instant::now())
    }

    fn transition_at(self, event: TimerEvent, now: Instant) -> Self {
        match (self, event) {
            (TimerState::Idle, TimerEvent::Play) => TimerState::Running {
                started_at: now,
                accumulated_seconds: 0
            },
            
            (TimerState::Running {
                started_at,
                accumulated_seconds
            }, TimerEvent::Pause) => TimerState::Paused {
                accumulated_seconds: now.duration_since(started_at).as_secs() + accumulated_seconds
            },
            
            (TimerState::Paused {
                accumulated_seconds
            }, TimerEvent::Resume) => TimerState::Running {
                started_at: now,
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
        // Arrange & Act
        let result = state.transition(TimerEvent::Stop);

        // Assert
        assert_eq!(result, TimerState::Idle)
    }

    #[test]
    fn play_transitions_to_running() {
        // Arrange
        let state = TimerState::Idle;
        let now = Instant::now();

        // Act
        let result = state.transition_at(TimerEvent::Play, now);

        // Assert
        assert_eq!(result, TimerState::Running { started_at: now, accumulated_seconds: 0 })
    }

    #[test]
    fn pause_transitions_to_paused() {
        // Arrange
        let now = Instant::now();
        let state = TimerState::Running { started_at: now, accumulated_seconds: 2 };

        // Act
        let result = state.transition_at(TimerEvent::Pause, now);

        // Assert
        assert_eq!(result, TimerState::Paused { accumulated_seconds: 2 })
    }

    #[test]
    fn resume_transitions_to_running() {
        // Arrange
        let now = Instant::now();
        let state = TimerState::Paused { accumulated_seconds: 2 };

        // Act
        let result = state.transition_at(TimerEvent::Resume, now);

        // Assert
        assert_eq!(result, TimerState::Running { started_at: now, accumulated_seconds: 2 } )
    }

    #[rstest]
    #[case::running_play(TimerState::Running {
        started_at: Instant::now(),
        accumulated_seconds: 10
    }, TimerEvent::Play)]
    #[case::paused_play(TimerState::Paused {
        accumulated_seconds: 10
    }, TimerEvent::Play)]

    #[case::idle_pause(TimerState::Idle, TimerEvent::Pause)]
    #[case::paused_pause(TimerState::Paused {
        accumulated_seconds: 10
    }, TimerEvent::Pause)]

    #[case::idle_resume(TimerState::Idle, TimerEvent::Resume)]
    #[case::running_resume(TimerState::Running {
        started_at: Instant::now(),
        accumulated_seconds: 10
    }, TimerEvent::Resume)]
    fn event_does_not_change_state(#[case] state: TimerState, #[case] event: TimerEvent) {
        // Arrange & Act
        let result = state.clone().transition(event);

        // Assert
        assert_eq!(result, state)
    }

    #[test]
    fn pause_adds_elapsed_time_to_accumulated_time() {
        // Arrange
        use std::time::Duration;

        let started_at = Instant::now();
        let now = started_at + Duration::from_secs(5);

        let state = TimerState::Running {
            started_at,
            accumulated_seconds: 2,
        };

        // Act
        let result = state.transition_at(TimerEvent::Pause, now);

        // Assert
        assert_eq!(
            result,
            TimerState::Paused {
                accumulated_seconds: 7,
            }
        );
    }
}