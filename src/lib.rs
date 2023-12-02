use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use error::WebXrError;
use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, XrSession};

pub mod error;
mod init;

#[derive(Clone)]
struct WebXrSettings {
    vr_supported: bool,
    ar_supported: bool,
    pub vr_button: String,
    pub ar_button: String,
}

struct WebXrContext {
    pub session: RefCell<Option<Result<XrSession, WebXrError>>>,
    pub render_context: RefCell<Option<Result<WebGl2RenderingContext, WebXrError>>>,
    pub render_state: RefCell<Result<(), WebXrError>>,
    pub initialized: bool,
}

impl WebXrContext {
    fn default() -> WebXrContext {
        WebXrContext {
            session: RefCell::new(None),
            render_context: RefCell::new(None),
            render_state: RefCell::new(Ok(())),
            initialized: false,
        }
    }
}

#[derive(Event, Default)]
struct XrSessionRequestedEvent {}

struct AppPointer(*mut App);

pub struct WebXrPlugin {
    settings: WebXrSettings,
}

impl Plugin for WebXrPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(webxr_runner);
        let context = WebXrContext::default();
        AsyncComputeTaskPool::get().spawn(init::initialize_webxr(
            self.settings.clone(),
            "canvas[data-raw-handle]",
            context.session.clone(),
            context.render_context.clone(),
        ));
        app.insert_non_send_resource(context);

        app.add_event::<XrSessionRequestedEvent>();

        app.add_systems(PreUpdate, on_xr_session_requested);
    }
}

fn on_xr_session_requested(
    mut context: NonSendMut<WebXrContext>,
    mut event_writer: EventWriter<XrSessionRequestedEvent>,
    pointer: NonSend<AppPointer>,
) {
    if !context.initialized {
        if let Some(Ok(session)) = &*context.session.clone().borrow() {
            if let Some(Ok(render_context)) = &*context.render_context.clone().borrow() {
                context.initialized = true;
                event_writer.send_default();
                context.render_state.replace(request_web_xr_frame(
                    session,
                    render_context,
                    pointer,
                ));
            }
        }
    }
}

fn request_web_xr_frame(
    session: &XrSession,
    render_context: &WebGl2RenderingContext,
    pointer: NonSend<'_, AppPointer>,
) -> Result<(), WebXrError> {
    trace!("Setting up WebXr render context...");
    let web_gl_layer =
        web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context(session, render_context)
            .map_err(|err| WebXrError::JsError(err))?;
    let mut render_state_init = web_sys::XrRenderStateInit::new();
    render_state_init.base_layer(Some(&web_gl_layer));
    session.update_render_state_with_state(&render_state_init);

    trace!("starting webxr_rendering_loop");
    // Wierd hacky closure stuff that I don't understand. Taken from a wasm-bindgen example:
    // https://github.com/rustwasm/wasm-bindgen/blob/ebe658739c075fe78781d87ee9aa46533922476d/examples/webxr/src/lib.rs#L119-L151
    let closure: Rc<RefCell<Option<Closure<dyn FnMut(f64, web_sys::XrFrame)>>>>;
    closure = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let pointer = pointer.0.clone();

    *closure.borrow_mut() = Some(Closure::wrap(Box::new(
        move |time: f64, frame: web_sys::XrFrame| {
            unsafe {
                info!("xr frame update!");
                let mut pointer = pointer.clone().read();
                //// Insert XRFrame stuff
                pointer.update();
            }

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

            // should the app update happen before or after new request animation frame?
            //func(time, frame);
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

fn webxr_runner(app: App) {
    //get winit canvas and make xr compatible.

    let unsafe_app = UnsafeCell::new(app);
    let pointer = unsafe_app.get();

    unsafe {
        let mut app = unsafe_app.get().read();
        app.insert_non_send_resource(AppPointer(pointer));

        trace!("starting winit_runner");
        bevy::winit::winit_runner(app);
    }
}
