use std::{cell::Cell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::appstate::State;

pub(crate) struct MouseInfo {
    pub left_click: Cell<bool>,
    pub x_coord: Cell<f64>,
    pub y_coord: Cell<f64>,
    pub last_x: Cell<Option<f64>>,
    pub last_y: Cell<Option<f64>>
}

impl MouseInfo {
    pub fn new() -> MouseInfo {
        // Cell used here because simpler, more efficient. Refcell uses Runtime borrow checking
        // Cell also has get and set methods
        // Cell ideal for Copy on primitive datatypes
        MouseInfo {
            left_click: Cell::new(false),
            x_coord: Cell::new(0.0),
            y_coord: Cell::new(0.0),
            last_x: Cell::new(None),
            last_y: Cell::new(None),
        }
    }

    pub fn press(&self) {
        self.left_click.set(true);
        self.last_x.set(None);
        self.last_y.set(None);
    }

    pub fn released(&self) {
        self.left_click.set(false);
        self.last_x.set(None);
        self.last_y.set(None);
    }

    pub fn is_pressed(&self) -> bool {
        self.left_click.get()
    }

    pub fn update_position(&self, x: f64, y: f64) {
        self.x_coord.set(x);
        self.y_coord.set(y);
    }

    pub fn get_coords(&self) -> (f64, f64) {
        (self.x_coord.get(), self.y_coord.get())
    }

    pub fn get_last_coords(&self) -> (Option<f64>, Option<f64>) {
        (self.last_x.get(), self.last_y.get())
    }

    pub fn update_last_coords(&self, x: f64, y: f64) {
        self.last_x.set(Some(x));
        self.last_y.set(Some(y));
    }
}

// Event Handlers for mouse actions

fn create_mousedown_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.mouse_info().press();
        state.mouse_info().update_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mousemove_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |event: MouseEvent| {
        state.mouse_info().update_position(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_mouseup_handler(state: Rc<State>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_event: MouseEvent| {
        state.mouse_info().released();
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