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
}
