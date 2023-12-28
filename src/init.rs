use crate::{error::WebXrError, events::WebXrSessionInitialized, WebXrSettings, XrFrame, XrMode};
use bevy::{app::PluginsState, prelude::*, tasks::AsyncComputeTaskPool, winit::WinitSettings};
use std::sync::{Arc, Mutex};
use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
    time::Duration,
};
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{HtmlButtonElement, HtmlCanvasElement, XrReferenceSpace, XrSession, XrSessionMode};

///
/// This is central unsafe method to get winit and webxr working together in bevy with minimal upstreaming.
///  
pub(crate) fn webxr_runner(mut app: App) {
    let settings = app
        .world
        .remove_resource::<WebXrSettings>()
        .expect("WebXrSettings not found!");

    let app_mutex = Arc::new(Mutex::new(app));

    let app_clone = app_mutex.clone();
    spawn_local(async {
        initialize_webxr(settings, app_clone).await;
    });
}

async fn initialize_webxr(settings: WebXrSettings, app: Arc<Mutex<App>>) {
    let supported_sessions = get_supported_sessions().await.unwrap_throw();
    if supported_sessions.vr & settings.vr_supported {
        info!(
            "Initialize vr button: {:?}",
            initialize_button(XrButtonType::VR, settings.clone(), app.clone())
        );
    };
    if supported_sessions.ar & settings.ar_supported {
        info!(
            "Initialize ar button: {:?}",
            initialize_button(XrButtonType::AR, settings.clone(), app.clone())
        );
    };
    // maybe initialize inline here.
}

struct SupportedSessions {
    inline: bool,
    vr: bool,
    ar: bool,
}

async fn get_supported_sessions() -> Result<SupportedSessions, WebXrError> {
    let xr = web_sys::window()
        .ok_or(WebXrError::NoWindow)?
        .navigator()
        .xr();

    let inline =
        wasm_bindgen_futures::JsFuture::from(xr.is_session_supported(XrSessionMode::Inline))
            .await
            .map_err(|err| WebXrError::JsError(err))?
            .as_bool()
            .ok_or(WebXrError::NotABool)?;

    let vr =
        wasm_bindgen_futures::JsFuture::from(xr.is_session_supported(XrSessionMode::ImmersiveVr))
            .await
            .map_err(|err| WebXrError::JsError(err))?
            .as_bool()
            .ok_or(WebXrError::NotABool)?;

    let ar =
        wasm_bindgen_futures::JsFuture::from(xr.is_session_supported(XrSessionMode::ImmersiveAr))
            .await
            .map_err(|err| WebXrError::JsError(err))?
            .as_bool()
            .ok_or(WebXrError::NotABool)?;

    Ok(SupportedSessions { inline, vr, ar })
}

#[derive(Clone, Copy)]
enum XrButtonType {
    VR,
    AR,
}

fn initialize_button(
    button_type: XrButtonType,
    settings: WebXrSettings,
    app: Arc<Mutex<App>>,
) -> Result<(), WebXrError> {
    let document = web_sys::window()
        .ok_or(WebXrError::NoWindow)?
        .document()
        .ok_or(WebXrError::NoDocument)?;

    let button: HtmlButtonElement = if let Ok(Some(element)) =
        document.query_selector(match button_type {
            XrButtonType::VR => &settings.vr_button,
            XrButtonType::AR => &settings.ar_button,
        }) {
        element
            .dyn_into()
            .map_err(|err| WebXrError::ElementNotButtonElement(err))?
    } else {
        let button = document
            .create_element("button")
            .map_err(|err| WebXrError::JsError(err))?
            .dyn_into::<web_sys::HtmlButtonElement>()
            .map_err(|err| WebXrError::ElementNotButtonElement(err))?;
        match button_type {
            XrButtonType::VR => button.set_inner_text("Enter VR"),
            XrButtonType::AR => button.set_inner_text("Enter AR"),
        }
        document
            .body()
            .ok_or(WebXrError::NoBody)?
            .append_child(&button)
            .map_err(|err| WebXrError::JsError(err))?;
        button
    };

    //button.set_attribute("disabled", "true").unwrap();

    let closure = Closure::<dyn FnMut()>::new(move || {
        AsyncComputeTaskPool::get().spawn(initialize_session(
            match button_type {
                XrButtonType::VR => XrMode::VR,
                XrButtonType::AR => XrMode::AR,
            },
            settings.clone(),
            app.clone(),
        ));
    });

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget();

    Ok(())
}

async fn initialize_session(mode: XrMode, settings: WebXrSettings, app: Arc<Mutex<App>>) {
    let session = request_session(mode).await;

    info!("Session requested: {:?}", session);

    let session = session.unwrap_throw();

    let canvas = initialize_canvas(&settings.canvas);

    info!("Canvas initialized: {:?}", canvas);

    let canvas = canvas.unwrap_throw();

    info!(
        "Reference space initialized: {:?}",
        initialize_reference_space(&session).await
    );

    info!(
        "Render context initialized{:?}",
        initialize_render_context(&session, &canvas).await
    );

    info!(
        "Frame initialized: {:?}",
        request_first_web_xr_frame(&session, app, mode)
    );
}

async fn request_session(mode: XrMode) -> Result<XrSession, WebXrError> {
    let xr = web_sys::window()
        .ok_or(WebXrError::NoWindow)?
        .navigator()
        .xr();

    let session = wasm_bindgen_futures::JsFuture::from(xr.request_session(match mode {
        XrMode::VR => XrSessionMode::ImmersiveVr,
        XrMode::AR => XrSessionMode::ImmersiveAr,
        XrMode::Inline => XrSessionMode::Inline,
    }))
    .await
    .map(|session| session.into())
    .map_err(|err| WebXrError::SessionRequestError(err));

    session
}

pub fn initialize_canvas(canvas: &str) -> Result<web_sys::HtmlCanvasElement, WebXrError> {
    let window = web_sys::window().ok_or(WebXrError::NoWindow)?;
    let document = window.document().ok_or(WebXrError::NoDocument)?;

    if let Ok(Some(canvas_element)) = document.query_selector(canvas) {
        let canvas_element = canvas_element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|err| WebXrError::ElementNotCanvasElement(err))?;
        Ok(canvas_element)
    } else {
        let element = document
            .create_element("canvas")
            .map_err(|err| WebXrError::JsError(err))?;
        let canvas_element = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|err| WebXrError::ElementNotCanvasElement(err))?;
        canvas_element.set_id(canvas);
        canvas_element.set_attribute(canvas, canvas).unwrap();
        document
            .body()
            .ok_or(WebXrError::NoBody)?
            .append_child(&canvas_element)
            .map_err(|err| WebXrError::JsError(err))?;
        Ok(canvas_element)
    }
}

async fn initialize_render_context(
    session: &XrSession,
    canvas: &HtmlCanvasElement,
) -> Result<(), WebXrError> {
    let context = canvas
        .get_context("webgl2")
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::WebGl2ContextNotFound)?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| WebXrError::WebGl2ContextNotFound)?;

    info!("Rendering context before xr compatible: {:?}", context);

    let promise = wasm_bindgen_futures::JsFuture::from(context.make_xr_compatible())
        .await
        .map_err(|err| WebXrError::JsError(err))?;

    info!("Promise of make xr compatible: {:?}", promise);
    info!("Rendering context after xr compatible: {:?}", context);

    let web_gl_layer =
        web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context(&session, &context)
            .map_err(|err| WebXrError::JsError(err))?;

    info!("XrWeGlLayer: {:?}", web_gl_layer);

    let mut render_state_init = web_sys::XrRenderStateInit::new();

    info!(
        "Render state init before base layer: {:?}",
        render_state_init
    );

    render_state_init.base_layer(Some(&web_gl_layer));

    info!(
        "Render state init after base layer: {:?}",
        render_state_init
    );

    session.update_render_state_with_state(&render_state_init);

    info!(
        "Render state base layer of session after update session render state: {:?}",
        session.render_state().base_layer()
    );

    Ok(())
}

async fn initialize_reference_space(session: &XrSession) -> Result<XrReferenceSpace, WebXrError> {
    let reference_space =
        JsFuture::from(session.request_reference_space(web_sys::XrReferenceSpaceType::Local))
            .await
            .map_err(|err| WebXrError::JsError(err))?
            .into();

    Ok(reference_space)
}

fn request_first_web_xr_frame(
    session: &XrSession,
    app: Arc<Mutex<App>>,
    mode: XrMode,
) -> Result<(), WebXrError> {
    info!("Starting webxr rendering!");

    // Wierd hacky closure stuff that I don't understand. Taken from a wasm-bindgen example:
    // https://github.com/rustwasm/wasm-bindgen/blob/ebe658739c075fe78781d87ee9aa46533922476d/examples/webxr/src/lib.rs#L119-L151
    let closure: Rc<RefCell<Option<Closure<dyn FnMut(f64, web_sys::XrFrame)>>>> =
        Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let app_clone = app.clone();

    *closure.borrow_mut() = Some(Closure::wrap(Box::new(
        move |time: f64, frame: web_sys::XrFrame| {
            info!("Update xr frame!");

            let mut app = app_clone.lock().unwrap();
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

            print_frame_index(frame_index);
        },
    )
        as Box<dyn FnMut(f64, web_sys::XrFrame)>));

    let frame_index = session
        .request_animation_frame(closure.borrow().as_ref().unwrap().as_ref().unchecked_ref());

    print_frame_index(frame_index);

    let mut app = app.lock().unwrap();

    app.world.send_event(WebXrSessionInitialized(mode));

    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    Ok(())
}

fn print_frame_index(frame_index: u32) {
    let mut string = "Xr Frame #".to_string();
    string.push_str(&frame_index.to_string());
    string.push_str(" requested!");
    info!(string);
}
