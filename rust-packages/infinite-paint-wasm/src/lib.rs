mod utils;
mod canvas;

use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d};
use std::cell::RefCell; use std::num::NonZero;
// concurrency primative that allows for interior mutability
use std::rc::Rc; // a way to have multiple owners of the same data

struct MouseInfo {
    left_click: Rc<RefCell<bool>>,
    x_coord: Rc<RefCell<f64>>,
    y_coord: Rc<RefCell<f64>>,
    last_x: Rc<RefCell<Option<f64>>>,
    last_y: Rc<RefCell<Option<f64>>>
}

impl MouseInfo {
    fn new() -> MouseInfo {
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

struct State {
    context: Rc<CanvasRenderingContext2d>,
    mouse_info: Rc<MouseInfo>
}
impl State {
    fn new(context: Rc<CanvasRenderingContext2d>) -> State {
        State {
            context,
            mouse_info: Rc::new(MouseInfo::new())
        }
    }
}


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

    /* 
    This is a block expression to group related statements and declarations together.
    It creates a closure 🤢🥲 that creates a pointer to a function that performs some action
        when a direction key is pressed.
    An event listener with callback is added to the "mousedown" action
    */
    // Mouse Down Event
    {
        // in a closure its better to copy variables getting picked up from outside
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            state.mouse_info.pressed(); // click
            state.mouse_info.update_position(event.offset_x() as f64, event.offset_y() as f64);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse Move Event
    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            state.mouse_info.update_position(event.offset_x() as f64, event.offset_y() as f64);
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse Up Event
    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            state.mouse_info.released();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Render Loop
    {
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let state = state.clone();
            if *state.mouse_info.left_click.borrow() {
                let context = &state.context;
                let x = *state.mouse_info.x_coord.borrow();
                let y = *state.mouse_info.y_coord.borrow();
                let mut last_x = state.mouse_info.last_x.borrow_mut();
                let mut last_y = state.mouse_info.last_y.borrow_mut();
                
                if let (Some(lx), Some(ly)) = (*last_x, *last_y) {
                    context.begin_path();
                    context.move_to(lx, ly);
                    context.line_to(x, y);
                    context.stroke();
                }
                
                *last_x = Some(x);
                *last_y = Some(y);
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

