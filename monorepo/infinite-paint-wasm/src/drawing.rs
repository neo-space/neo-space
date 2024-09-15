use web_sys::CanvasRenderingContext2d;

use crate::mouse::MouseInfo;

pub fn draw_line(mouse_info: &MouseInfo, context: &CanvasRenderingContext2d) {
    let x = mouse_info.get_x_coord();
    let y = mouse_info.get_y_coord();
    let mut last_x = mouse_info.last_x.borrow_mut();
    let mut last_y = mouse_info.last_y.borrow_mut();
    
    if let (Some(lx), Some(ly)) = (*last_x, *last_y) {
        context.begin_path();
        context.move_to(lx, ly);
        context.line_to(x, y);
        context.stroke();
    }
    
    *last_x = Some(x);
    *last_y = Some(y);
}