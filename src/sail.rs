use crate::models::{Cosmos, Rune, Vessel};

/// Defines the reason for the Velo program's execution halt.
pub enum Termination {
    Stopped,                      // Vessel velocity/pointer reached zero.
    NoSignal(usize, usize),       // Vessel traveled out of the Cosmos bounds.
    NoInitialVelocityOrDirection, // Start Rune was not a Thrust rune.
}

pub struct Config {
    debug: bool,
    trace: bool,
    ignore_void: bool,
}

impl Config {
    pub fn new(debug: bool, trace: bool, ignore_void: bool) -> Self {
        Self {
            debug,
            trace,
            ignore_void,
        }
    }
}

/// Runs the Velo program by moving the Vessel through the Cosmos grid.
pub fn sail(cosmos: Cosmos, mut vessel: Vessel, config: Config) -> Termination {
    let width = cosmos.width();
    let height = cosmos.height();

    // Check for initial velocity requirement (must start on a Thrust rune)
    if vessel.velocity() == 0 {
        return Termination::NoInitialVelocityOrDirection;
    }

    // The execution loop: continues as long as the Velocity/Pointer is positive.
    while vessel.velocity() > 0 {
        match vessel.get_next_coordinate() {
            Ok((x, y)) => {
                // Check if the next coordinates are within the Cosmos boundaries.
                if x >= width || y >= height {
                    return Termination::NoSignal(x.min(width - 1), y.min(height - 1));
                }

                let rune = cosmos.get(x, y);

                // Update the vessel's position.
                vessel.move_to(x, y);

                // Impact the Rune and execute the associated instruction/movement.
                vessel.impact_rune(rune);

                if rune == Rune::Debug && config.debug {
                    println!("[Debug] Vessel: {:?}. Rune: {:?}", vessel, rune);
                }

                if config.trace && !(config.ignore_void && rune == Rune::Void) {
                    println!("Vessel: {:?}. Rune: {:?}", vessel, rune);
                }
            }
            Err(_) => return Termination::NoSignal(vessel.x(), vessel.y()),
        }
    }

    Termination::Stopped
}
