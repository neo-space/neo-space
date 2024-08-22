mod utils;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::cell::{self, Cell}; // concurrency primative that allows for interior mutability
use std::rc::Rc; // a way to have multiple owners of the same data

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> { // indicates succeed w JsValue fail with no meaninful return value
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let _ = document.body().unwrap().append_child(&canvas);
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?; // this is like a CSS portion
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    // multiple owners of context
    let context = Rc::new(context);
    // multiple owners of key pressed and mark it for single 
    // threaded use with Cell (RefCell is multithreadded)
    let pressed = Rc::new(Cell::new(false)); // lets you clone without moving

    /* 
    This is a block expression to group related statements and declarations together.
    It creates a closure 🤢🥲 that creates a pointer to a function that performs some action
        when a direction key is pressed.
    An event listener with callback is added to the "mousedown" action
    */
    {
        // in a closure its better to copy variables getting picked up from outside
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Here is the part that is essentially drawing mode.
    // You can basically make a pointer to a drawing function and 
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed .set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

