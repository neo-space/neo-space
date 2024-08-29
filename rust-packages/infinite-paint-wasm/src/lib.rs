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

// TODO: Remove this comment
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

    // adding the mouse event handlers (clousures)
    add_mouse_event_listeners(&canvas, state.clone())?;

    // render loop closure
    // {
    //     let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    //     let g = f.clone();
    //     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //         let state = state.clone();
    //         if state.mouse_info().is_pressed() {
    //             // TODO: there will be deeper functionality here based on user action mode
    //             state.draw_line();
    //         }
    
    //         request_animation_frame(f.borrow().as_ref().unwrap());
    //     }) as Box<dyn FnMut()>));
    
    //     request_animation_frame(g.borrow().as_ref().unwrap());
    // }
    start_animation_loop(state.clone());

    Ok(())
}

pub fn start_animation_loop(state: Rc<State>) -> Result<(), JsValue> {
    let window = window().unwrap();
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // DON'T import borrow, it will cause errors, we want RefCell's borrow here
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if state.mouse_info().is_pressed() {
            state.draw_line();
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

