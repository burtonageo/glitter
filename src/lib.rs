#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

mod context;
mod buffer;
mod types;

pub use gl_lib as gl;

pub use context::Context;
pub use buffer::Buffer;
pub use types::{Color, BufferBits,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};
