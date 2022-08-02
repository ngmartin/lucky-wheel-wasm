use crate::browser;
use crate::item::Items;
use crate::utils;

use core::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Renderer {
    radius: f64,
    context: CanvasRenderingContext2d,
    config_items: Rc<RefCell<Items>>,
}

impl Renderer {
    pub fn new(canvas_id: &str, items: Rc<RefCell<Items>>) -> Renderer {
        let document = browser::document();
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("canvas id not found");
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .expect("get context 2d failed")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let diameter = context.canvas().unwrap().width() as f64;
        let radius = diameter / 2.0;

        Renderer {
            context,
            radius,
            config_items: items,
        }
    }

    pub fn draw(&self, degree: f64) {
        for item in &self.config_items.borrow().items {
            let start_ang = utils::round_radian(
                utils::degree_to_radian(item.start_degree) + utils::degree_to_radian(degree),
            );
            let end_ang = utils::round_radian(
                utils::degree_to_radian(item.end_degree) + utils::degree_to_radian(degree),
            );

            self.context.save();
            // color
            self.context.begin_path();
            self.context.set_fill_style(&JsValue::from(&item.color));
            self.context.move_to(self.radius, self.radius);
            self.context
                .arc(self.radius, self.radius, self.radius, start_ang, end_ang)
                .unwrap();
            self.context.line_to(self.radius, self.radius);
            self.context.fill();
            // text
            self.context.translate(self.radius, self.radius).unwrap();
            self.context
                .rotate(
                    start_ang + utils::degree_to_radian(item.end_degree - item.start_degree) / 2.0,
                )
                .unwrap();
            self.context.set_text_align("right");
            self.context.set_fill_style(&"#fff".into());
            self.context.set_font("bold 30px sans-serif");
            self.context
                .fill_text(&item.label, self.radius - 10.0, 10.0)
                .unwrap();

            self.context.restore();
        }
    }
}
