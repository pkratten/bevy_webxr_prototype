use std::{cell::RefCell, rc::Rc};

use bevy::{prelude::*, tasks::ComputeTaskPool};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::XrSession;

use crate::{error::WebXrError, AppPointer, WebXrContext};

pub(crate) fn poll_session(
    context: NonSendMut<WebXrContext>,
    mut session_initialized: Local<bool>,
    app_pointer: NonSend<AppPointer>,
) {
    if !*session_initialized {
        if let Some(Ok(_)) = *context.session.clone().borrow() {
            let session = context.session.clone();
            let canvas = context.canvas.clone();
            let pointer = app_pointer.0.clone();
            ComputeTaskPool::get().spawn(init_session(session, canvas, pointer));
            *session_initialized = true;
        } else {
            *session_initialized = false;
        }
    }
}

async fn init_session(
    session: Rc<RefCell<Option<Result<XrSession, WebXrError>>>>,
    canvas: String,
    pointer: *mut App,
) -> Result<(), WebXrError> {
    if let Some(Ok(session)) = &*session.borrow() {
        init_context(session, canvas).await?;
        request_web_xr_frame(session, pointer)?;
        Ok(())
    } else {
        Err(WebXrError::SessionError)
    }
}

struct SessionInitTask;

pub(crate) fn poll_session_init() {}

async fn init_context(session: &XrSession, canvas: String) -> Result<(), WebXrError> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .query_selector(&canvas)
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::CanvasNotFound)?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| WebXrError::CanvasNotFound)?;

    let context = canvas
        .get_context("webgl2")
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::WebGl2ContextNotFound)?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| WebXrError::WebGl2ContextNotFound)?;

    wasm_bindgen_futures::JsFuture::from(context.make_xr_compatible())
        .await
        .map_err(|err| WebXrError::JsError(err))?;

    let layer_init = web_sys::XrWebGlLayerInit::new();

    let web_gl_layer = web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context_and_layer_init(
        &session,
        &context,
        &layer_init,
    )
    .map_err(|err| WebXrError::JsError(err))?;

    info!("{:?}", web_gl_layer);

    let mut render_state_init = web_sys::XrRenderStateInit::new();

    info!("{:?}", render_state_init);

    render_state_init.base_layer(Some(&web_gl_layer));

    info!("{:?}", render_state_init);

    session.update_render_state_with_state(&render_state_init);

    info!("{:?}", session.render_state().base_layer());

    Ok(())
}

struct XrFrame {
    pub time: f64,
    pub webxr_frame: web_sys::XrFrame,
}

fn request_web_xr_frame(session: &XrSession, pointer: *mut App) -> Result<(), WebXrError> {
    info!("starting webxr_rendering_loop");
    // Wierd hacky closure stuff that I don't understand. Taken from a wasm-bindgen example:
    // https://github.com/rustwasm/wasm-bindgen/blob/ebe658739c075fe78781d87ee9aa46533922476d/examples/webxr/src/lib.rs#L119-L151
    let closure: Rc<RefCell<Option<Closure<dyn FnMut(f64, web_sys::XrFrame)>>>>;
    closure = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    *closure.borrow_mut() = Some(Closure::wrap(Box::new(
        move |time: f64, frame: web_sys::XrFrame| {
            info!("xr frame update!");
            let mut app = unsafe { pointer.clone().read() };
            //// Insert XRFrame stuff
            app.world.insert_non_send_resource(XrFrame {
                time: time,
                webxr_frame: frame,
            });

            app.update();

            let frame = app
                .world
                .remove_non_send_resource::<XrFrame>()
                .unwrap()
                .webxr_frame;

            //request_animation_frame(&session, closure_clone.borrow().as_ref().unwrap());
            let frame_index = frame.session().request_animation_frame(
                closure_clone
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            );

            let mut string = "Xr Frame #".to_string();
            string.push_str(&frame_index.to_string());
            string.push_str(" requested!");
            info!(string);
        },
    )
        as Box<dyn FnMut(f64, web_sys::XrFrame)>));

    let frame_index = session
        .request_animation_frame(closure.borrow().as_ref().unwrap().as_ref().unchecked_ref());

    let mut string = "Xr Frame #".to_string();
    string.push_str(&frame_index.to_string());
    string.push_str(" requested!");
    info!(string);

    Ok(())
}
