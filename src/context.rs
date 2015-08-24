use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::buffer::{ArrayBufferBinder, ElementArrayBufferBinder};
use super::types::DrawingMode;

pub struct Context {
    pub array_buffer: ArrayBufferBinder,
    pub element_array_buffer: ElementArrayBufferBinder
}

impl Context {
    pub unsafe fn current_context() -> Self {
        Context {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder
        }
    }

    pub fn clear_color(&mut self, color: super::Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&mut self, buffers: super::BufferBits) {
        unsafe {
            gl::Clear(buffers.bits())
        }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: super::ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
        }
    }

    pub unsafe fn draw_arrays(&self,
                              mode: DrawingMode,
                              first: u32,
                              count: usize)
    {
        gl::DrawArrays(mode as GLenum, first as GLint, count as GLsizei);
    }
}

#[macro_export]
macro_rules! bind_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.array_buffer.bind($buffer)
    }
}

#[macro_export]
macro_rules! bind_element_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.element_array_buffer.bind($buffer)
    }
}
