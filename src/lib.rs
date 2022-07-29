use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct ItemConfig {
    color: String,
    label: String,
}

#[wasm_bindgen]
pub struct LuckyWheel {
    radius: f64,
    context: CanvasRenderingContext2d,
    angle: f64,
    items: Vec<ItemConfig>,
}

#[wasm_bindgen]
impl LuckyWheel {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> LuckyWheel {
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

        let document = window().unwrap().document().unwrap();
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

        LuckyWheel {
            context,
            radius,
            angle,
            items,
        }
    }

    pub fn draw(&self, degree: u16) {
        let mut i = 0.0;

        for item in &self.items {
            let start_ang = self.round_radian(self.angle * i + self.degree_to_radian(degree));
            let end_ang = self.round_radian(start_ang + self.angle);
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

    fn degree_to_radian(&self, degree: u16) -> f64 {
        (2.0 * f64::consts::PI / 360.0) * degree as f64
    }

    fn round_radian(&self, radian: f64) -> f64 {
        radian % (2.0 * f64::consts::PI)
    }
}
