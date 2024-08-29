use std::cell::Cell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

use crate::mouse::MouseInfo;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum UserActionMode {
    Drag,
    Draw
}

pub struct State {
    context: Rc<CanvasRenderingContext2d>,
    mouse_info: Rc<MouseInfo>,
    user_action_mode: Cell<UserActionMode>
}

impl State {
    pub fn new(context: Rc<CanvasRenderingContext2d>) -> State {
        State {
            context,
            mouse_info: Rc::new(MouseInfo::new()),
            user_action_mode: Cell::new(UserActionMode::Drag)
        }
    }

    pub fn mouse_info(&self) -> Rc<MouseInfo> {
        self.mouse_info.clone()
    }

    pub fn set_user_action_mode(&self, mode: UserActionMode) {
        self.user_action_mode.set(mode);
    }

    pub fn get_user_action_mode(&self) -> UserActionMode {
        self.user_action_mode.get()
    }

    pub fn draw_line(&self) {
        let (x, y) = self.mouse_info.get_coords();
        let (last_x, last_y) = self.mouse_info.get_last_coords();
        
        if let (Some(lx), Some(ly)) = (last_x, last_y) {
            self.context.begin_path();
            self.context.move_to(lx, ly);
            self.context.line_to(x, y);
            self.context.stroke();
        }
        
        self.mouse_info.update_last_coords(x, y);
    }
}

#[wasm_bindgen]
pub struct WasmStateWrapper(Rc<State>);

#[wasm_bindgen]
impl WasmStateWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(context: CanvasRenderingContext2d) -> WasmStateWrapper {
        WasmStateWrapper(Rc::new(State::new(Rc::new(context))))
    }

    pub fn set_user_action_mode(&self, mode: UserActionMode) {
        self.0.set_user_action_mode(mode);
    }

    pub fn get_user_action_mode(&self) -> UserActionMode {
        self.0.get_user_action_mode()
    }

    pub fn draw_line(&self) {
        self.0.draw_line();
    }
}