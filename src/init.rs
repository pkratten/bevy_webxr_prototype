use crate::{error::WebXrError, WebXrContext, WebXrSettings};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{EventTarget, HtmlButtonElement, WebGl2RenderingContext, XrSession, XrSessionMode};

pub(crate) async fn initialize_webxr(
    settings: WebXrSettings,
    session_ref: Rc<RefCell<Option<Result<XrSession, WebXrError>>>>,
) {
    if let Ok(supported_session) = get_supported_sessions().await {
        if supported_session.vr & settings.vr_supported {
            initialize_button(&settings.vr_button, ButtonType::VR, session_ref.clone())
        };
        if supported_session.ar & settings.ar_supported {
            initialize_button(&settings.ar_button, ButtonType::AR, session_ref.clone())
        };
    } else {
        error!("WebXR not supported!");
    }
}

struct SupportedSessions {
    inline: bool,
    vr: bool,
    ar: bool,
}

// impl SupportedSessions {
//     pub fn any(self) -> bool {
//         self.inline | self.vr | self.ar
//     }
// }

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

#[derive(Clone, Copy)]
enum ButtonType {
    VR,
    AR,
}

fn initialize_button(
    selector: &str,
    button_type: ButtonType,
    session_ref: Rc<RefCell<Option<Result<XrSession, WebXrError>>>>,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let button: HtmlButtonElement = if let Ok(Some(element)) = document.query_selector(selector) {
        element.dyn_into().unwrap()
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
        button
    };

    //button.set_attribute("disabled", "true").unwrap();

    let button: HtmlButtonElement = button.dyn_into().unwrap();

    let closure = Closure::<dyn FnMut()>::new(move || {
        let session_ref = session_ref.clone();
        AsyncComputeTaskPool::get().spawn(async move {
            info!("Session await spawned!");
            let xr = web_sys::window().unwrap().navigator().xr();

            let session =
                wasm_bindgen_futures::JsFuture::from(xr.request_session(match button_type {
                    ButtonType::VR => XrSessionMode::ImmersiveVr,
                    ButtonType::AR => XrSessionMode::ImmersiveAr,
                }))
                .await;

            match session {
                Ok(session) => {
                    session_ref.replace(Some(Ok(session.into())));
                }
                Err(err) => {
                    session_ref.replace(Some(Err(WebXrError::JsError(err))));
                }
            };
        });
    });

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}
