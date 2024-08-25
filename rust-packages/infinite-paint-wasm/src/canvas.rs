use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::cell::Cell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct Canvas {
    canvas: HtmlCanvasElement,
    context: Rc<CanvasRenderingContext2d>,
    pressed: Rc<Cell<bool>>,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Canvas, JsValue> {
        let document = web_sys::window()
            .unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        // allows the canvas element to be edited by JS and CSS
        canvas.set_id("wasm_canvas"); 
        // TODO: What does this do??
        document.body().unwrap().append_child(&canvas)?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let context = Rc::new(context);
        let pressed = Rc::new(Cell::new(false));

        Ok(Canvas {
            canvas,
            context,
            pressed
        })
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }
}