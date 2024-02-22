#[cfg(target_arch = "wasm32")]
pub use wasm_timer::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;
