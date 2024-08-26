use std::rc::Rc;

use web_sys::CanvasRenderingContext2d;

use crate::mouse::MouseInfo;



pub(crate) struct State {
    pub context: Rc<CanvasRenderingContext2d>,
    pub mouse_info: Rc<MouseInfo>
}
impl State {
    pub fn new(context: Rc<CanvasRenderingContext2d>) -> State {
        State {
            context,
            mouse_info: Rc::new(MouseInfo::new())
        }
    }

    pub fn draw_line(&self) {
        let x = *self.mouse_info.x_coord.borrow();
        let y = *self.mouse_info.y_coord.borrow();
        let mut last_x = self.mouse_info.last_x.borrow_mut();
        let mut last_y = self.mouse_info.last_y.borrow_mut();
        
        if let (Some(lx), Some(ly)) = (*last_x, *last_y) {
            self.context.begin_path();
            self.context.move_to(lx, ly);
            self.context.line_to(x, y);
            self.context.stroke();
        }
        
        *last_x = Some(x);
        *last_y = Some(y);
    }
}