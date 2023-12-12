use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use error::WebXrError;
use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, XrSession};

pub mod error;
mod graphics;
mod init;
mod session;

#[derive(Clone)]
struct WebXrSettings {
    pub vr_supported: bool,
    pub ar_supported: bool,
    pub vr_button: String,
    pub ar_button: String,
    pub canvas: String,
}

impl Default for WebXrSettings {
    fn default() -> WebXrSettings {
        WebXrSettings {
            vr_supported: true,
            ar_supported: true,
            vr_button: "vr_button".to_string(),
            ar_button: "ar_button".to_string(),
            canvas: "canvas[data-raw-handle]".to_string(),
        }
    }
}

pub(crate) struct WebXrContext {
    pub session: Rc<RefCell<Option<Result<XrSession, WebXrError>>>>,
    pub canvas: String,
}

impl Default for WebXrContext {
    fn default() -> WebXrContext {
        WebXrContext {
            session: Rc::new(RefCell::new(None)),
            canvas: "canvas[data-raw-handle]".to_string(),
        }
    }
}

struct AppPointer(*mut App);

#[derive(Default)]
pub struct WebXrPlugin {
    settings: WebXrSettings,
}

impl Plugin for WebXrPlugin {
    fn build(&self, app: &mut App) {
        info!(
            "{:?}",
            make_canvas_xr_compatible("canvas[data-raw-handle]".to_string())
        );

        app.set_runner(webxr_runner);
        let context = WebXrContext {
            canvas: self.settings.canvas.clone(),
            ..default()
        };

        AsyncComputeTaskPool::get().spawn(init::initialize_webxr(
            self.settings.clone(),
            context.session.clone(),
        ));
        app.insert_non_send_resource(context);

        app.add_systems(PreUpdate, session::poll_session);
    }
}

fn make_canvas_xr_compatible(canvas: String) -> Result<(), WebXrError> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let context = document
        .query_selector(&canvas)
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::CanvasNotFound)?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| WebXrError::CanvasNotFound)?
        .get_context("webgl2")
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::WebGl2ContextNotFound)?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| WebXrError::WebGl2ContextNotFound)?;

    context.make_xr_compatible();
    Ok(())
}

fn webxr_runner(app: App) {
    let unsafe_app = UnsafeCell::new(app);
    let pointer = unsafe_app.get();

    unsafe {
        let mut app = unsafe_app.get().read();
        app.insert_non_send_resource(AppPointer(pointer));

        info!("starting winit_runner");
        bevy::winit::winit_runner(app);
    }
}
