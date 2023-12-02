use crate::{error::WebXrError, WebXrContext, WebXrSettings};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use std::cell::RefCell;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{EventTarget, WebGl2RenderingContext, XrSession, XrSessionMode};

pub(crate) async fn initialize_webxr(
    settings: WebXrSettings,
    canvas: &str,
    session_ref: RefCell<Option<Result<XrSession, WebXrError>>>,
    render_context_ref: RefCell<Option<Result<WebGl2RenderingContext, WebXrError>>>,
) -> Result<WebXrContext, WebXrError> {
    if !settings.vr_supported & !settings.ar_supported {
        warn!("WebXR VR and AR are disabled!");
    }

    let supported_sessions = get_supported_sessions().await?;

    if !supported_sessions.any() {
        return Err(WebXrError::NotSupported);
    }

    let context = get_gl_context(canvas).await;
    render_context_ref.replace(Some(context));

    if settings.vr_supported {
        initialize_button(&settings.vr_button, ButtonType::VR, session_ref.clone())
    }
    if settings.ar_supported {
        initialize_button(&settings.ar_button, ButtonType::AR, session_ref.clone())
    }

    todo!()
}

struct SupportedSessions {
    inline: bool,
    vr: bool,
    ar: bool,
}

impl SupportedSessions {
    pub fn any(self) -> bool {
        self.inline | self.vr | self.ar
    }
}

async fn get_supported_sessions() -> Result<SupportedSessions, WebXrError> {
    let xr = web_sys::window().unwrap().navigator().xr();

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

async fn get_gl_context(window_selector: &str) -> Result<WebGl2RenderingContext, WebXrError> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let context = document
        .query_selector(&window_selector)
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::CanvasNotFound)?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| WebXrError::CanvasNotFound)?
        .get_context("webgl2")
        .map_err(|err| WebXrError::JsError(err))?
        .ok_or(WebXrError::WebGl2ContextNotFound)?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| WebXrError::WebGl2ContextNotFound)?;

    wasm_bindgen_futures::JsFuture::from(context.make_xr_compatible())
        .await
        .map_err(|err| WebXrError::JsError(err))?;

    Ok(context)
}

#[derive(Clone, Copy)]
enum ButtonType {
    VR,
    AR,
}

fn initialize_button(
    selector: &str,
    button_type: ButtonType,
    session_ref: RefCell<Option<Result<XrSession, WebXrError>>>,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let button = if let Ok(Some(element)) = document.query_selector(selector) {
        element
    } else {
        let button = document
            .create_element("button")
            .unwrap()
            .dyn_into::<web_sys::HtmlButtonElement>()
            .unwrap();
        match button_type {
            ButtonType::VR => button.set_inner_text("Enter VR"),
            ButtonType::AR => button.set_inner_text("Enter AR"),
        }
        document.body().unwrap().append_child(&button).unwrap();
        button.into()
    };
    button.set_attribute("disabled", "true").unwrap();

    let event_target: EventTarget = button.clone().into();
    let mousedown_closure = Closure::<dyn FnMut()>::new(move || {
        let session_ref = session_ref.clone();
        info!("Request session mousedown!");
        let xr = web_sys::window().unwrap().navigator().xr();
        let session = wasm_bindgen_futures::JsFuture::from(match button_type {
            ButtonType::VR => xr.request_session(XrSessionMode::ImmersiveVr),
            ButtonType::AR => xr.request_session(XrSessionMode::ImmersiveAr),
        });
        AsyncComputeTaskPool::get().spawn(async move {
            match session.await {
                Ok(session) => session_ref.replace(Some(Ok(session.into()))),
                Err(err) => session_ref.replace(Some(Err(WebXrError::JsError(err)))),
            };
        });
    });
    event_target
        .add_event_listener_with_callback(&"mousedown", mousedown_closure.as_ref().unchecked_ref())
        .unwrap();
    mousedown_closure.forget();
}
