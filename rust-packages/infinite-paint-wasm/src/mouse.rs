use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::appstate::State;



pub(crate) struct MouseInfo {
    pub left_click: Rc<RefCell<bool>>,
    pub x_coord: Rc<RefCell<f64>>,
    pub y_coord: Rc<RefCell<f64>>,
    pub last_x: Rc<RefCell<Option<f64>>>,
    pub last_y: Rc<RefCell<Option<f64>>>
}

impl MouseInfo {
    pub fn new() -> MouseInfo {
        MouseInfo {
            left_click: Rc::new(RefCell::new(false)),
            x_coord: Rc::new(RefCell::new(0.0)),
            y_coord: Rc::new(RefCell::new(0.0)),
            last_x: Rc::new(RefCell::new(None)),
            last_y: Rc::new(RefCell::new(None)),
        }
    }

    pub fn pressed(&self) {
        *self.left_click.borrow_mut() = true;
        *self.last_x.borrow_mut() = None;
        *self.last_y.borrow_mut() = None;
    }

    pub fn released(&self) {
        *self.left_click.borrow_mut() = false;
        *self.last_x.borrow_mut() = None;
        *self.last_y.borrow_mut() = None;
    }

    pub fn update_position(&self, x: f64, y:f64) {
        *self.x_coord.borrow_mut() = x;
        *self.y_coord.borrow_mut() = y;
    }
}

// Event Handlers for mouse actions

fn create_mousedown_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.mouse_info.pressed();
        state.mouse_info.update_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mousemove_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.mouse_info.update_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mouseup_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_event: MouseEvent| {
        state.mouse_info.released();
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