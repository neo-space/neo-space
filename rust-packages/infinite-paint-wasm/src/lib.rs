mod utils;
mod canvas;

use mouse::add_mouse_event_listeners;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d};
use std::cell::RefCell; 
use std::rc::Rc;

mod appstate;
use appstate::State;
mod mouse;


#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> { // indicates succeed w JsValue fail with no meaninful return value
    // initialize the canvas element
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let _ = document.body().unwrap().append_child(&canvas);
    canvas.set_width(500);
    canvas.set_height(500);
    canvas.style().set_property("border", "solid")?; // this is like a CSS portion
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let state = Rc::new(State::new(context.into()));

    add_mouse_event_listeners(&canvas, state.clone())?;

    // Render Loop
    {
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let state = state.clone();
            if *state.mouse_info.left_click.borrow() {
                // TODO: there will be deeper functionality here based on user action mode
                state.draw_line();
            }
    
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window().unwrap().request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

