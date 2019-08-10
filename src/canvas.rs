use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use chal_engine::state::{State, StateEvent};

use crate::render::Renderer;

// pub fn create_webgl_context(state: Rc<RefCell<State>>) -> Result<WebGlRenderingContext, JsValue> {
pub fn create_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // attach_mouse_move_handler(&canvas, Rc::clone(&state));
    // attach_mouse_down_handler(&canvas, Rc::clone(&state));
    // attach_mouse_up_handler(&canvas, Rc::clone(&state));
    // attach_zoom_handler(&canvas, Rc::clone(&state));

    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(WebGlRenderingContext::DEPTH_TEST);

    Ok(gl)
}

fn attach_mouse_move_handler(canvas: &web_sys::HtmlCanvasElement, state: Rc<RefCell<State>>) {
    let handler = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        state.borrow_mut().msg(&StateEvent::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref()).unwrap();
    handler.forget();
}

fn attach_mouse_down_handler(canvas: &web_sys::HtmlCanvasElement, state: Rc<RefCell<State>>) {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        state.borrow_mut().msg(&StateEvent::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref()).unwrap();
    handler.forget();
}

fn attach_mouse_up_handler(canvas: &web_sys::HtmlCanvasElement, state: Rc<RefCell<State>>) {
    let handler = move |_: web_sys::MouseEvent| {
        state.borrow_mut().msg(&StateEvent::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref()).unwrap();
    handler.forget();
}

fn attach_zoom_handler(canvas: &web_sys::HtmlCanvasElement, state: Rc<RefCell<State>>) {
    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();

        let zoom_amount = event.delta_y() / 50.0;

        state.borrow_mut().msg(&StateEvent::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref()).unwrap();
    handler.forget();
}
