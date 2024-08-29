use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::appstate::State;

// Event Handlers for mouse actions

fn create_mousedown_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.is_mouse_pressed();
        state.set_mouse_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mousemove_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.set_mouse_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mouseup_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_event: MouseEvent| {
        state.is_mouse_released();
    }) as Box<dyn FnMut(MouseEvent)>)
}

pub fn add_mouse_event_listeners(canvas: &HtmlCanvasElement, state: Rc<State>) -> Result<(), JsValue> {
    let mousedown_handler = create_mousedown_handler(state.clone());
    let mousemove_handler = create_mousemove_handler(state.clone());
    let mouseup_handler = create_mouseup_handler(state.clone());

    canvas.add_event_listener_with_callback("mousedown", mousedown_handler.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mousemove", mousemove_handler.as_ref().unchecked_ref())?;
    canvas.add_event_listener_with_callback("mouseup", mouseup_handler.as_ref().unchecked_ref())?;

    mousedown_handler.forget();
    mousemove_handler.forget();
    mouseup_handler.forget();

    Ok(())
}