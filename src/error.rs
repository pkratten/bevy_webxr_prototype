use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error, Clone)]
pub enum WebXrError {
    #[error("Some js error.")]
    JsError(JsValue),
    #[error("Failed to get bool from JsValue!")]
    NotABool,
    #[error("WebXr currently not supported!")]
    NotSupported,
    #[error("Canvas couldn't be found!")]
    CanvasNotFound,
    #[error("WebGL2 context not found!")]
    WebGl2ContextNotFound,
    #[error("Session lost!")]
    SessionError,
    #[error("Browser window not found!")]
    NoWindow,
    #[error("HTML Document not found!")]
    NoDocument,
    #[error("XR Initialize Element is not a html button element!")]
    ElementNotButtonElement(web_sys::Element),
    #[error("HTML Body not found!")]
    NoBody,
    #[error("Failed to request xr session!")]
    SessionRequestError(JsValue),
}
