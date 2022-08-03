const MAX_VELOCITY: f64 = 15.0;
const MIN_VELOCITY: f64 = 0.2;
const DURATION: f64 = 6.0 * 360.0;
const DEGREE_DELTA: f64 = 5.0;

pub struct Spinning {
    pub degree: f64,
    velocity: f64,
    stoped_degree: f64,
    status: Status,
    duration: f64,
}

#[derive(PartialEq)]
enum Status {
    STARTING,
    STARTED,
    STOPPED,
}

impl Spinning {
    pub fn new() -> Spinning {
        Spinning {
            degree: 0.0,
            velocity: MAX_VELOCITY,
            stoped_degree: 0.0,
            status: Status::STOPPED,
            duration: DURATION,
        }
    }

    pub fn start(&mut self, stoped_degree: f64) {
        self.stoped_degree = stoped_degree;
        self.velocity = MAX_VELOCITY;
        self.status = Status::STARTING;
        self.duration = DURATION;
    }

    pub fn tick(&mut self) {
        match self.status {
            Status::STARTING => {
                self.add_degree(self.velocity);

                if self.stoped_degree >= self.degree - DEGREE_DELTA
                    && self.stoped_degree <= self.degree + DEGREE_DELTA
                {
                    self.degree = self.stoped_degree;
                    self.status = Status::STARTED;
                }
            }
            Status::STARTED => {
                self.add_degree(self.velocity);

                self.duration = self.duration - self.velocity;
                self.velocity = self.duration * MAX_VELOCITY / DURATION;

                if self.velocity < MIN_VELOCITY {
                    self.velocity = MIN_VELOCITY
                }

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

    fn add_degree(&mut self, degree: f64) {
        self.degree = (self.degree + degree) % 360.0;
    }
}
