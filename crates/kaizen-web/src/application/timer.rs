use crate::domain::timer_state_machine::{TimerEvent, TimerState};
use web_time::Instant;

pub struct Timer {
    state: TimerState
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            state: TimerState::Idle
        }
    }

    pub fn start(&mut self) {
        let transition = match self.state {
            TimerState::Paused { .. } => TimerEvent::Resume,
            _ => TimerEvent::Play
        };

        self.state = self
            .state
            .clone()
            .transition(transition);
    }

    pub fn pause(&mut self) {
        self.state = self
            .state
            .clone()
            .transition(TimerEvent::Pause);
    }

    pub fn stop(&mut self) {
        self.state = self
            .state
            .clone()
            .transition(TimerEvent::Stop);
    }

    pub fn elapsed_seconds(&self) -> u64 {
        self.elapsed_seconds_at(Instant::now())
    }

    fn elapsed_seconds_at(&self, now: Instant) -> u64 {
        match &self.state {
            TimerState::Idle => 0,

            TimerState::Running {
                started_at,
                accumulated_seconds,
            } => {
                accumulated_seconds + now.duration_since(*started_at).as_secs()
            }

            TimerState::Paused {
                accumulated_seconds,
            } => *accumulated_seconds,
        }
    }

    pub fn state(&self) -> TimerState {
        self.state.clone()
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, TimerState::Idle)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use rstest::rstest;
    use super::*;

    #[test]
    fn new_creates_idle_timer() {
        // Arrange & Act
        let timer = Timer::new();

        // Assert
        assert_eq!(timer.state, TimerState::Idle)
    }

    #[test]
    fn start_transitions_to_running() {
        // Arrange
        let mut timer = Timer::new();

        // Act
        timer.start();

        // Assert
        assert!(matches!(
            timer.state,
            TimerState::Running {
                accumulated_seconds: 0,
                ..
            }
        ));
    }

    #[test]
    fn start_resumes_paused_timer() {
        // Arrange
        let mut timer = Timer {
            state: TimerState::Paused {
                accumulated_seconds: 42,
            },
        };

        // Act
        timer.start();

        // Assert
        assert!(matches!(
            timer.state,
            TimerState::Running {
                accumulated_seconds: 42,
                ..
            }
        ));
    }

    #[test]
    fn pause_pauses_running_timer() {
        // Arrange
        

        let mut timer = Timer {
            state: TimerState::Running {
                started_at: Instant::now(),
                accumulated_seconds: 10,
            },
        };

        // Act
        timer.pause();

        // Assert
        assert!(matches!(
            timer.state,
            TimerState::Paused { .. }
        ));
    }

    #[test]
    fn stop_stops_timer() {
        // Arrange
        let mut timer = Timer {
            state: TimerState::Paused {
                accumulated_seconds: 10,
            },
        };

        // Act
        timer.stop();

        // Assert
        assert_eq!(timer.state, TimerState::Idle);
    }

    #[test]
    fn elapsed_seconds_returns_zero_when_idle() {
        // Arrange
        let timer = Timer {
            state: TimerState::Idle,
        };

        // Act
        let result = timer.elapsed_seconds();

        // Assert
        assert_eq!(result, 0);
    }

    #[rstest]
    #[case::zero(0)]
    #[case::some_accumulated_time(42)]
    #[case::one_hour(3_600)]
    fn elapsed_seconds_returns_accumulated_time_when_paused(
        #[case] accumulated_seconds: u64,
    ) {
        // Arrange
        let timer = Timer {
            state: TimerState::Paused {
                accumulated_seconds,
            },
        };

        // Act
        let result = timer.elapsed_seconds();

        // Assert
        assert_eq!(result, accumulated_seconds);
    }

    #[rstest]
    #[case::only_current_segment(0, 5, 5)]
    #[case::with_accumulated_time(10, 5, 15)]
    #[case::no_elapsed_current_segment(10, 0, 10)]
    fn elapsed_seconds_combines_accumulated_and_current_segment(
        #[case] accumulated_seconds: u64,
        #[case] current_segment_seconds: u64,
        #[case] expected_seconds: u64,
    ) {
        // Arrange
        let started_at = Instant::now();
        let now = started_at + Duration::from_secs(current_segment_seconds);

        let timer = Timer {
            state: TimerState::Running {
                started_at,
                accumulated_seconds,
            },
        };

        // Act
        let result = timer.elapsed_seconds_at(now);

        // Assert
        assert_eq!(result, expected_seconds);
    }
}