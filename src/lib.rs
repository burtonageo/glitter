#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

#[macro_use] mod context;
mod buffer;
mod shader;
mod program;
mod vertex_data;
mod vertex_buffer;
mod index_data;
mod uniform_data;
mod types;

pub use gl_lib as gl;

pub use context::Context;
pub use buffer::{Buffer, BufferBinding, BufferDataUsage,
                 STREAM_DRAW, STATIC_DRAW, DYNAMIC_DRAW,
                 ArrayBufferBinder, ElementArrayBufferBinder,
                 ArrayBufferBinding, ElementArrayBufferBinding};
pub use shader::{Shader, ShaderType, VERTEX_SHADER, FRAGMENT_SHADER};
pub use program::{Program, ProgramAttrib, ProgramBinder, ProgramBinding};
pub use uniform_data::{UniformData, UniformDatum, UniformPrimitive,
                       UniformPrimitiveType, UniformDatumType};
pub use vertex_data::{VertexData, VertexDatum,
                      VertexBytes, VertexAttribBinder};
pub use index_data::{IndexData, IndexDatum, IndexDatumType};
pub use vertex_buffer::{VertexBuffer, VertexBufferBinding, IndexBuffer};
pub use types::{Color, Viewport, GLError, BufferBits,
                DrawingMode, DataType,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT,
                POINTS, LINE_STRIP, LINE_LOOP, LINES,
                TRIANGLE_STRIP, TRIANGLE_FAN, TRIANGLES,
                BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT,
                FIXED, FLOAT};
