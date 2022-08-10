#[cfg(test)]
mod tests;

const MIN_VELOCITY_L: f64 = 0.1;
const MIN_VELOCITY_H: f64 = 1.0;
const MAX_VELOCITY_L: f64 = 1.0;
const MAX_VELOCITY_H: f64 = 30.0;
const MIN_CIRCLE: u32 = 360;

struct Config {
    min_velocity: f64,
    max_velocity: f64,
    circles: u32,
}

pub struct Spinning {
    degree: f64,
    velocity: f64,
    stoped_degree: f64,
    status: Status,
    duration: f64,
    config: Config,
}

#[derive(PartialEq)]
enum Status {
    STARTING,
    STARTED,
    STOPPED,
}

fn validate_velocity(min_velocity: f64, max_velocity: f64) {
    if min_velocity < MIN_VELOCITY_L || min_velocity > MIN_VELOCITY_H {
        panic!(
            "min_velocity must be between {} and {}",
            MIN_VELOCITY_L, MIN_VELOCITY_H
        );
    }

    if max_velocity < MAX_VELOCITY_L || max_velocity > MAX_VELOCITY_H {
        panic!(
            "max_velocity must be between {} and {}",
            MAX_VELOCITY_L, MAX_VELOCITY_H
        );
    }
}

fn validate_circles(circles: u32) {
    if circles < MIN_CIRCLE {
        panic!("circles must be equal or greater than {}", MIN_CIRCLE);
    }
}

impl Spinning {
    pub fn new(min_velocity: f64, max_velocity: f64, circles: u32) -> Spinning {
        validate_velocity(min_velocity, max_velocity);
        validate_circles(circles);

        Spinning {
            degree: 0.0,
            velocity: max_velocity,
            stoped_degree: 0.0,
            status: Status::STOPPED,
            duration: circles as f64,
            config: Config {
                min_velocity,
                max_velocity,
                circles,
            },
        }
    }

    pub fn start(&mut self, stoped_degree: u32) {
        let remain_degree = self.config.circles % 360;

        self.stoped_degree = (stoped_degree + 360 - remain_degree) as f64 % 360.0;
        self.velocity = self.config.max_velocity;
        self.status = if self.stoped_degree == self.degree {
            Status::STARTED
        } else {
            Status::STARTING
        };
        self.duration = self.config.circles as f64;
    }

    pub fn tick(&mut self) {
        match self.status {
            Status::STARTING => {
                if (self.degree - self.stoped_degree).abs() <= self.config.max_velocity {
                    self.degree = self.stoped_degree;
                    self.status = Status::STARTED;
                } else {
                    self.add_degree(self.velocity);
                }
            }
            Status::STARTED => {
                self.add_degree(self.velocity);

                self.duration = self.duration - self.velocity;
                self.velocity =
                    self.duration * self.config.max_velocity / self.config.circles as f64;
                self.adjust_min_velocity();

                if self.duration <= 0.0 {
                    self.status = Status::STOPPED;
                }
            }
            Status::STOPPED => (),
        }
    }

    pub fn is_stop(&self) -> bool {
        self.status == Status::STOPPED
    }

    pub fn degree(&self) -> f64 {
        self.degree
    }

    fn add_degree(&mut self, degree: f64) {
        self.degree = (self.degree + degree) % 360.0;
    }

    fn adjust_min_velocity(&mut self) {
        if self.velocity < self.config.min_velocity {
            self.velocity = self.config.min_velocity
        }
    }
}
