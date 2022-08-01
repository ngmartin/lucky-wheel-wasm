use crate::browser;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const MAX_VELOCITY: f64 = 15.0;
const MIN_VELOCITY: f64 = 0.2;
const DURATION: f64 = 6.0 * 360.0;
const DEGREE_DELTA: f64 = 5.0;

pub struct RenderLoop {
    renderer: Rc<RefCell<Renderer>>,
    degree: f64,
    animation_id: Option<i32>,
    pub closure: Option<Closure<dyn FnMut()>>,
    velocity: f64,
    stoped_degree: f64,
    is_starting: bool,
    duration: f64,
}

impl RenderLoop {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> RenderLoop {
        RenderLoop {
            renderer,
            closure: None,
            animation_id: None,
            degree: 0.0,
            velocity: MAX_VELOCITY,
            stoped_degree: 0.0,
            is_starting: false,
            duration: DURATION,
        }
    }

    pub fn render_loop(&mut self) {
        if self.is_starting {
            if self.stoped_degree >= self.degree - DEGREE_DELTA
                && self.stoped_degree <= self.degree + DEGREE_DELTA
            {
                self.degree = self.stoped_degree;
                self.is_starting = false;
            }
        } else {
            self.duration = self.duration - self.velocity;
            self.velocity = self.duration * MAX_VELOCITY / DURATION;

            if self.velocity < MIN_VELOCITY {
                self.velocity = MIN_VELOCITY
            }

            if self.duration <= 0.0 {
                self.velocity = 0.0;
                self.stop();
            }
        }

        self.renderer.borrow().draw(self.degree);
        self.add_degree(self.velocity);

        self.animation_id = if let Some(ref closure) = self.closure {
            Some(browser::request_animation_frame(closure))
        } else {
            None
        };
    }

    pub fn start(&mut self, stoped_degree: f64) {
        self.stoped_degree = stoped_degree;
        self.velocity = MAX_VELOCITY;
        self.is_starting = true;
        self.duration = DURATION;

        self.render_loop();
    }

    fn stop(&mut self) {
        if let Some(id) = self.animation_id {
            browser::cancel_animation_frame(id);
            self.animation_id = None;
        }
    }

    fn add_degree(&mut self, degree: f64) {
        self.degree = (self.degree + degree) % 360.0;
    }
}
