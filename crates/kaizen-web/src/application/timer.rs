use crate::domain::timer_state_machine::{TimerEvent, TimerState};

struct Timer {
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
}

#[cfg(test)]
mod tests {
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
        use web_time::Instant;

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
}