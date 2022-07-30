use crate::browser;
use crate::utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

struct ItemConfig {
    color: String,
    label: String,
}

pub struct Renderer {
    radius: f64,
    context: CanvasRenderingContext2d,
    angle: f64,
    items: Vec<ItemConfig>,
}

impl Renderer {
    pub fn new(canvas_id: &str) -> Renderer {
        let items = vec![
            ItemConfig {
                color: String::from("#f82"),
                label: String::from("test1"),
            },
            ItemConfig {
                color: String::from("#0bf"),
                label: String::from("test2"),
            },
            ItemConfig {
                color: String::from("#fb0"),
                label: String::from("test3"),
            },
        ];

        let document = browser::document();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let diameter = context.canvas().unwrap().width() as f64;
        let radius = diameter / 2.0;
        let angle = 2.0 * f64::consts::PI / items.len() as f64;

        Renderer {
            context,
            radius,
            angle,
            items,
        }
    }

    pub fn draw(&self, degree: f64) {
        let mut i = 0.0;

        for item in &self.items {
            let start_ang = utils::round_radian(self.angle * i + utils::degree_to_radian(degree));
            let end_ang = utils::round_radian(start_ang + self.angle);
            i += 1.0;

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
            self.context.rotate(start_ang + self.angle / 2.0).unwrap();
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
