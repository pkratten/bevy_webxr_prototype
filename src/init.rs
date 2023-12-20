use crate::{error::WebXrError, events::WebXrSessionInitialized, WebXrSettings, XrFrame, XrMode};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool, winit::WinitSettings};
use std::{
    cell::{RefCell, UnsafeCell},
    rc::Rc,
    time::Duration,
};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{HtmlButtonElement, HtmlCanvasElement, XrSession, XrSessionMode};

///
/// This is central unsafe method to get winit and webxr working together in bevy with minimal upstreaming.
///  
pub(crate) fn webxr_runner(mut app: App) {
    let settings = app
        .world
        .remove_resource::<WebXrSettings>()
        .expect("WebXrSettings not found!");

    let unsafe_app = UnsafeCell::new(app);

    initialize_webxr(settings, unsafe_app.get());

    unsafe {
        let app = unsafe_app.get().read();

        info!("starting winit_runner");
        bevy::winit::winit_runner(app);
    }
}

async fn initialize_webxr(settings: WebXrSettings, app: *mut App) {
    let supported_sessions = get_supported_sessions().await.unwrap_throw();
    if supported_sessions.vr & settings.vr_supported {
        debug!(
            "Initialize vr button: {:?}",
            initialize_button(XrButtonType::VR, settings.clone(), app)
        );
    };
    if supported_sessions.ar & settings.ar_supported {
        debug!(
            "Initialize ar button: {:?}",
            initialize_button(XrButtonType::AR, settings.clone(), app)
        );
    };
    // maybe initialize inline here.ror!("WebXR not supported!");
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
    app: *mut App,
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
            app,
        ));
    });

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget();

    Ok(())
}

async fn initialize_session(
    mode: XrMode,
    settings: WebXrSettings,
    app: *mut App,
) -> Result<(), WebXrError> {
    let session = request_session(mode).await?;

    let canvas = initialize_canvas(Some(&settings.canvas))?;

    initialize_render_context(&session, &canvas).await?;

    request_first_web_xr_frame(&session, app, mode)?;

    Ok(())
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

fn initialize_canvas(canvas: Option<&str>) -> Result<web_sys::HtmlCanvasElement, WebXrError> {
    let window = web_sys::window().ok_or(WebXrError::NoWindow)?;
    let document = window.document().ok_or(WebXrError::NoDocument)?;

    if let Some(canvas) = canvas {
        document
            .query_selector(canvas)
            .map_err(|err| WebXrError::JsError(err))?
            .ok_or(WebXrError::CanvasNotFound)?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| WebXrError::CanvasNotFound)
    } else {
        todo!()
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

    debug!("{:?}", web_gl_layer);

    let mut render_state_init = web_sys::XrRenderStateInit::new();

    debug!("{:?}", render_state_init);

    render_state_init.base_layer(Some(&web_gl_layer));

    debug!("{:?}", render_state_init);

    session.update_render_state_with_state(&render_state_init);

    debug!("{:?}", session.render_state().base_layer());

    Ok(())
}

#[derive(Resource)]
struct WinitSettingsBackup(pub Option<WinitSettings>);

fn request_first_web_xr_frame(
    session: &XrSession,
    app: *mut App,
    mode: XrMode,
) -> Result<(), WebXrError> {
    info!("Starting webxr rendering!");

    // Wierd hacky closure stuff that I don't understand. Taken from a wasm-bindgen example:
    // https://github.com/rustwasm/wasm-bindgen/blob/ebe658739c075fe78781d87ee9aa46533922476d/examples/webxr/src/lib.rs#L119-L151
    let closure: Rc<RefCell<Option<Closure<dyn FnMut(f64, web_sys::XrFrame)>>>> =
        Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    *closure.borrow_mut() = Some(Closure::wrap(Box::new(
        move |time: f64, frame: web_sys::XrFrame| {
            debug!("Update xr frame!");
            let mut app = unsafe { app.clone().read() };
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

    let mut app = unsafe { app.clone().read() };

    let winit_settings = app.world.remove_resource();
    app.world
        .insert_resource(WinitSettingsBackup(winit_settings));
    app.world.insert_resource(WinitSettings {
        return_from_run: false,
        focused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
            wait: Duration::MAX,
        },
        unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
            wait: Duration::MAX,
        },
    });

    app.world.send_event(WebXrSessionInitialized(mode));

    Ok(())
}

fn print_frame_index(frame_index: u32) {
    let mut string = "Xr Frame #".to_string();
    string.push_str(&frame_index.to_string());
    string.push_str(" requested!");
    debug!(string);
}
