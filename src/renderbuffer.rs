use std::marker::PhantomData;
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::ContextOf;
use types::GLError;

pub struct Renderbuffer {
    gl_id: GLuint
}

impl Renderbuffer {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl<AB, EAB, P, FB, RB, TU> ContextOf<AB, EAB, P, FB, RB, TU> {
    pub fn gen_renderbuffer(&self) -> Renderbuffer {
        unsafe {
            let mut id : GLuint = 0;

            gl::GenRenderbuffers(1, &mut id as *mut GLuint);
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`n` is negative",
                _ => "Unknown error"
            }

            Renderbuffer {
                gl_id: id
            }
        }
    }

    pub fn bind_renderbuffer<'a>(self, renderbuffer: &'a mut Renderbuffer)
        -> (
            RenderbufferBinding<'a>,
            ContextOf<AB, EAB, P, FB, (), TU>
        )
        where RB: BorrowMut<RenderbufferBinder>
    {
        let (mut renderbuffer_binder, gl) = self.split_renderbuffer();
        (renderbuffer_binder.borrow_mut().bind(renderbuffer), gl)
    }
}



gl_enum! {
    pub gl_enum RenderbufferTarget {
        Renderbuffer as RENDERBUFFER = gl::RENDERBUFFER
    }
}

pub struct RenderbufferBinding<'a> {
    phantom: PhantomData<&'a mut Renderbuffer>
}

impl<'a> RenderbufferBinding<'a> {
    fn target(&self) -> RenderbufferTarget {
        RenderbufferTarget::Renderbuffer
    }
}

pub struct RenderbufferBinder;
impl RenderbufferBinder {
    pub fn bind<'a>(&mut self, renderbuffer: &'a mut Renderbuffer)
        -> RenderbufferBinding<'a>
    {
        let binding = RenderbufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindRenderbuffer(binding.target().gl_enum(),
                                 renderbuffer.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
