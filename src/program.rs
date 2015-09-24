use std::ptr;
use std::marker::PhantomData;
use std::ffi::CString;
use gl;
use gl::types::*;
use types::GLError;
use context::Context;
use shader::Shader;
use uniform_data::{UniformData, UniformDatumType, UniformPrimitiveType};

pub struct Program {
    gl_id: GLuint
}

impl Program {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}



unsafe fn _get_program_iv(program: &Program,
                          pname: GLenum,
                          params: *mut GLint)
{
    gl::GetProgramiv(program.gl_id(), pname, params);
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`pname` is not an accepted value",
        GLError::InvalidValue => "`program` is not a value generated by OpenGL",
        GLError::InvalidOperation => "`program` does not refer to a program object",
        _ => "Unknown error"
    }
}

impl Context {
    pub fn create_program(&self) -> Result<Program, ()> {
        unsafe {
            let id = gl::CreateProgram();
            if id > 0 {
                Ok(Program { gl_id: id })
            }
            else {
                Err(())
            }
        }
    }

    pub fn attach_shader(&self, program: &mut Program, shader: &Shader) {
        unsafe {
            gl::AttachShader(program.gl_id(), shader.gl_id());
            dbg_gl_error! {
                GLError::InvalidValue => "One of either `program` or `shader` is not an OpenGL object",
                GLError::InvalidOperation => "`shader` is already attached to `program`, `shader` is not a shader object, or `program` is not a program object",
                _ => "Unknown error"
            }
        }
    }

    pub fn link_program(&self, program: &mut Program) -> Result<(), GLError> {
        let success = unsafe {
            gl::LinkProgram(program.gl_id());
            dbg_gl_error! {
                GLError::InvalidValue => "`program` is not a value from OpenGL",
                GLError::InvalidOperation => "`program` is not a program object",
                _ => "Unknown error"
            }

            let mut link_status : GLint = 0;
            _get_program_iv(program,
                            gl::LINK_STATUS,
                            &mut link_status as *mut GLint);

            link_status == gl::TRUE as GLint
        };

        if success {
            Ok(())
        }
        else {
            let msg = match self.get_program_info_log(&program) {
                Some(s) => { s },
                None => { String::from("[Unknown program error]") }
            };
            Err(GLError::Message(msg))
        }
    }

    pub fn get_program_info_log(&self, program: &Program) -> Option<String> {
        unsafe {
            let mut info_length : GLint = 0;
            _get_program_iv(program,
                            gl::INFO_LOG_LENGTH,
                            &mut info_length as *mut GLint);

            if info_length > 0 {
                let mut bytes = Vec::<u8>::with_capacity(info_length as usize);

                gl::GetProgramInfoLog(program.gl_id(),
                                      info_length,
                                      ptr::null_mut(),
                                      bytes.as_mut_ptr() as *mut GLchar);
                dbg_gl_sanity_check! {
                    GLError::InvalidValue => "`program` is not a value generated by OpenGL, or `maxLength` < 0",
                    GLError::InvalidOperation => "`program` is not a program object",
                    _ => "Unknown error"
                }
                bytes.set_len((info_length - 1) as usize);

                String::from_utf8(bytes).ok()
            }
            else {
                None
            }
        }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str)
        -> Result<ProgramAttrib, ()>
    {
        let c_str = try!(CString::new(name).or(Err(())));
        let str_ptr = c_str.as_ptr() as *const GLchar;
        unsafe {
            let index = gl::GetAttribLocation(program.gl_id(), str_ptr);
            dbg_gl_error! {
                GLError::InvalidOperation => "`program` has not been linked, `program` is not a program object, or `program` is not a value generated by OpenGL",
                _ => "Unknown error"
            }

            if index >= 0 {
                Ok(ProgramAttrib { gl_index: index as GLuint })
            }
            else {
                Err(())
            }
        }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str)
        -> Result<ProgramUniform, ()>
    {
        let c_str = try!(CString::new(name).or(Err(())));
        let str_ptr = c_str.as_ptr() as *const GLchar;
        unsafe {
            let index = gl::GetUniformLocation(program.gl_id(), str_ptr);
            dbg_gl_error! {
                GLError::InvalidValue => "`program` is not a value generated by OpenGL",
                GLError::InvalidOperation => "`program` is not a program object, or has not been successfully linked",
                _ => "Unknown error"
            }

            if index >= 0 {
                Ok(ProgramUniform { gl_index: index as GLuint })
            }
            else {
                Err(())
            }
        }
    }
}


pub struct ProgramBinding<'a> {
    phantom: PhantomData<&'a mut Program>
}

impl<'a> ProgramBinding<'a> {
    pub fn set_uniform<T>(&self, uniform: ProgramUniform, val: T)
        where T: UniformData
    {
        let idx = uniform.gl_index as GLint;
        let count = val.uniform_elements() as GLsizei;
        let ptr = val.uniform_bytes().as_ptr();
        unsafe {
            match T::uniform_datum_type() {
                UniformDatumType::Vec1(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform1fv(idx, count, ptr as *const GLfloat);
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform1iv(idx, count, ptr as *const GLint);
                        }
                    }
                },
                UniformDatumType::Vec2(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform2fv(idx, count, ptr as *const GLfloat);
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform2iv(idx, count, ptr as *const GLint);
                        }
                    }
                },
                UniformDatumType::Vec3(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform3fv(idx, count, ptr as *const GLfloat);
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform3iv(idx, count, ptr as *const GLint);
                        }
                    }
                },
                UniformDatumType::Vec4(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform4fv(idx, count, ptr as *const GLfloat);
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform4iv(idx, count, ptr as *const GLint);
                        }
                    }
                },
                UniformDatumType::Matrix2x2 => {
                    gl::UniformMatrix2fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat);
                },
                UniformDatumType::Matrix3x3 => {
                    gl::UniformMatrix3fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat);
                },
                UniformDatumType::Matrix4x4 => {
                    gl::UniformMatrix4fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat);
                },
            }

            dbg_gl_error! {
                GLError::InvalidOperation => "Invalid uniform operation",
                GLError::InvalidValue => "`count` < 0 or `transpose` is not GL_FALSE",
                _ => "Unknown error"
            }
        }
    }
}

pub struct ProgramBinder;
impl ProgramBinder {
    pub fn bind(&mut self, program: &mut Program) -> ProgramBinding
    {
        let binding = ProgramBinding { phantom: PhantomData };
        unsafe {
            gl::UseProgram(program.gl_id());
            dbg_gl_error! {
                GLError::InvalidValue => "`program` is neither 0 nor an object generated by OpenGL",
                GLError::InvalidOperation => "`program` is not a program object or `program` could not be made part of the current state",
                _ => "Unknown error"
            }
        }
        binding
    }
}



#[derive(Debug, Clone, Copy)]
pub struct ProgramAttrib {
    pub gl_index: GLuint
}

#[derive(Debug, Clone, Copy)]
pub struct ProgramUniform {
    pub gl_index: GLuint
}
