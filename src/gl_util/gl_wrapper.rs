use gl::types::*;
use std::any::Any;
use std::mem;
use std::ffi::c_void;

pub unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).ActiveShaderProgram.f)(pipeline, program) } else { gl::ActiveShaderProgram(pipeline, program) } }
pub unsafe fn ActiveTexture(texture: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).ActiveTexture.f)(texture) } else { gl::ActiveTexture(texture) } }
pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).AttachShader.f)(program, shader) } else { gl::AttachShader(program, shader) } }
pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>((*STORAGE).BeginConditionalRender.f)(id, mode) } else { gl::BeginConditionalRender(id, mode) } }
pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BeginQuery.f)(target, id) } else { gl::BeginQuery(target, id) } }
pub unsafe fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>((*STORAGE).BeginQueryIndexed.f)(target, index, id) } else { gl::BeginQueryIndexed(target, index, id) } }
pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).BeginTransformFeedback.f)(primitiveMode) } else { gl::BeginTransformFeedback(primitiveMode) } }
pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>((*STORAGE).BindAttribLocation.f)(program, index, name) } else { gl::BindAttribLocation(program, index, name) } }
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BindBuffer.f)(target, buffer) } else { gl::BindBuffer(target, buffer) } }
pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>((*STORAGE).BindBufferBase.f)(target, index, buffer) } else { gl::BindBufferBase(target, index, buffer) } }
pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).BindBufferRange.f)(target, index, buffer, offset, size) } else { gl::BindBufferRange(target, index, buffer, offset, size) } }
pub unsafe fn BindBuffersBase(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint) -> ()>((*STORAGE).BindBuffersBase.f)(target, first, count, buffers) } else { gl::BindBuffersBase(target, first, count, buffers) } }
pub unsafe fn BindBuffersRange(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizeiptr) -> ()>((*STORAGE).BindBuffersRange.f)(target, first, count, buffers, offsets, sizes) } else { gl::BindBuffersRange(target, first, count, buffers, offsets, sizes) } }
pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>((*STORAGE).BindFragDataLocation.f)(program, color, name) } else { gl::BindFragDataLocation(program, color, name) } }
pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, *const GLchar) -> ()>((*STORAGE).BindFragDataLocationIndexed.f)(program, colorNumber, index, name) } else { gl::BindFragDataLocationIndexed(program, colorNumber, index, name) } }
pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BindFramebuffer.f)(target, framebuffer) } else { gl::BindFramebuffer(target, framebuffer) } }
pub unsafe fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum) -> ()>((*STORAGE).BindImageTexture.f)(unit, texture, level, layered, layer, access, format) } else { gl::BindImageTexture(unit, texture, level, layered, layer, access, format) } }
pub unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>((*STORAGE).BindImageTextures.f)(first, count, textures) } else { gl::BindImageTextures(first, count, textures) } }
pub unsafe fn BindProgramPipeline(pipeline: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).BindProgramPipeline.f)(pipeline) } else { gl::BindProgramPipeline(pipeline) } }
pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BindRenderbuffer.f)(target, renderbuffer) } else { gl::BindRenderbuffer(target, renderbuffer) } }
pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).BindSampler.f)(unit, sampler) } else { gl::BindSampler(unit, sampler) } }
pub unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>((*STORAGE).BindSamplers.f)(first, count, samplers) } else { gl::BindSamplers(first, count, samplers) } }
pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BindTexture.f)(target, texture) } else { gl::BindTexture(target, texture) } }
pub unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).BindTextureUnit.f)(unit, texture) } else { gl::BindTextureUnit(unit, texture) } }
pub unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>((*STORAGE).BindTextures.f)(first, count, textures) } else { gl::BindTextures(first, count, textures) } }
pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).BindTransformFeedback.f)(target, id) } else { gl::BindTransformFeedback(target, id) } }
pub unsafe fn BindVertexArray(array: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).BindVertexArray.f)(array) } else { gl::BindVertexArray(array) } }
pub unsafe fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>((*STORAGE).BindVertexBuffer.f)(bindingindex, buffer, offset, stride) } else { gl::BindVertexBuffer(bindingindex, buffer, offset, stride) } }
pub unsafe fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>((*STORAGE).BindVertexBuffers.f)(first, count, buffers, offsets, strides) } else { gl::BindVertexBuffers(first, count, buffers, offsets, strides) } }
pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).BlendColor.f)(red, green, blue, alpha) } else { gl::BlendColor(red, green, blue, alpha) } }
pub unsafe fn BlendEquation(mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).BlendEquation.f)(mode) } else { gl::BlendEquation(mode) } }
pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).BlendEquationSeparate.f)(modeRGB, modeAlpha) } else { gl::BlendEquationSeparate(modeRGB, modeAlpha) } }
pub unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>((*STORAGE).BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) } else { gl::BlendEquationSeparatei(buf, modeRGB, modeAlpha) } }
pub unsafe fn BlendEquationi(buf: GLuint, mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>((*STORAGE).BlendEquationi.f)(buf, mode) } else { gl::BlendEquationi(buf, mode) } }
pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).BlendFunc.f)(sfactor, dfactor) } else { gl::BlendFunc(sfactor, dfactor) } }
pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>((*STORAGE).BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) } else { gl::BlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) } }
pub unsafe fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> ()>((*STORAGE).BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) } else { gl::BlendFuncSeparatei(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) } }
pub unsafe fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>((*STORAGE).BlendFunci.f)(buf, src, dst) } else { gl::BlendFunci(buf, src, dst) } }
pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>((*STORAGE).BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } else { gl::BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } }
pub unsafe fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>((*STORAGE).BlitNamedFramebuffer.f)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } else { gl::BlitNamedFramebuffer(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) } }
pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) -> ()>((*STORAGE).BufferData.f)(target, size, data, usage) } else { gl::BufferData(target, size, data, usage) } }
pub unsafe fn BufferStorage(target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield) -> ()>((*STORAGE).BufferStorage.f)(target, size, data, flags) } else { gl::BufferStorage(target, size, data, flags) } }
pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void) -> ()>((*STORAGE).BufferSubData.f)(target, offset, size, data) } else { gl::BufferSubData(target, offset, size, data) } }
pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>((*STORAGE).CheckFramebufferStatus.f)(target) } else { gl::CheckFramebufferStatus(target) } }
pub unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> GLenum>((*STORAGE).CheckNamedFramebufferStatus.f)(framebuffer, target) } else { gl::CheckNamedFramebufferStatus(framebuffer, target) } }
pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).ClampColor.f)(target, clamp) } else { gl::ClampColor(target, clamp) } }
pub unsafe fn Clear(mask: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>((*STORAGE).Clear.f)(mask) } else { gl::Clear(mask) } }
pub unsafe fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearBufferData.f)(target, internalformat, format, type_, data) } else { gl::ClearBufferData(target, internalformat, format, type_, data) } }
pub unsafe fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) } else { gl::ClearBufferSubData(target, internalformat, offset, size, format, type_, data) } }
pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>((*STORAGE).ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) } else { gl::ClearBufferfi(buffer, drawbuffer, depth, stencil) } }
pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>((*STORAGE).ClearBufferfv.f)(buffer, drawbuffer, value) } else { gl::ClearBufferfv(buffer, drawbuffer, value) } }
pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>((*STORAGE).ClearBufferiv.f)(buffer, drawbuffer, value) } else { gl::ClearBufferiv(buffer, drawbuffer, value) } }
pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>((*STORAGE).ClearBufferuiv.f)(buffer, drawbuffer, value) } else { gl::ClearBufferuiv(buffer, drawbuffer, value) } }
pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).ClearColor.f)(red, green, blue, alpha) } else { gl::ClearColor(red, green, blue, alpha) } }
pub unsafe fn ClearDepth(depth: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>((*STORAGE).ClearDepth.f)(depth) } else { gl::ClearDepth(depth) } }
pub unsafe fn ClearDepthf(d: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>((*STORAGE).ClearDepthf.f)(d) } else { gl::ClearDepthf(d) } }
pub unsafe fn ClearNamedBufferData(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearNamedBufferData.f)(buffer, internalformat, format, type_, data) } else { gl::ClearNamedBufferData(buffer, internalformat, format, type_, data) } }
pub unsafe fn ClearNamedBufferSubData(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearNamedBufferSubData.f)(buffer, internalformat, offset, size, format, type_, data) } else { gl::ClearNamedBufferSubData(buffer, internalformat, offset, size, format, type_, data) } }
pub unsafe fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint) -> ()>((*STORAGE).ClearNamedFramebufferfi.f)(framebuffer, buffer, drawbuffer, depth, stencil) } else { gl::ClearNamedFramebufferfi(framebuffer, buffer, drawbuffer, depth, stencil) } }
pub unsafe fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLfloat) -> ()>((*STORAGE).ClearNamedFramebufferfv.f)(framebuffer, buffer, drawbuffer, value) } else { gl::ClearNamedFramebufferfv(framebuffer, buffer, drawbuffer, value) } }
pub unsafe fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLint) -> ()>((*STORAGE).ClearNamedFramebufferiv.f)(framebuffer, buffer, drawbuffer, value) } else { gl::ClearNamedFramebufferiv(framebuffer, buffer, drawbuffer, value) } }
pub unsafe fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLuint) -> ()>((*STORAGE).ClearNamedFramebufferuiv.f)(framebuffer, buffer, drawbuffer, value) } else { gl::ClearNamedFramebufferuiv(framebuffer, buffer, drawbuffer, value) } }
pub unsafe fn ClearStencil(s: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint) -> ()>((*STORAGE).ClearStencil.f)(s) } else { gl::ClearStencil(s) } }
pub unsafe fn ClearTexImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearTexImage.f)(texture, level, format, type_, data) } else { gl::ClearTexImage(texture, level, format, type_, data) } }
pub unsafe fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).ClearTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) } else { gl::ClearTexSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) } }
pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>((*STORAGE).ClientWaitSync.f)(sync, flags, timeout) } else { gl::ClientWaitSync(sync, flags, timeout) } }
pub unsafe fn ClipControl(origin: GLenum, depth: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).ClipControl.f)(origin, depth) } else { gl::ClipControl(origin, depth) } }
pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>((*STORAGE).ColorMask.f)(red, green, blue, alpha) } else { gl::ColorMask(red, green, blue, alpha) } }
pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> ()>((*STORAGE).ColorMaski.f)(index, r, g, b, a) } else { gl::ColorMaski(index, r, g, b, a) } }
pub unsafe fn ColorP3ui(type_: GLenum, color: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).ColorP3ui.f)(type_, color) } else { gl::ColorP3ui(type_, color) } }
pub unsafe fn ColorP3uiv(type_: GLenum, color: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).ColorP3uiv.f)(type_, color) } else { gl::ColorP3uiv(type_, color) } }
pub unsafe fn ColorP4ui(type_: GLenum, color: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).ColorP4ui.f)(type_, color) } else { gl::ColorP4ui(type_, color) } }
pub unsafe fn ColorP4uiv(type_: GLenum, color: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).ColorP4uiv.f)(type_, color) } else { gl::ColorP4uiv(type_, color) } }
pub unsafe fn CompileShader(shader: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).CompileShader.f)(shader) } else { gl::CompileShader(shader) } }
pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) } else { gl::CompressedTexImage1D(target, level, internalformat, width, border, imageSize, data) } }
pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) } else { gl::CompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data) } }
pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) } else { gl::CompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data) } }
pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) } else { gl::CompressedTexSubImage1D(target, level, xoffset, width, format, imageSize, data) } }
pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) } else { gl::CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data) } }
pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) } else { gl::CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) } }
pub unsafe fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTextureSubImage1D.f)(texture, level, xoffset, width, format, imageSize, data) } else { gl::CompressedTextureSubImage1D(texture, level, xoffset, width, format, imageSize, data) } }
pub unsafe fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, imageSize, data) } else { gl::CompressedTextureSubImage2D(texture, level, xoffset, yoffset, width, height, format, imageSize, data) } }
pub unsafe fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).CompressedTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) } else { gl::CompressedTextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) } }
pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>((*STORAGE).CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) } else { gl::CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size) } }
pub unsafe fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>((*STORAGE).CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) } else { gl::CopyImageSubData(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) } }
pub unsafe fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr) -> ()>((*STORAGE).CopyNamedBufferSubData.f)(readBuffer, writeBuffer, readOffset, writeOffset, size) } else { gl::CopyNamedBufferSubData(readBuffer, writeBuffer, readOffset, writeOffset, size) } }
pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> ()>((*STORAGE).CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) } else { gl::CopyTexImage1D(target, level, internalformat, x, y, width, border) } }
pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>((*STORAGE).CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) } else { gl::CopyTexImage2D(target, level, internalformat, x, y, width, height, border) } }
pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>((*STORAGE).CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) } else { gl::CopyTexSubImage1D(target, level, xoffset, x, y, width) } }
pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) } else { gl::CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) } }
pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) } else { gl::CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height) } }
pub unsafe fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei) -> ()>((*STORAGE).CopyTextureSubImage1D.f)(texture, level, xoffset, x, y, width) } else { gl::CopyTextureSubImage1D(texture, level, xoffset, x, y, width) } }
pub unsafe fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).CopyTextureSubImage2D.f)(texture, level, xoffset, yoffset, x, y, width, height) } else { gl::CopyTextureSubImage2D(texture, level, xoffset, yoffset, x, y, width, height) } }
pub unsafe fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).CopyTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, x, y, width, height) } else { gl::CopyTextureSubImage3D(texture, level, xoffset, yoffset, zoffset, x, y, width, height) } }
pub unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateBuffers.f)(n, buffers) } else { gl::CreateBuffers(n, buffers) } }
pub unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateFramebuffers.f)(n, framebuffers) } else { gl::CreateFramebuffers(n, framebuffers) } }
pub unsafe fn CreateProgram() -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> GLuint>((*STORAGE).CreateProgram.f)() } else { gl::CreateProgram() } }
pub unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateProgramPipelines.f)(n, pipelines) } else { gl::CreateProgramPipelines(n, pipelines) } }
pub unsafe fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateQueries.f)(target, n, ids) } else { gl::CreateQueries(target, n, ids) } }
pub unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateRenderbuffers.f)(n, renderbuffers) } else { gl::CreateRenderbuffers(n, renderbuffers) } }
pub unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateSamplers.f)(n, samplers) } else { gl::CreateSamplers(n, samplers) } }
pub unsafe fn CreateShader(type_: GLenum) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>((*STORAGE).CreateShader.f)(type_) } else { gl::CreateShader(type_) } }
pub unsafe fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint>((*STORAGE).CreateShaderProgramv.f)(type_, count, strings) } else { gl::CreateShaderProgramv(type_, count, strings) } }
pub unsafe fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateTextures.f)(target, n, textures) } else { gl::CreateTextures(target, n, textures) } }
pub unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateTransformFeedbacks.f)(n, ids) } else { gl::CreateTransformFeedbacks(n, ids) } }
pub unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).CreateVertexArrays.f)(n, arrays) } else { gl::CreateVertexArrays(n, arrays) } }
pub unsafe fn CullFace(mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).CullFace.f)(mode) } else { gl::CullFace(mode) } }
pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const c_void) -> ()>((*STORAGE).DebugMessageCallback.f)(callback, userParam) } else { gl::DebugMessageCallback(callback, userParam) } }
pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> ()>((*STORAGE).DebugMessageControl.f)(source, type_, severity, count, ids, enabled) } else { gl::DebugMessageControl(source, type_, severity, count, ids, enabled) } }
pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> ()>((*STORAGE).DebugMessageInsert.f)(source, type_, id, severity, length, buf) } else { gl::DebugMessageInsert(source, type_, id, severity, length, buf) } }
pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteBuffers.f)(n, buffers) } else { gl::DeleteBuffers(n, buffers) } }
pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteFramebuffers.f)(n, framebuffers) } else { gl::DeleteFramebuffers(n, framebuffers) } }
pub unsafe fn DeleteProgram(program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).DeleteProgram.f)(program) } else { gl::DeleteProgram(program) } }
pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteProgramPipelines.f)(n, pipelines) } else { gl::DeleteProgramPipelines(n, pipelines) } }
pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteQueries.f)(n, ids) } else { gl::DeleteQueries(n, ids) } }
pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteRenderbuffers.f)(n, renderbuffers) } else { gl::DeleteRenderbuffers(n, renderbuffers) } }
pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteSamplers.f)(count, samplers) } else { gl::DeleteSamplers(count, samplers) } }
pub unsafe fn DeleteShader(shader: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).DeleteShader.f)(shader) } else { gl::DeleteShader(shader) } }
pub unsafe fn DeleteSync(sync: GLsync) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsync) -> ()>((*STORAGE).DeleteSync.f)(sync) } else { gl::DeleteSync(sync) } }
pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteTextures.f)(n, textures) } else { gl::DeleteTextures(n, textures) } }
pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteTransformFeedbacks.f)(n, ids) } else { gl::DeleteTransformFeedbacks(n, ids) } }
pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>((*STORAGE).DeleteVertexArrays.f)(n, arrays) } else { gl::DeleteVertexArrays(n, arrays) } }
pub unsafe fn DepthFunc(func: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).DepthFunc.f)(func) } else { gl::DepthFunc(func) } }
pub unsafe fn DepthMask(flag: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>((*STORAGE).DepthMask.f)(flag) } else { gl::DepthMask(flag) } }
pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>((*STORAGE).DepthRange.f)(n, f) } else { gl::DepthRange(n, f) } }
pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLdouble) -> ()>((*STORAGE).DepthRangeArrayv.f)(first, count, v) } else { gl::DepthRangeArrayv(first, count, v) } }
pub unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>((*STORAGE).DepthRangeIndexed.f)(index, n, f) } else { gl::DepthRangeIndexed(index, n, f) } }
pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>((*STORAGE).DepthRangef.f)(n, f) } else { gl::DepthRangef(n, f) } }
pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).DetachShader.f)(program, shader) } else { gl::DetachShader(program, shader) } }
pub unsafe fn Disable(cap: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).Disable.f)(cap) } else { gl::Disable(cap) } }
pub unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).DisableVertexArrayAttrib.f)(vaobj, index) } else { gl::DisableVertexArrayAttrib(vaobj, index) } }
pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).DisableVertexAttribArray.f)(index) } else { gl::DisableVertexAttribArray(index) } }
pub unsafe fn Disablei(target: GLenum, index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).Disablei.f)(target, index) } else { gl::Disablei(target, index) } }
pub unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) } else { gl::DispatchCompute(num_groups_x, num_groups_y, num_groups_z) } }
pub unsafe fn DispatchComputeIndirect(indirect: GLintptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLintptr) -> ()>((*STORAGE).DispatchComputeIndirect.f)(indirect) } else { gl::DispatchComputeIndirect(indirect) } }
pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>((*STORAGE).DrawArrays.f)(mode, first, count) } else { gl::DrawArrays(mode, first, count) } }
pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const c_void) -> ()>((*STORAGE).DrawArraysIndirect.f)(mode, indirect) } else { gl::DrawArraysIndirect(mode, indirect) } }
pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).DrawArraysInstanced.f)(mode, first, count, instancecount) } else { gl::DrawArraysInstanced(mode, first, count, instancecount) } }
pub unsafe fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint) -> ()>((*STORAGE).DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) } else { gl::DrawArraysInstancedBaseInstance(mode, first, count, instancecount, baseinstance) } }
pub unsafe fn DrawBuffer(buf: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).DrawBuffer.f)(buf) } else { gl::DrawBuffer(buf) } }
pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>((*STORAGE).DrawBuffers.f)(n, bufs) } else { gl::DrawBuffers(n, bufs) } }
pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void) -> ()>((*STORAGE).DrawElements.f)(mode, count, type_, indices) } else { gl::DrawElements(mode, count, type_, indices) } }
pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint) -> ()>((*STORAGE).DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) } else { gl::DrawElementsBaseVertex(mode, count, type_, indices, basevertex) } }
pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void) -> ()>((*STORAGE).DrawElementsIndirect.f)(mode, type_, indirect) } else { gl::DrawElementsIndirect(mode, type_, indirect) } }
pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei) -> ()>((*STORAGE).DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) } else { gl::DrawElementsInstanced(mode, count, type_, indices, instancecount) } }
pub unsafe fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint) -> ()>((*STORAGE).DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) } else { gl::DrawElementsInstancedBaseInstance(mode, count, type_, indices, instancecount, baseinstance) } }
pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint) -> ()>((*STORAGE).DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) } else { gl::DrawElementsInstancedBaseVertex(mode, count, type_, indices, instancecount, basevertex) } }
pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint) -> ()>((*STORAGE).DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) } else { gl::DrawElementsInstancedBaseVertexBaseInstance(mode, count, type_, indices, instancecount, basevertex, baseinstance) } }
pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void) -> ()>((*STORAGE).DrawRangeElements.f)(mode, start, end, count, type_, indices) } else { gl::DrawRangeElements(mode, start, end, count, type_, indices) } }
pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint) -> ()>((*STORAGE).DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) } else { gl::DrawRangeElementsBaseVertex(mode, start, end, count, type_, indices, basevertex) } }
pub unsafe fn DrawTransformFeedback(mode: GLenum, id: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).DrawTransformFeedback.f)(mode, id) } else { gl::DrawTransformFeedback(mode, id) } }
pub unsafe fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei) -> ()>((*STORAGE).DrawTransformFeedbackInstanced.f)(mode, id, instancecount) } else { gl::DrawTransformFeedbackInstanced(mode, id, instancecount) } }
pub unsafe fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>((*STORAGE).DrawTransformFeedbackStream.f)(mode, id, stream) } else { gl::DrawTransformFeedbackStream(mode, id, stream) } }
pub unsafe fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei) -> ()>((*STORAGE).DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) } else { gl::DrawTransformFeedbackStreamInstanced(mode, id, stream, instancecount) } }
pub unsafe fn Enable(cap: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).Enable.f)(cap) } else { gl::Enable(cap) } }
pub unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).EnableVertexArrayAttrib.f)(vaobj, index) } else { gl::EnableVertexArrayAttrib(vaobj, index) } }
pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).EnableVertexAttribArray.f)(index) } else { gl::EnableVertexAttribArray(index) } }
pub unsafe fn Enablei(target: GLenum, index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).Enablei.f)(target, index) } else { gl::Enablei(target, index) } }
pub unsafe fn EndConditionalRender() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).EndConditionalRender.f)() } else { gl::EndConditionalRender() } }
pub unsafe fn EndQuery(target: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).EndQuery.f)(target) } else { gl::EndQuery(target) } }
pub unsafe fn EndQueryIndexed(target: GLenum, index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).EndQueryIndexed.f)(target, index) } else { gl::EndQueryIndexed(target, index) } }
pub unsafe fn EndTransformFeedback() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).EndTransformFeedback.f)() } else { gl::EndTransformFeedback() } }
pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>((*STORAGE).FenceSync.f)(condition, flags) } else { gl::FenceSync(condition, flags) } }
pub unsafe fn Finish() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).Finish.f)() } else { gl::Finish() } }
pub unsafe fn Flush() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).Flush.f)() } else { gl::Flush() } }
pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>((*STORAGE).FlushMappedBufferRange.f)(target, offset, length) } else { gl::FlushMappedBufferRange(target, offset, length) } }
pub unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).FlushMappedNamedBufferRange.f)(buffer, offset, length) } else { gl::FlushMappedNamedBufferRange(buffer, offset, length) } }
pub unsafe fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>((*STORAGE).FramebufferParameteri.f)(target, pname, param) } else { gl::FramebufferParameteri(target, pname, param) } }
pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>((*STORAGE).FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) } else { gl::FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) } }
pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>((*STORAGE).FramebufferTexture.f)(target, attachment, texture, level) } else { gl::FramebufferTexture(target, attachment, texture, level) } }
pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>((*STORAGE).FramebufferTexture1D.f)(target, attachment, textarget, texture, level) } else { gl::FramebufferTexture1D(target, attachment, textarget, texture, level) } }
pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>((*STORAGE).FramebufferTexture2D.f)(target, attachment, textarget, texture, level) } else { gl::FramebufferTexture2D(target, attachment, textarget, texture, level) } }
pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) -> ()>((*STORAGE).FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) } else { gl::FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset) } }
pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>((*STORAGE).FramebufferTextureLayer.f)(target, attachment, texture, level, layer) } else { gl::FramebufferTextureLayer(target, attachment, texture, level, layer) } }
pub unsafe fn FrontFace(mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).FrontFace.f)(mode) } else { gl::FrontFace(mode) } }
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenBuffers.f)(n, buffers) } else { gl::GenBuffers(n, buffers) } }
pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenFramebuffers.f)(n, framebuffers) } else { gl::GenFramebuffers(n, framebuffers) } }
pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenProgramPipelines.f)(n, pipelines) } else { gl::GenProgramPipelines(n, pipelines) } }
pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenQueries.f)(n, ids) } else { gl::GenQueries(n, ids) } }
pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenRenderbuffers.f)(n, renderbuffers) } else { gl::GenRenderbuffers(n, renderbuffers) } }
pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenSamplers.f)(count, samplers) } else { gl::GenSamplers(count, samplers) } }
pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenTextures.f)(n, textures) } else { gl::GenTextures(n, textures) } }
pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenTransformFeedbacks.f)(n, ids) } else { gl::GenTransformFeedbacks(n, ids) } }
pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>((*STORAGE).GenVertexArrays.f)(n, arrays) } else { gl::GenVertexArrays(n, arrays) } }
pub unsafe fn GenerateMipmap(target: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).GenerateMipmap.f)(target) } else { gl::GenerateMipmap(target) } }
pub unsafe fn GenerateTextureMipmap(texture: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).GenerateTextureMipmap.f)(texture) } else { gl::GenerateTextureMipmap(texture) } }
pub unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) } else { gl::GetActiveAtomicCounterBufferiv(program, bufferIndex, pname, params) } }
pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>((*STORAGE).GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) } else { gl::GetActiveAttrib(program, index, bufSize, length, size, type_, name) } }
pub unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) } else { gl::GetActiveSubroutineName(program, shadertype, index, bufsize, length, name) } }
pub unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) } else { gl::GetActiveSubroutineUniformName(program, shadertype, index, bufsize, length, name) } }
pub unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) } else { gl::GetActiveSubroutineUniformiv(program, shadertype, index, pname, values) } }
pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>((*STORAGE).GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) } else { gl::GetActiveUniform(program, index, bufSize, length, size, type_, name) } }
pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) } else { gl::GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName) } }
pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) } else { gl::GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params) } }
pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) } else { gl::GetActiveUniformName(program, uniformIndex, bufSize, length, uniformName) } }
pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) } else { gl::GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params) } }
pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>((*STORAGE).GetAttachedShaders.f)(program, maxCount, count, shaders) } else { gl::GetAttachedShaders(program, maxCount, count, shaders) } }
pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>((*STORAGE).GetAttribLocation.f)(program, name) } else { gl::GetAttribLocation(program, name) } }
pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>((*STORAGE).GetBooleani_v.f)(target, index, data) } else { gl::GetBooleani_v(target, index, data) } }
pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>((*STORAGE).GetBooleanv.f)(pname, data) } else { gl::GetBooleanv(pname, data) } }
pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>((*STORAGE).GetBufferParameteri64v.f)(target, pname, params) } else { gl::GetBufferParameteri64v(target, pname, params) } }
pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetBufferParameteriv.f)(target, pname, params) } else { gl::GetBufferParameteriv(target, pname, params) } }
pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const *mut c_void) -> ()>((*STORAGE).GetBufferPointerv.f)(target, pname, params) } else { gl::GetBufferPointerv(target, pname, params) } }
pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void) -> ()>((*STORAGE).GetBufferSubData.f)(target, offset, size, data) } else { gl::GetBufferSubData(target, offset, size, data) } }
pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut c_void) -> ()>((*STORAGE).GetCompressedTexImage.f)(target, level, img) } else { gl::GetCompressedTexImage(target, level, img) } }
pub unsafe fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut c_void) -> ()>((*STORAGE).GetCompressedTextureImage.f)(texture, level, bufSize, pixels) } else { gl::GetCompressedTextureImage(texture, level, bufSize, pixels) } }
pub unsafe fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLsizei, *mut c_void) -> ()>((*STORAGE).GetCompressedTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) } else { gl::GetCompressedTextureSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) } }
pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint>((*STORAGE).GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) } else { gl::GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog) } }
pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLdouble) -> ()>((*STORAGE).GetDoublei_v.f)(target, index, data) } else { gl::GetDoublei_v(target, index, data) } }
pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>((*STORAGE).GetDoublev.f)(pname, data) } else { gl::GetDoublev(pname, data) } }
pub unsafe fn GetError() -> GLenum { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> GLenum>((*STORAGE).GetError.f)() } else { gl::GetError() } }
pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>((*STORAGE).GetFloati_v.f)(target, index, data) } else { gl::GetFloati_v(target, index, data) } }
pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>((*STORAGE).GetFloatv.f)(pname, data) } else { gl::GetFloatv(pname, data) } }
pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>((*STORAGE).GetFragDataIndex.f)(program, name) } else { gl::GetFragDataIndex(program, name) } }
pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>((*STORAGE).GetFragDataLocation.f)(program, name) } else { gl::GetFragDataLocation(program, name) } }
pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) } else { gl::GetFramebufferAttachmentParameteriv(target, attachment, pname, params) } }
pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetFramebufferParameteriv.f)(target, pname, params) } else { gl::GetFramebufferParameteriv(target, pname, params) } }
pub unsafe fn GetGraphicsResetStatus() -> GLenum { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> GLenum>((*STORAGE).GetGraphicsResetStatus.f)() } else { gl::GetGraphicsResetStatus() } }
pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>((*STORAGE).GetInteger64i_v.f)(target, index, data) } else { gl::GetInteger64i_v(target, index, data) } }
pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>((*STORAGE).GetInteger64v.f)(pname, data) } else { gl::GetInteger64v(pname, data) } }
pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>((*STORAGE).GetIntegeri_v.f)(target, index, data) } else { gl::GetIntegeri_v(target, index, data) } }
pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>((*STORAGE).GetIntegerv.f)(pname, data) } else { gl::GetIntegerv(pname, data) } }
pub unsafe fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64) -> ()>((*STORAGE).GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) } else { gl::GetInternalformati64v(target, internalformat, pname, bufSize, params) } }
pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> ()>((*STORAGE).GetInternalformativ.f)(target, internalformat, pname, bufSize, params) } else { gl::GetInternalformativ(target, internalformat, pname, bufSize, params) } }
pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>((*STORAGE).GetMultisamplefv.f)(pname, index, val) } else { gl::GetMultisamplefv(pname, index, val) } }
pub unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>((*STORAGE).GetNamedBufferParameteri64v.f)(buffer, pname, params) } else { gl::GetNamedBufferParameteri64v(buffer, pname, params) } }
pub unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetNamedBufferParameteriv.f)(buffer, pname, params) } else { gl::GetNamedBufferParameteriv(buffer, pname, params) } }
pub unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *const *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>((*STORAGE).GetNamedBufferPointerv.f)(buffer, pname, params) } else { gl::GetNamedBufferPointerv(buffer, pname, params) } }
pub unsafe fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void) -> ()>((*STORAGE).GetNamedBufferSubData.f)(buffer, offset, size, data) } else { gl::GetNamedBufferSubData(buffer, offset, size, data) } }
pub unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetNamedFramebufferAttachmentParameteriv.f)(framebuffer, attachment, pname, params) } else { gl::GetNamedFramebufferAttachmentParameteriv(framebuffer, attachment, pname, params) } }
pub unsafe fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GLenum, param: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetNamedFramebufferParameteriv.f)(framebuffer, pname, param) } else { gl::GetNamedFramebufferParameteriv(framebuffer, pname, param) } }
pub unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetNamedRenderbufferParameteriv.f)(renderbuffer, pname, params) } else { gl::GetNamedRenderbufferParameteriv(renderbuffer, pname, params) } }
pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetObjectLabel.f)(identifier, name, bufSize, length, label) } else { gl::GetObjectLabel(identifier, name, bufSize, length, label) } }
pub unsafe fn GetObjectPtrLabel(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetObjectPtrLabel.f)(ptr, bufSize, length, label) } else { gl::GetObjectPtrLabel(ptr, bufSize, length, label) } }
pub unsafe fn GetPointerv(pname: GLenum, params: *const *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const *mut c_void) -> ()>((*STORAGE).GetPointerv.f)(pname, params) } else { gl::GetPointerv(pname, params) } }
pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void) -> ()>((*STORAGE).GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) } else { gl::GetProgramBinary(program, bufSize, length, binaryFormat, binary) } }
pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetProgramInfoLog.f)(program, bufSize, length, infoLog) } else { gl::GetProgramInfoLog(program, bufSize, length, infoLog) } }
pub unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetProgramInterfaceiv.f)(program, programInterface, pname, params) } else { gl::GetProgramInterfaceiv(program, programInterface, pname, params) } }
pub unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) } else { gl::GetProgramPipelineInfoLog(pipeline, bufSize, length, infoLog) } }
pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetProgramPipelineiv.f)(pipeline, pname, params) } else { gl::GetProgramPipelineiv(pipeline, pname, params) } }
pub unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>((*STORAGE).GetProgramResourceIndex.f)(program, programInterface, name) } else { gl::GetProgramResourceIndex(program, programInterface, name) } }
pub unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>((*STORAGE).GetProgramResourceLocation.f)(program, programInterface, name) } else { gl::GetProgramResourceLocation(program, programInterface, name) } }
pub unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>((*STORAGE).GetProgramResourceLocationIndex.f)(program, programInterface, name) } else { gl::GetProgramResourceLocationIndex(program, programInterface, name) } }
pub unsafe fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) } else { gl::GetProgramResourceName(program, programInterface, index, bufSize, length, name) } }
pub unsafe fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *const GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>((*STORAGE).GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) } else { gl::GetProgramResourceiv(program, programInterface, index, propCount, props, bufSize, length, params) } }
pub unsafe fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetProgramStageiv.f)(program, shadertype, pname, values) } else { gl::GetProgramStageiv(program, shadertype, pname, values) } }
pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetProgramiv.f)(program, pname, params) } else { gl::GetProgramiv(program, pname, params) } }
pub unsafe fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>((*STORAGE).GetQueryBufferObjecti64v.f)(id, buffer, pname, offset) } else { gl::GetQueryBufferObjecti64v(id, buffer, pname, offset) } }
pub unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>((*STORAGE).GetQueryBufferObjectiv.f)(id, buffer, pname, offset) } else { gl::GetQueryBufferObjectiv(id, buffer, pname, offset) } }
pub unsafe fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>((*STORAGE).GetQueryBufferObjectui64v.f)(id, buffer, pname, offset) } else { gl::GetQueryBufferObjectui64v(id, buffer, pname, offset) } }
pub unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>((*STORAGE).GetQueryBufferObjectuiv.f)(id, buffer, pname, offset) } else { gl::GetQueryBufferObjectuiv(id, buffer, pname, offset) } }
pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetQueryIndexediv.f)(target, index, pname, params) } else { gl::GetQueryIndexediv(target, index, pname, params) } }
pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>((*STORAGE).GetQueryObjecti64v.f)(id, pname, params) } else { gl::GetQueryObjecti64v(id, pname, params) } }
pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetQueryObjectiv.f)(id, pname, params) } else { gl::GetQueryObjectiv(id, pname, params) } }
pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint64) -> ()>((*STORAGE).GetQueryObjectui64v.f)(id, pname, params) } else { gl::GetQueryObjectui64v(id, pname, params) } }
pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>((*STORAGE).GetQueryObjectuiv.f)(id, pname, params) } else { gl::GetQueryObjectuiv(id, pname, params) } }
pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetQueryiv.f)(target, pname, params) } else { gl::GetQueryiv(target, pname, params) } }
pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetRenderbufferParameteriv.f)(target, pname, params) } else { gl::GetRenderbufferParameteriv(target, pname, params) } }
pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetSamplerParameterIiv.f)(sampler, pname, params) } else { gl::GetSamplerParameterIiv(sampler, pname, params) } }
pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>((*STORAGE).GetSamplerParameterIuiv.f)(sampler, pname, params) } else { gl::GetSamplerParameterIuiv(sampler, pname, params) } }
pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetSamplerParameterfv.f)(sampler, pname, params) } else { gl::GetSamplerParameterfv(sampler, pname, params) } }
pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetSamplerParameteriv.f)(sampler, pname, params) } else { gl::GetSamplerParameteriv(sampler, pname, params) } }
pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetShaderInfoLog.f)(shader, bufSize, length, infoLog) } else { gl::GetShaderInfoLog(shader, bufSize, length, infoLog) } }
pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>((*STORAGE).GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) } else { gl::GetShaderPrecisionFormat(shadertype, precisiontype, range, precision) } }
pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>((*STORAGE).GetShaderSource.f)(shader, bufSize, length, source) } else { gl::GetShaderSource(shader, bufSize, length, source) } }
pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetShaderiv.f)(shader, pname, params) } else { gl::GetShaderiv(shader, pname, params) } }
pub unsafe fn GetString(name: GLenum) -> *const GLubyte { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>((*STORAGE).GetString.f)(name) } else { gl::GetString(name) } }
pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>((*STORAGE).GetStringi.f)(name, index) } else { gl::GetStringi(name, index) } }
pub unsafe fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>((*STORAGE).GetSubroutineIndex.f)(program, shadertype, name) } else { gl::GetSubroutineIndex(program, shadertype, name) } }
pub unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>((*STORAGE).GetSubroutineUniformLocation.f)(program, shadertype, name) } else { gl::GetSubroutineUniformLocation(program, shadertype, name) } }
pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>((*STORAGE).GetSynciv.f)(sync, pname, bufSize, length, values) } else { gl::GetSynciv(sync, pname, bufSize, length, values) } }
pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void) -> ()>((*STORAGE).GetTexImage.f)(target, level, format, type_, pixels) } else { gl::GetTexImage(target, level, format, type_, pixels) } }
pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetTexLevelParameterfv.f)(target, level, pname, params) } else { gl::GetTexLevelParameterfv(target, level, pname, params) } }
pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>((*STORAGE).GetTexLevelParameteriv.f)(target, level, pname, params) } else { gl::GetTexLevelParameteriv(target, level, pname, params) } }
pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetTexParameterIiv.f)(target, pname, params) } else { gl::GetTexParameterIiv(target, pname, params) } }
pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>((*STORAGE).GetTexParameterIuiv.f)(target, pname, params) } else { gl::GetTexParameterIuiv(target, pname, params) } }
pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetTexParameterfv.f)(target, pname, params) } else { gl::GetTexParameterfv(target, pname, params) } }
pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>((*STORAGE).GetTexParameteriv.f)(target, pname, params) } else { gl::GetTexParameteriv(target, pname, params) } }
pub unsafe fn GetTextureImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetTextureImage.f)(texture, level, format, type_, bufSize, pixels) } else { gl::GetTextureImage(texture, level, format, type_, bufSize, pixels) } }
pub unsafe fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetTextureLevelParameterfv.f)(texture, level, pname, params) } else { gl::GetTextureLevelParameterfv(texture, level, pname, params) } }
pub unsafe fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLint) -> ()>((*STORAGE).GetTextureLevelParameteriv.f)(texture, level, pname, params) } else { gl::GetTextureLevelParameteriv(texture, level, pname, params) } }
pub unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetTextureParameterIiv.f)(texture, pname, params) } else { gl::GetTextureParameterIiv(texture, pname, params) } }
pub unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>((*STORAGE).GetTextureParameterIuiv.f)(texture, pname, params) } else { gl::GetTextureParameterIuiv(texture, pname, params) } }
pub unsafe fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetTextureParameterfv.f)(texture, pname, params) } else { gl::GetTextureParameterfv(texture, pname, params) } }
pub unsafe fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetTextureParameteriv.f)(texture, pname, params) } else { gl::GetTextureParameteriv(texture, pname, params) } }
pub unsafe fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) } else { gl::GetTextureSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) } }
pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar) -> ()>((*STORAGE).GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) } else { gl::GetTransformFeedbackVarying(program, index, bufSize, length, size, type_, name) } }
pub unsafe fn GetTransformFeedbacki64_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64) -> ()>((*STORAGE).GetTransformFeedbacki64_v.f)(xfb, pname, index, param) } else { gl::GetTransformFeedbacki64_v(xfb, pname, index, param) } }
pub unsafe fn GetTransformFeedbacki_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint) -> ()>((*STORAGE).GetTransformFeedbacki_v.f)(xfb, pname, index, param) } else { gl::GetTransformFeedbacki_v(xfb, pname, index, param) } }
pub unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetTransformFeedbackiv.f)(xfb, pname, param) } else { gl::GetTransformFeedbackiv(xfb, pname, param) } }
pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>((*STORAGE).GetUniformBlockIndex.f)(program, uniformBlockName) } else { gl::GetUniformBlockIndex(program, uniformBlockName) } }
pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> ()>((*STORAGE).GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) } else { gl::GetUniformIndices(program, uniformCount, uniformNames, uniformIndices) } }
pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>((*STORAGE).GetUniformLocation.f)(program, name) } else { gl::GetUniformLocation(program, name) } }
pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut GLuint) -> ()>((*STORAGE).GetUniformSubroutineuiv.f)(shadertype, location, params) } else { gl::GetUniformSubroutineuiv(shadertype, location, params) } }
pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLdouble) -> ()>((*STORAGE).GetUniformdv.f)(program, location, params) } else { gl::GetUniformdv(program, location, params) } }
pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>((*STORAGE).GetUniformfv.f)(program, location, params) } else { gl::GetUniformfv(program, location, params) } }
pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>((*STORAGE).GetUniformiv.f)(program, location, params) } else { gl::GetUniformiv(program, location, params) } }
pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>((*STORAGE).GetUniformuiv.f)(program, location, params) } else { gl::GetUniformuiv(program, location, params) } }
pub unsafe fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64) -> ()>((*STORAGE).GetVertexArrayIndexed64iv.f)(vaobj, index, pname, param) } else { gl::GetVertexArrayIndexed64iv(vaobj, index, pname, param) } }
pub unsafe fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetVertexArrayIndexediv.f)(vaobj, index, pname, param) } else { gl::GetVertexArrayIndexediv(vaobj, index, pname, param) } }
pub unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetVertexArrayiv.f)(vaobj, pname, param) } else { gl::GetVertexArrayiv(vaobj, pname, param) } }
pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetVertexAttribIiv.f)(index, pname, params) } else { gl::GetVertexAttribIiv(index, pname, params) } }
pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>((*STORAGE).GetVertexAttribIuiv.f)(index, pname, params) } else { gl::GetVertexAttribIuiv(index, pname, params) } }
pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>((*STORAGE).GetVertexAttribLdv.f)(index, pname, params) } else { gl::GetVertexAttribLdv(index, pname, params) } }
pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>((*STORAGE).GetVertexAttribPointerv.f)(index, pname, pointer) } else { gl::GetVertexAttribPointerv(index, pname, pointer) } }
pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>((*STORAGE).GetVertexAttribdv.f)(index, pname, params) } else { gl::GetVertexAttribdv(index, pname, params) } }
pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>((*STORAGE).GetVertexAttribfv.f)(index, pname, params) } else { gl::GetVertexAttribfv(index, pname, params) } }
pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>((*STORAGE).GetVertexAttribiv.f)(index, pname, params) } else { gl::GetVertexAttribiv(index, pname, params) } }
pub unsafe fn GetnColorTable(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnColorTable.f)(target, format, type_, bufSize, table) } else { gl::GetnColorTable(target, format, type_, bufSize, table) } }
pub unsafe fn GetnCompressedTexImage(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnCompressedTexImage.f)(target, lod, bufSize, pixels) } else { gl::GetnCompressedTexImage(target, lod, bufSize, pixels) } }
pub unsafe fn GetnConvolutionFilter(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnConvolutionFilter.f)(target, format, type_, bufSize, image) } else { gl::GetnConvolutionFilter(target, format, type_, bufSize, image) } }
pub unsafe fn GetnHistogram(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnHistogram.f)(target, reset, format, type_, bufSize, values) } else { gl::GetnHistogram(target, reset, format, type_, bufSize, values) } }
pub unsafe fn GetnMapdv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLdouble) -> ()>((*STORAGE).GetnMapdv.f)(target, query, bufSize, v) } else { gl::GetnMapdv(target, query, bufSize, v) } }
pub unsafe fn GetnMapfv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLfloat) -> ()>((*STORAGE).GetnMapfv.f)(target, query, bufSize, v) } else { gl::GetnMapfv(target, query, bufSize, v) } }
pub unsafe fn GetnMapiv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLint) -> ()>((*STORAGE).GetnMapiv.f)(target, query, bufSize, v) } else { gl::GetnMapiv(target, query, bufSize, v) } }
pub unsafe fn GetnMinmax(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnMinmax.f)(target, reset, format, type_, bufSize, values) } else { gl::GetnMinmax(target, reset, format, type_, bufSize, values) } }
pub unsafe fn GetnPixelMapfv(map: GLenum, bufSize: GLsizei, values: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLfloat) -> ()>((*STORAGE).GetnPixelMapfv.f)(map, bufSize, values) } else { gl::GetnPixelMapfv(map, bufSize, values) } }
pub unsafe fn GetnPixelMapuiv(map: GLenum, bufSize: GLsizei, values: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>((*STORAGE).GetnPixelMapuiv.f)(map, bufSize, values) } else { gl::GetnPixelMapuiv(map, bufSize, values) } }
pub unsafe fn GetnPixelMapusv(map: GLenum, bufSize: GLsizei, values: *mut GLushort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLushort) -> ()>((*STORAGE).GetnPixelMapusv.f)(map, bufSize, values) } else { gl::GetnPixelMapusv(map, bufSize, values) } }
pub unsafe fn GetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLubyte) -> ()>((*STORAGE).GetnPolygonStipple.f)(bufSize, pattern) } else { gl::GetnPolygonStipple(bufSize, pattern) } }
pub unsafe fn GetnSeparableFilter(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void, GLsizei, *mut c_void, *mut c_void) -> ()>((*STORAGE).GetnSeparableFilter.f)(target, format, type_, rowBufSize, row, columnBufSize, column, span) } else { gl::GetnSeparableFilter(target, format, type_, rowBufSize, row, columnBufSize, column, span) } }
pub unsafe fn GetnTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).GetnTexImage.f)(target, level, format, type_, bufSize, pixels) } else { gl::GetnTexImage(target, level, format, type_, bufSize, pixels) } }
pub unsafe fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble) -> ()>((*STORAGE).GetnUniformdv.f)(program, location, bufSize, params) } else { gl::GetnUniformdv(program, location, bufSize, params) } }
pub unsafe fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> ()>((*STORAGE).GetnUniformfv.f)(program, location, bufSize, params) } else { gl::GetnUniformfv(program, location, bufSize, params) } }
pub unsafe fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> ()>((*STORAGE).GetnUniformiv.f)(program, location, bufSize, params) } else { gl::GetnUniformiv(program, location, bufSize, params) } }
pub unsafe fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> ()>((*STORAGE).GetnUniformuiv.f)(program, location, bufSize, params) } else { gl::GetnUniformuiv(program, location, bufSize, params) } }
pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).Hint.f)(target, mode) } else { gl::Hint(target, mode) } }
pub unsafe fn InvalidateBufferData(buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).InvalidateBufferData.f)(buffer) } else { gl::InvalidateBufferData(buffer) } }
pub unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).InvalidateBufferSubData.f)(buffer, offset, length) } else { gl::InvalidateBufferSubData(buffer, offset, length) } }
pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>((*STORAGE).InvalidateFramebuffer.f)(target, numAttachments, attachments) } else { gl::InvalidateFramebuffer(target, numAttachments, attachments) } }
pub unsafe fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>((*STORAGE).InvalidateNamedFramebufferData.f)(framebuffer, numAttachments, attachments) } else { gl::InvalidateNamedFramebufferData(framebuffer, numAttachments, attachments) } }
pub unsafe fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).InvalidateNamedFramebufferSubData.f)(framebuffer, numAttachments, attachments, x, y, width, height) } else { gl::InvalidateNamedFramebufferSubData(framebuffer, numAttachments, attachments, x, y, width, height) } }
pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) } else { gl::InvalidateSubFramebuffer(target, numAttachments, attachments, x, y, width, height) } }
pub unsafe fn InvalidateTexImage(texture: GLuint, level: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>((*STORAGE).InvalidateTexImage.f)(texture, level) } else { gl::InvalidateTexImage(texture, level) } }
pub unsafe fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>((*STORAGE).InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) } else { gl::InvalidateTexSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth) } }
pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsBuffer.f)(buffer) } else { gl::IsBuffer(buffer) } }
pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>((*STORAGE).IsEnabled.f)(cap) } else { gl::IsEnabled(cap) } }
pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> GLboolean>((*STORAGE).IsEnabledi.f)(target, index) } else { gl::IsEnabledi(target, index) } }
pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsFramebuffer.f)(framebuffer) } else { gl::IsFramebuffer(framebuffer) } }
pub unsafe fn IsProgram(program: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsProgram.f)(program) } else { gl::IsProgram(program) } }
pub unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsProgramPipeline.f)(pipeline) } else { gl::IsProgramPipeline(pipeline) } }
pub unsafe fn IsQuery(id: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsQuery.f)(id) } else { gl::IsQuery(id) } }
pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsRenderbuffer.f)(renderbuffer) } else { gl::IsRenderbuffer(renderbuffer) } }
pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsSampler.f)(sampler) } else { gl::IsSampler(sampler) } }
pub unsafe fn IsShader(shader: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsShader.f)(shader) } else { gl::IsShader(shader) } }
pub unsafe fn IsSync(sync: GLsync) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>((*STORAGE).IsSync.f)(sync) } else { gl::IsSync(sync) } }
pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsTexture.f)(texture) } else { gl::IsTexture(texture) } }
pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsTransformFeedback.f)(id) } else { gl::IsTransformFeedback(id) } }
pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).IsVertexArray.f)(array) } else { gl::IsVertexArray(array) } }
pub unsafe fn LineWidth(width: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>((*STORAGE).LineWidth.f)(width) } else { gl::LineWidth(width) } }
pub unsafe fn LinkProgram(program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).LinkProgram.f)(program) } else { gl::LinkProgram(program) } }
pub unsafe fn LogicOp(opcode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).LogicOp.f)(opcode) } else { gl::LogicOp(opcode) } }
pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut c_void>((*STORAGE).MapBuffer.f)(target, access) } else { gl::MapBuffer(target, access) } }
pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void>((*STORAGE).MapBufferRange.f)(target, offset, length, access) } else { gl::MapBufferRange(target, offset, length, access) } }
pub unsafe fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut c_void { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> *mut c_void>((*STORAGE).MapNamedBuffer.f)(buffer, access) } else { gl::MapNamedBuffer(buffer, access) } }
pub unsafe fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void>((*STORAGE).MapNamedBufferRange.f)(buffer, offset, length, access) } else { gl::MapNamedBufferRange(buffer, offset, length, access) } }
pub unsafe fn MemoryBarrier(barriers: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>((*STORAGE).MemoryBarrier.f)(barriers) } else { gl::MemoryBarrier(barriers) } }
pub unsafe fn MemoryBarrierByRegion(barriers: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>((*STORAGE).MemoryBarrierByRegion.f)(barriers) } else { gl::MemoryBarrierByRegion(barriers) } }
pub unsafe fn MinSampleShading(value: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>((*STORAGE).MinSampleShading.f)(value) } else { gl::MinSampleShading(value) } }
pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>((*STORAGE).MultiDrawArrays.f)(mode, first, count, drawcount) } else { gl::MultiDrawArrays(mode, first, count, drawcount) } }
pub unsafe fn MultiDrawArraysIndirect(mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei) -> ()>((*STORAGE).MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) } else { gl::MultiDrawArraysIndirect(mode, indirect, drawcount, stride) } }
pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei) -> ()>((*STORAGE).MultiDrawElements.f)(mode, count, type_, indices, drawcount) } else { gl::MultiDrawElements(mode, count, type_, indices, drawcount) } }
pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint) -> ()>((*STORAGE).MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) } else { gl::MultiDrawElementsBaseVertex(mode, count, type_, indices, drawcount, basevertex) } }
pub unsafe fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei) -> ()>((*STORAGE).MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) } else { gl::MultiDrawElementsIndirect(mode, type_, indirect, drawcount, stride) } }
pub unsafe fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>((*STORAGE).MultiTexCoordP1ui.f)(texture, type_, coords) } else { gl::MultiTexCoordP1ui(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>((*STORAGE).MultiTexCoordP1uiv.f)(texture, type_, coords) } else { gl::MultiTexCoordP1uiv(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>((*STORAGE).MultiTexCoordP2ui.f)(texture, type_, coords) } else { gl::MultiTexCoordP2ui(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>((*STORAGE).MultiTexCoordP2uiv.f)(texture, type_, coords) } else { gl::MultiTexCoordP2uiv(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>((*STORAGE).MultiTexCoordP3ui.f)(texture, type_, coords) } else { gl::MultiTexCoordP3ui(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>((*STORAGE).MultiTexCoordP3uiv.f)(texture, type_, coords) } else { gl::MultiTexCoordP3uiv(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>((*STORAGE).MultiTexCoordP4ui.f)(texture, type_, coords) } else { gl::MultiTexCoordP4ui(texture, type_, coords) } }
pub unsafe fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>((*STORAGE).MultiTexCoordP4uiv.f)(texture, type_, coords) } else { gl::MultiTexCoordP4uiv(texture, type_, coords) } }
pub unsafe fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum) -> ()>((*STORAGE).NamedBufferData.f)(buffer, size, data, usage) } else { gl::NamedBufferData(buffer, size, data, usage) } }
pub unsafe fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield) -> ()>((*STORAGE).NamedBufferStorage.f)(buffer, size, data, flags) } else { gl::NamedBufferStorage(buffer, size, data, flags) } }
pub unsafe fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void) -> ()>((*STORAGE).NamedBufferSubData.f)(buffer, offset, size, data) } else { gl::NamedBufferSubData(buffer, offset, size, data) } }
pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>((*STORAGE).NamedFramebufferDrawBuffer.f)(framebuffer, buf) } else { gl::NamedFramebufferDrawBuffer(framebuffer, buf) } }
pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>((*STORAGE).NamedFramebufferDrawBuffers.f)(framebuffer, n, bufs) } else { gl::NamedFramebufferDrawBuffers(framebuffer, n, bufs) } }
pub unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>((*STORAGE).NamedFramebufferParameteri.f)(framebuffer, pname, param) } else { gl::NamedFramebufferParameteri(framebuffer, pname, param) } }
pub unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>((*STORAGE).NamedFramebufferReadBuffer.f)(framebuffer, src) } else { gl::NamedFramebufferReadBuffer(framebuffer, src) } }
pub unsafe fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLuint) -> ()>((*STORAGE).NamedFramebufferRenderbuffer.f)(framebuffer, attachment, renderbuffertarget, renderbuffer) } else { gl::NamedFramebufferRenderbuffer(framebuffer, attachment, renderbuffertarget, renderbuffer) } }
pub unsafe fn NamedFramebufferTexture(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint) -> ()>((*STORAGE).NamedFramebufferTexture.f)(framebuffer, attachment, texture, level) } else { gl::NamedFramebufferTexture(framebuffer, attachment, texture, level) } }
pub unsafe fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint) -> ()>((*STORAGE).NamedFramebufferTextureLayer.f)(framebuffer, attachment, texture, level, layer) } else { gl::NamedFramebufferTextureLayer(framebuffer, attachment, texture, level, layer) } }
pub unsafe fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).NamedRenderbufferStorage.f)(renderbuffer, internalformat, width, height) } else { gl::NamedRenderbufferStorage(renderbuffer, internalformat, width, height) } }
pub unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).NamedRenderbufferStorageMultisample.f)(renderbuffer, samples, internalformat, width, height) } else { gl::NamedRenderbufferStorageMultisample(renderbuffer, samples, internalformat, width, height) } }
pub unsafe fn NormalP3ui(type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).NormalP3ui.f)(type_, coords) } else { gl::NormalP3ui(type_, coords) } }
pub unsafe fn NormalP3uiv(type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).NormalP3uiv.f)(type_, coords) } else { gl::NormalP3uiv(type_, coords) } }
pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>((*STORAGE).ObjectLabel.f)(identifier, name, length, label) } else { gl::ObjectLabel(identifier, name, length, label) } }
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(*const c_void, GLsizei, *const GLchar) -> ()>((*STORAGE).ObjectPtrLabel.f)(ptr, length, label) } else { gl::ObjectPtrLabel(ptr, length, label) } }
pub unsafe fn PatchParameterfv(pname: GLenum, values: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>((*STORAGE).PatchParameterfv.f)(pname, values) } else { gl::PatchParameterfv(pname, values) } }
pub unsafe fn PatchParameteri(pname: GLenum, value: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>((*STORAGE).PatchParameteri.f)(pname, value) } else { gl::PatchParameteri(pname, value) } }
pub unsafe fn PauseTransformFeedback() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).PauseTransformFeedback.f)() } else { gl::PauseTransformFeedback() } }
pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>((*STORAGE).PixelStoref.f)(pname, param) } else { gl::PixelStoref(pname, param) } }
pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>((*STORAGE).PixelStorei.f)(pname, param) } else { gl::PixelStorei(pname, param) } }
pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>((*STORAGE).PointParameterf.f)(pname, param) } else { gl::PointParameterf(pname, param) } }
pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>((*STORAGE).PointParameterfv.f)(pname, params) } else { gl::PointParameterfv(pname, params) } }
pub unsafe fn PointParameteri(pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>((*STORAGE).PointParameteri.f)(pname, param) } else { gl::PointParameteri(pname, param) } }
pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>((*STORAGE).PointParameteriv.f)(pname, params) } else { gl::PointParameteriv(pname, params) } }
pub unsafe fn PointSize(size: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>((*STORAGE).PointSize.f)(size) } else { gl::PointSize(size) } }
pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>((*STORAGE).PolygonMode.f)(face, mode) } else { gl::PolygonMode(face, mode) } }
pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>((*STORAGE).PolygonOffset.f)(factor, units) } else { gl::PolygonOffset(factor, units) } }
pub unsafe fn PopDebugGroup() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).PopDebugGroup.f)() } else { gl::PopDebugGroup() } }
pub unsafe fn PrimitiveRestartIndex(index: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).PrimitiveRestartIndex.f)(index) } else { gl::PrimitiveRestartIndex(index) } }
pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const c_void, GLsizei) -> ()>((*STORAGE).ProgramBinary.f)(program, binaryFormat, binary, length) } else { gl::ProgramBinary(program, binaryFormat, binary, length) } }
pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>((*STORAGE).ProgramParameteri.f)(program, pname, value) } else { gl::ProgramParameteri(program, pname, value) } }
pub unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble) -> ()>((*STORAGE).ProgramUniform1d.f)(program, location, v0) } else { gl::ProgramUniform1d(program, location, v0) } }
pub unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).ProgramUniform1dv.f)(program, location, count, value) } else { gl::ProgramUniform1dv(program, location, count, value) } }
pub unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>((*STORAGE).ProgramUniform1f.f)(program, location, v0) } else { gl::ProgramUniform1f(program, location, v0) } }
pub unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).ProgramUniform1fv.f)(program, location, count, value) } else { gl::ProgramUniform1fv(program, location, count, value) } }
pub unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>((*STORAGE).ProgramUniform1i.f)(program, location, v0) } else { gl::ProgramUniform1i(program, location, v0) } }
pub unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>((*STORAGE).ProgramUniform1iv.f)(program, location, count, value) } else { gl::ProgramUniform1iv(program, location, count, value) } }
pub unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>((*STORAGE).ProgramUniform1ui.f)(program, location, v0) } else { gl::ProgramUniform1ui(program, location, v0) } }
pub unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).ProgramUniform1uiv.f)(program, location, count, value) } else { gl::ProgramUniform1uiv(program, location, count, value) } }
pub unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble) -> ()>((*STORAGE).ProgramUniform2d.f)(program, location, v0, v1) } else { gl::ProgramUniform2d(program, location, v0, v1) } }
pub unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).ProgramUniform2dv.f)(program, location, count, value) } else { gl::ProgramUniform2dv(program, location, count, value) } }
pub unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>((*STORAGE).ProgramUniform2f.f)(program, location, v0, v1) } else { gl::ProgramUniform2f(program, location, v0, v1) } }
pub unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).ProgramUniform2fv.f)(program, location, count, value) } else { gl::ProgramUniform2fv(program, location, count, value) } }
pub unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>((*STORAGE).ProgramUniform2i.f)(program, location, v0, v1) } else { gl::ProgramUniform2i(program, location, v0, v1) } }
pub unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>((*STORAGE).ProgramUniform2iv.f)(program, location, count, value) } else { gl::ProgramUniform2iv(program, location, count, value) } }
pub unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>((*STORAGE).ProgramUniform2ui.f)(program, location, v0, v1) } else { gl::ProgramUniform2ui(program, location, v0, v1) } }
pub unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).ProgramUniform2uiv.f)(program, location, count, value) } else { gl::ProgramUniform2uiv(program, location, count, value) } }
pub unsafe fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).ProgramUniform3d.f)(program, location, v0, v1, v2) } else { gl::ProgramUniform3d(program, location, v0, v1, v2) } }
pub unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).ProgramUniform3dv.f)(program, location, count, value) } else { gl::ProgramUniform3dv(program, location, count, value) } }
pub unsafe fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).ProgramUniform3f.f)(program, location, v0, v1, v2) } else { gl::ProgramUniform3f(program, location, v0, v1, v2) } }
pub unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).ProgramUniform3fv.f)(program, location, count, value) } else { gl::ProgramUniform3fv(program, location, count, value) } }
pub unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>((*STORAGE).ProgramUniform3i.f)(program, location, v0, v1, v2) } else { gl::ProgramUniform3i(program, location, v0, v1, v2) } }
pub unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>((*STORAGE).ProgramUniform3iv.f)(program, location, count, value) } else { gl::ProgramUniform3iv(program, location, count, value) } }
pub unsafe fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).ProgramUniform3ui.f)(program, location, v0, v1, v2) } else { gl::ProgramUniform3ui(program, location, v0, v1, v2) } }
pub unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).ProgramUniform3uiv.f)(program, location, count, value) } else { gl::ProgramUniform3uiv(program, location, count, value) } }
pub unsafe fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).ProgramUniform4d.f)(program, location, v0, v1, v2, v3) } else { gl::ProgramUniform4d(program, location, v0, v1, v2, v3) } }
pub unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).ProgramUniform4dv.f)(program, location, count, value) } else { gl::ProgramUniform4dv(program, location, count, value) } }
pub unsafe fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).ProgramUniform4f.f)(program, location, v0, v1, v2, v3) } else { gl::ProgramUniform4f(program, location, v0, v1, v2, v3) } }
pub unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).ProgramUniform4fv.f)(program, location, count, value) } else { gl::ProgramUniform4fv(program, location, count, value) } }
pub unsafe fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> ()>((*STORAGE).ProgramUniform4i.f)(program, location, v0, v1, v2, v3) } else { gl::ProgramUniform4i(program, location, v0, v1, v2, v3) } }
pub unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>((*STORAGE).ProgramUniform4iv.f)(program, location, count, value) } else { gl::ProgramUniform4iv(program, location, count, value) } }
pub unsafe fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) } else { gl::ProgramUniform4ui(program, location, v0, v1, v2, v3) } }
pub unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).ProgramUniform4uiv.f)(program, location, count, value) } else { gl::ProgramUniform4uiv(program, location, count, value) } }
pub unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2x3dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2x3fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2x4dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix2x4fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3x2dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3x2fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3x4dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix3x4fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4x2dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4x2fv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4x3dv(program, location, count, transpose, value) } }
pub unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) } else { gl::ProgramUniformMatrix4x3fv(program, location, count, transpose, value) } }
pub unsafe fn ProvokingVertex(mode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).ProvokingVertex.f)(mode) } else { gl::ProvokingVertex(mode) } }
pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>((*STORAGE).PushDebugGroup.f)(source, id, length, message) } else { gl::PushDebugGroup(source, id, length, message) } }
pub unsafe fn QueryCounter(id: GLuint, target: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>((*STORAGE).QueryCounter.f)(id, target) } else { gl::QueryCounter(id, target) } }
pub unsafe fn ReadBuffer(src: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> ()>((*STORAGE).ReadBuffer.f)(src) } else { gl::ReadBuffer(src) } }
pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void) -> ()>((*STORAGE).ReadPixels.f)(x, y, width, height, format, type_, pixels) } else { gl::ReadPixels(x, y, width, height, format, type_, pixels) } }
pub unsafe fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) -> ()>((*STORAGE).ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data) } else { gl::ReadnPixels(x, y, width, height, format, type_, bufSize, data) } }
pub unsafe fn ReleaseShaderCompiler() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).ReleaseShaderCompiler.f)() } else { gl::ReleaseShaderCompiler() } }
pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).RenderbufferStorage.f)(target, internalformat, width, height) } else { gl::RenderbufferStorage(target, internalformat, width, height) } }
pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) } else { gl::RenderbufferStorageMultisample(target, samples, internalformat, width, height) } }
pub unsafe fn ResumeTransformFeedback() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).ResumeTransformFeedback.f)() } else { gl::ResumeTransformFeedback() } }
pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>((*STORAGE).SampleCoverage.f)(value, invert) } else { gl::SampleCoverage(value, invert) } }
pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>((*STORAGE).SampleMaski.f)(maskNumber, mask) } else { gl::SampleMaski(maskNumber, mask) } }
pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>((*STORAGE).SamplerParameterIiv.f)(sampler, pname, param) } else { gl::SamplerParameterIiv(sampler, pname, param) } }
pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>((*STORAGE).SamplerParameterIuiv.f)(sampler, pname, param) } else { gl::SamplerParameterIuiv(sampler, pname, param) } }
pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>((*STORAGE).SamplerParameterf.f)(sampler, pname, param) } else { gl::SamplerParameterf(sampler, pname, param) } }
pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>((*STORAGE).SamplerParameterfv.f)(sampler, pname, param) } else { gl::SamplerParameterfv(sampler, pname, param) } }
pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>((*STORAGE).SamplerParameteri.f)(sampler, pname, param) } else { gl::SamplerParameteri(sampler, pname, param) } }
pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>((*STORAGE).SamplerParameteriv.f)(sampler, pname, param) } else { gl::SamplerParameteriv(sampler, pname, param) } }
pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).Scissor.f)(x, y, width, height) } else { gl::Scissor(x, y, width, height) } }
pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLint) -> ()>((*STORAGE).ScissorArrayv.f)(first, count, v) } else { gl::ScissorArrayv(first, count, v) } }
pub unsafe fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).ScissorIndexed.f)(index, left, bottom, width, height) } else { gl::ScissorIndexed(index, left, bottom, width, height) } }
pub unsafe fn ScissorIndexedv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).ScissorIndexedv.f)(index, v) } else { gl::ScissorIndexedv(index, v) } }
pub unsafe fn SecondaryColorP3ui(type_: GLenum, color: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).SecondaryColorP3ui.f)(type_, color) } else { gl::SecondaryColorP3ui(type_, color) } }
pub unsafe fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).SecondaryColorP3uiv.f)(type_, color) } else { gl::SecondaryColorP3uiv(type_, color) } }
pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const c_void, length: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei) -> ()>((*STORAGE).ShaderBinary.f)(count, shaders, binaryformat, binary, length) } else { gl::ShaderBinary(count, shaders, binaryformat, binary, length) } }
pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>((*STORAGE).ShaderSource.f)(shader, count, string, length) } else { gl::ShaderSource(shader, count, string, length) } }
pub unsafe fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) } else { gl::ShaderStorageBlockBinding(program, storageBlockIndex, storageBlockBinding) } }
pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>((*STORAGE).StencilFunc.f)(func, ref_, mask) } else { gl::StencilFunc(func, ref_, mask) } }
pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>((*STORAGE).StencilFuncSeparate.f)(face, func, ref_, mask) } else { gl::StencilFuncSeparate(face, func, ref_, mask) } }
pub unsafe fn StencilMask(mask: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).StencilMask.f)(mask) } else { gl::StencilMask(mask) } }
pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).StencilMaskSeparate.f)(face, mask) } else { gl::StencilMaskSeparate(face, mask) } }
pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>((*STORAGE).StencilOp.f)(fail, zfail, zpass) } else { gl::StencilOp(fail, zfail, zpass) } }
pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>((*STORAGE).StencilOpSeparate.f)(face, sfail, dpfail, dppass) } else { gl::StencilOpSeparate(face, sfail, dpfail, dppass) } }
pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>((*STORAGE).TexBuffer.f)(target, internalformat, buffer) } else { gl::TexBuffer(target, internalformat, buffer) } }
pub unsafe fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).TexBufferRange.f)(target, internalformat, buffer, offset, size) } else { gl::TexBufferRange(target, internalformat, buffer, offset, size) } }
pub unsafe fn TexCoordP1ui(type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).TexCoordP1ui.f)(type_, coords) } else { gl::TexCoordP1ui(type_, coords) } }
pub unsafe fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).TexCoordP1uiv.f)(type_, coords) } else { gl::TexCoordP1uiv(type_, coords) } }
pub unsafe fn TexCoordP2ui(type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).TexCoordP2ui.f)(type_, coords) } else { gl::TexCoordP2ui(type_, coords) } }
pub unsafe fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).TexCoordP2uiv.f)(type_, coords) } else { gl::TexCoordP2uiv(type_, coords) } }
pub unsafe fn TexCoordP3ui(type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).TexCoordP3ui.f)(type_, coords) } else { gl::TexCoordP3ui(type_, coords) } }
pub unsafe fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).TexCoordP3uiv.f)(type_, coords) } else { gl::TexCoordP3uiv(type_, coords) } }
pub unsafe fn TexCoordP4ui(type_: GLenum, coords: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).TexCoordP4ui.f)(type_, coords) } else { gl::TexCoordP4ui(type_, coords) } }
pub unsafe fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).TexCoordP4uiv.f)(type_, coords) } else { gl::TexCoordP4uiv(type_, coords) } }
pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) } else { gl::TexImage1D(target, level, internalformat, width, border, format, type_, pixels) } }
pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) } else { gl::TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels) } }
pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) } else { gl::TexImage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations) } }
pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) } else { gl::TexImage3D(target, level, internalformat, width, height, depth, border, format, type_, pixels) } }
pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) } else { gl::TexImage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations) } }
pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>((*STORAGE).TexParameterIiv.f)(target, pname, params) } else { gl::TexParameterIiv(target, pname, params) } }
pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>((*STORAGE).TexParameterIuiv.f)(target, pname, params) } else { gl::TexParameterIuiv(target, pname, params) } }
pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>((*STORAGE).TexParameterf.f)(target, pname, param) } else { gl::TexParameterf(target, pname, param) } }
pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>((*STORAGE).TexParameterfv.f)(target, pname, params) } else { gl::TexParameterfv(target, pname, params) } }
pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>((*STORAGE).TexParameteri.f)(target, pname, param) } else { gl::TexParameteri(target, pname, param) } }
pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>((*STORAGE).TexParameteriv.f)(target, pname, params) } else { gl::TexParameteriv(target, pname, params) } }
pub unsafe fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei) -> ()>((*STORAGE).TexStorage1D.f)(target, levels, internalformat, width) } else { gl::TexStorage1D(target, levels, internalformat, width) } }
pub unsafe fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).TexStorage2D.f)(target, levels, internalformat, width, height) } else { gl::TexStorage2D(target, levels, internalformat, width, height) } }
pub unsafe fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) } else { gl::TexStorage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations) } }
pub unsafe fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>((*STORAGE).TexStorage3D.f)(target, levels, internalformat, width, height, depth) } else { gl::TexStorage3D(target, levels, internalformat, width, height, depth) } }
pub unsafe fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) } else { gl::TexStorage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations) } }
pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) } else { gl::TexSubImage1D(target, level, xoffset, width, format, type_, pixels) } }
pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) } else { gl::TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels) } }
pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) } else { gl::TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) } }
pub unsafe fn TextureBarrier() -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn() -> ()>((*STORAGE).TextureBarrier.f)() } else { gl::TextureBarrier() } }
pub unsafe fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint) -> ()>((*STORAGE).TextureBuffer.f)(texture, internalformat, buffer) } else { gl::TextureBuffer(texture, internalformat, buffer) } }
pub unsafe fn TextureBufferRange(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).TextureBufferRange.f)(texture, internalformat, buffer, offset, size) } else { gl::TextureBufferRange(texture, internalformat, buffer, offset, size) } }
pub unsafe fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>((*STORAGE).TextureParameterIiv.f)(texture, pname, params) } else { gl::TextureParameterIiv(texture, pname, params) } }
pub unsafe fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>((*STORAGE).TextureParameterIuiv.f)(texture, pname, params) } else { gl::TextureParameterIuiv(texture, pname, params) } }
pub unsafe fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>((*STORAGE).TextureParameterf.f)(texture, pname, param) } else { gl::TextureParameterf(texture, pname, param) } }
pub unsafe fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>((*STORAGE).TextureParameterfv.f)(texture, pname, param) } else { gl::TextureParameterfv(texture, pname, param) } }
pub unsafe fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>((*STORAGE).TextureParameteri.f)(texture, pname, param) } else { gl::TextureParameteri(texture, pname, param) } }
pub unsafe fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>((*STORAGE).TextureParameteriv.f)(texture, pname, param) } else { gl::TextureParameteriv(texture, pname, param) } }
pub unsafe fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei) -> ()>((*STORAGE).TextureStorage1D.f)(texture, levels, internalformat, width) } else { gl::TextureStorage1D(texture, levels, internalformat, width) } }
pub unsafe fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>((*STORAGE).TextureStorage2D.f)(texture, levels, internalformat, width, height) } else { gl::TextureStorage2D(texture, levels, internalformat, width, height) } }
pub unsafe fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TextureStorage2DMultisample.f)(texture, samples, internalformat, width, height, fixedsamplelocations) } else { gl::TextureStorage2DMultisample(texture, samples, internalformat, width, height, fixedsamplelocations) } }
pub unsafe fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>((*STORAGE).TextureStorage3D.f)(texture, levels, internalformat, width, height, depth) } else { gl::TextureStorage3D(texture, levels, internalformat, width, height, depth) } }
pub unsafe fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>((*STORAGE).TextureStorage3DMultisample.f)(texture, samples, internalformat, width, height, depth, fixedsamplelocations) } else { gl::TextureStorage3DMultisample(texture, samples, internalformat, width, height, depth, fixedsamplelocations) } }
pub unsafe fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TextureSubImage1D.f)(texture, level, xoffset, width, format, type_, pixels) } else { gl::TextureSubImage1D(texture, level, xoffset, width, format, type_, pixels) } }
pub unsafe fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, type_, pixels) } else { gl::TextureSubImage2D(texture, level, xoffset, yoffset, width, height, format, type_, pixels) } }
pub unsafe fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>((*STORAGE).TextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) } else { gl::TextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) } }
pub unsafe fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) } else { gl::TextureView(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) } }
pub unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).TransformFeedbackBufferBase.f)(xfb, index, buffer) } else { gl::TransformFeedbackBufferBase(xfb, index, buffer) } }
pub unsafe fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>((*STORAGE).TransformFeedbackBufferRange.f)(xfb, index, buffer, offset, size) } else { gl::TransformFeedbackBufferRange(xfb, index, buffer, offset, size) } }
pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>((*STORAGE).TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) } else { gl::TransformFeedbackVaryings(program, count, varyings, bufferMode) } }
pub unsafe fn Uniform1d(location: GLint, x: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLdouble) -> ()>((*STORAGE).Uniform1d.f)(location, x) } else { gl::Uniform1d(location, x) } }
pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).Uniform1dv.f)(location, count, value) } else { gl::Uniform1dv(location, count, value) } }
pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>((*STORAGE).Uniform1f.f)(location, v0) } else { gl::Uniform1f(location, v0) } }
pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).Uniform1fv.f)(location, count, value) } else { gl::Uniform1fv(location, count, value) } }
pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>((*STORAGE).Uniform1i.f)(location, v0) } else { gl::Uniform1i(location, v0) } }
pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>((*STORAGE).Uniform1iv.f)(location, count, value) } else { gl::Uniform1iv(location, count, value) } }
pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>((*STORAGE).Uniform1ui.f)(location, v0) } else { gl::Uniform1ui(location, v0) } }
pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).Uniform1uiv.f)(location, count, value) } else { gl::Uniform1uiv(location, count, value) } }
pub unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>((*STORAGE).Uniform2d.f)(location, x, y) } else { gl::Uniform2d(location, x, y) } }
pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).Uniform2dv.f)(location, count, value) } else { gl::Uniform2dv(location, count, value) } }
pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>((*STORAGE).Uniform2f.f)(location, v0, v1) } else { gl::Uniform2f(location, v0, v1) } }
pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).Uniform2fv.f)(location, count, value) } else { gl::Uniform2fv(location, count, value) } }
pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>((*STORAGE).Uniform2i.f)(location, v0, v1) } else { gl::Uniform2i(location, v0, v1) } }
pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>((*STORAGE).Uniform2iv.f)(location, count, value) } else { gl::Uniform2iv(location, count, value) } }
pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>((*STORAGE).Uniform2ui.f)(location, v0, v1) } else { gl::Uniform2ui(location, v0, v1) } }
pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).Uniform2uiv.f)(location, count, value) } else { gl::Uniform2uiv(location, count, value) } }
pub unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).Uniform3d.f)(location, x, y, z) } else { gl::Uniform3d(location, x, y, z) } }
pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).Uniform3dv.f)(location, count, value) } else { gl::Uniform3dv(location, count, value) } }
pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).Uniform3f.f)(location, v0, v1, v2) } else { gl::Uniform3f(location, v0, v1, v2) } }
pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).Uniform3fv.f)(location, count, value) } else { gl::Uniform3fv(location, count, value) } }
pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>((*STORAGE).Uniform3i.f)(location, v0, v1, v2) } else { gl::Uniform3i(location, v0, v1, v2) } }
pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>((*STORAGE).Uniform3iv.f)(location, count, value) } else { gl::Uniform3iv(location, count, value) } }
pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).Uniform3ui.f)(location, v0, v1, v2) } else { gl::Uniform3ui(location, v0, v1, v2) } }
pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).Uniform3uiv.f)(location, count, value) } else { gl::Uniform3uiv(location, count, value) } }
pub unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).Uniform4d.f)(location, x, y, z, w) } else { gl::Uniform4d(location, x, y, z, w) } }
pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>((*STORAGE).Uniform4dv.f)(location, count, value) } else { gl::Uniform4dv(location, count, value) } }
pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).Uniform4f.f)(location, v0, v1, v2, v3) } else { gl::Uniform4f(location, v0, v1, v2, v3) } }
pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>((*STORAGE).Uniform4fv.f)(location, count, value) } else { gl::Uniform4fv(location, count, value) } }
pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>((*STORAGE).Uniform4i.f)(location, v0, v1, v2, v3) } else { gl::Uniform4i(location, v0, v1, v2, v3) } }
pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>((*STORAGE).Uniform4iv.f)(location, count, value) } else { gl::Uniform4iv(location, count, value) } }
pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).Uniform4ui.f)(location, v0, v1, v2, v3) } else { gl::Uniform4ui(location, v0, v1, v2, v3) } }
pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>((*STORAGE).Uniform4uiv.f)(location, count, value) } else { gl::Uniform4uiv(location, count, value) } }
pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) } else { gl::UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding) } }
pub unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix2dv.f)(location, count, transpose, value) } else { gl::UniformMatrix2dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix2fv.f)(location, count, transpose, value) } else { gl::UniformMatrix2fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix2x3dv.f)(location, count, transpose, value) } else { gl::UniformMatrix2x3dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix2x3fv.f)(location, count, transpose, value) } else { gl::UniformMatrix2x3fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix2x4dv.f)(location, count, transpose, value) } else { gl::UniformMatrix2x4dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix2x4fv.f)(location, count, transpose, value) } else { gl::UniformMatrix2x4fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix3dv.f)(location, count, transpose, value) } else { gl::UniformMatrix3dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix3fv.f)(location, count, transpose, value) } else { gl::UniformMatrix3fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix3x2dv.f)(location, count, transpose, value) } else { gl::UniformMatrix3x2dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix3x2fv.f)(location, count, transpose, value) } else { gl::UniformMatrix3x2fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix3x4dv.f)(location, count, transpose, value) } else { gl::UniformMatrix3x4dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix3x4fv.f)(location, count, transpose, value) } else { gl::UniformMatrix3x4fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix4dv.f)(location, count, transpose, value) } else { gl::UniformMatrix4dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix4fv.f)(location, count, transpose, value) } else { gl::UniformMatrix4fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix4x2dv.f)(location, count, transpose, value) } else { gl::UniformMatrix4x2dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix4x2fv.f)(location, count, transpose, value) } else { gl::UniformMatrix4x2fv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>((*STORAGE).UniformMatrix4x3dv.f)(location, count, transpose, value) } else { gl::UniformMatrix4x3dv(location, count, transpose, value) } }
pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>((*STORAGE).UniformMatrix4x3fv.f)(location, count, transpose, value) } else { gl::UniformMatrix4x3fv(location, count, transpose, value) } }
pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>((*STORAGE).UniformSubroutinesuiv.f)(shadertype, count, indices) } else { gl::UniformSubroutinesuiv(shadertype, count, indices) } }
pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>((*STORAGE).UnmapBuffer.f)(target) } else { gl::UnmapBuffer(target) } }
pub unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>((*STORAGE).UnmapNamedBuffer.f)(buffer) } else { gl::UnmapNamedBuffer(buffer) } }
pub unsafe fn UseProgram(program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).UseProgram.f)(program) } else { gl::UseProgram(program) } }
pub unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>((*STORAGE).UseProgramStages.f)(pipeline, stages, program) } else { gl::UseProgramStages(pipeline, stages, program) } }
pub unsafe fn ValidateProgram(program: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).ValidateProgram.f)(program) } else { gl::ValidateProgram(program) } }
pub unsafe fn ValidateProgramPipeline(pipeline: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint) -> ()>((*STORAGE).ValidateProgramPipeline.f)(pipeline) } else { gl::ValidateProgramPipeline(pipeline) } }
pub unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).VertexArrayAttribBinding.f)(vaobj, attribindex, bindingindex) } else { gl::VertexArrayAttribBinding(vaobj, attribindex, bindingindex) } }
pub unsafe fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexArrayAttribFormat.f)(vaobj, attribindex, size, type_, normalized, relativeoffset) } else { gl::VertexArrayAttribFormat(vaobj, attribindex, size, type_, normalized, relativeoffset) } }
pub unsafe fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>((*STORAGE).VertexArrayAttribIFormat.f)(vaobj, attribindex, size, type_, relativeoffset) } else { gl::VertexArrayAttribIFormat(vaobj, attribindex, size, type_, relativeoffset) } }
pub unsafe fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>((*STORAGE).VertexArrayAttribLFormat.f)(vaobj, attribindex, size, type_, relativeoffset) } else { gl::VertexArrayAttribLFormat(vaobj, attribindex, size, type_, relativeoffset) } }
pub unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).VertexArrayBindingDivisor.f)(vaobj, bindingindex, divisor) } else { gl::VertexArrayBindingDivisor(vaobj, bindingindex, divisor) } }
pub unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).VertexArrayElementBuffer.f)(vaobj, buffer) } else { gl::VertexArrayElementBuffer(vaobj, buffer) } }
pub unsafe fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei) -> ()>((*STORAGE).VertexArrayVertexBuffer.f)(vaobj, bindingindex, buffer, offset, stride) } else { gl::VertexArrayVertexBuffer(vaobj, bindingindex, buffer, offset, stride) } }
pub unsafe fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>((*STORAGE).VertexArrayVertexBuffers.f)(vaobj, first, count, buffers, offsets, strides) } else { gl::VertexArrayVertexBuffers(vaobj, first, count, buffers, offsets, strides) } }
pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>((*STORAGE).VertexAttrib1d.f)(index, x) } else { gl::VertexAttrib1d(index, x) } }
pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttrib1dv.f)(index, v) } else { gl::VertexAttrib1dv(index, v) } }
pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>((*STORAGE).VertexAttrib1f.f)(index, x) } else { gl::VertexAttrib1f(index, x) } }
pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>((*STORAGE).VertexAttrib1fv.f)(index, v) } else { gl::VertexAttrib1fv(index, v) } }
pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>((*STORAGE).VertexAttrib1s.f)(index, x) } else { gl::VertexAttrib1s(index, x) } }
pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttrib1sv.f)(index, v) } else { gl::VertexAttrib1sv(index, v) } }
pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttrib2d.f)(index, x, y) } else { gl::VertexAttrib2d(index, x, y) } }
pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttrib2dv.f)(index, v) } else { gl::VertexAttrib2dv(index, v) } }
pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>((*STORAGE).VertexAttrib2f.f)(index, x, y) } else { gl::VertexAttrib2f(index, x, y) } }
pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>((*STORAGE).VertexAttrib2fv.f)(index, v) } else { gl::VertexAttrib2fv(index, v) } }
pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>((*STORAGE).VertexAttrib2s.f)(index, x, y) } else { gl::VertexAttrib2s(index, x, y) } }
pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttrib2sv.f)(index, v) } else { gl::VertexAttrib2sv(index, v) } }
pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttrib3d.f)(index, x, y, z) } else { gl::VertexAttrib3d(index, x, y, z) } }
pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttrib3dv.f)(index, v) } else { gl::VertexAttrib3dv(index, v) } }
pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).VertexAttrib3f.f)(index, x, y, z) } else { gl::VertexAttrib3f(index, x, y, z) } }
pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>((*STORAGE).VertexAttrib3fv.f)(index, v) } else { gl::VertexAttrib3fv(index, v) } }
pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>((*STORAGE).VertexAttrib3s.f)(index, x, y, z) } else { gl::VertexAttrib3s(index, x, y, z) } }
pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttrib3sv.f)(index, v) } else { gl::VertexAttrib3sv(index, v) } }
pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>((*STORAGE).VertexAttrib4Nbv.f)(index, v) } else { gl::VertexAttrib4Nbv(index, v) } }
pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttrib4Niv.f)(index, v) } else { gl::VertexAttrib4Niv(index, v) } }
pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttrib4Nsv.f)(index, v) } else { gl::VertexAttrib4Nsv(index, v) } }
pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>((*STORAGE).VertexAttrib4Nub.f)(index, x, y, z, w) } else { gl::VertexAttrib4Nub(index, x, y, z, w) } }
pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>((*STORAGE).VertexAttrib4Nubv.f)(index, v) } else { gl::VertexAttrib4Nubv(index, v) } }
pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttrib4Nuiv.f)(index, v) } else { gl::VertexAttrib4Nuiv(index, v) } }
pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>((*STORAGE).VertexAttrib4Nusv.f)(index, v) } else { gl::VertexAttrib4Nusv(index, v) } }
pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>((*STORAGE).VertexAttrib4bv.f)(index, v) } else { gl::VertexAttrib4bv(index, v) } }
pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttrib4d.f)(index, x, y, z, w) } else { gl::VertexAttrib4d(index, x, y, z, w) } }
pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttrib4dv.f)(index, v) } else { gl::VertexAttrib4dv(index, v) } }
pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).VertexAttrib4f.f)(index, x, y, z, w) } else { gl::VertexAttrib4f(index, x, y, z, w) } }
pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>((*STORAGE).VertexAttrib4fv.f)(index, v) } else { gl::VertexAttrib4fv(index, v) } }
pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttrib4iv.f)(index, v) } else { gl::VertexAttrib4iv(index, v) } }
pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>((*STORAGE).VertexAttrib4s.f)(index, x, y, z, w) } else { gl::VertexAttrib4s(index, x, y, z, w) } }
pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttrib4sv.f)(index, v) } else { gl::VertexAttrib4sv(index, v) } }
pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>((*STORAGE).VertexAttrib4ubv.f)(index, v) } else { gl::VertexAttrib4ubv(index, v) } }
pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttrib4uiv.f)(index, v) } else { gl::VertexAttrib4uiv(index, v) } }
pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>((*STORAGE).VertexAttrib4usv.f)(index, v) } else { gl::VertexAttrib4usv(index, v) } }
pub unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).VertexAttribBinding.f)(attribindex, bindingindex) } else { gl::VertexAttribBinding(attribindex, bindingindex) } }
pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).VertexAttribDivisor.f)(index, divisor) } else { gl::VertexAttribDivisor(index, divisor) } }
pub unsafe fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) } else { gl::VertexAttribFormat(attribindex, size, type_, normalized, relativeoffset) } }
pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>((*STORAGE).VertexAttribI1i.f)(index, x) } else { gl::VertexAttribI1i(index, x) } }
pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttribI1iv.f)(index, v) } else { gl::VertexAttribI1iv(index, v) } }
pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).VertexAttribI1ui.f)(index, x) } else { gl::VertexAttribI1ui(index, x) } }
pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttribI1uiv.f)(index, v) } else { gl::VertexAttribI1uiv(index, v) } }
pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>((*STORAGE).VertexAttribI2i.f)(index, x, y) } else { gl::VertexAttribI2i(index, x, y) } }
pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttribI2iv.f)(index, v) } else { gl::VertexAttribI2iv(index, v) } }
pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>((*STORAGE).VertexAttribI2ui.f)(index, x, y) } else { gl::VertexAttribI2ui(index, x, y) } }
pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttribI2uiv.f)(index, v) } else { gl::VertexAttribI2uiv(index, v) } }
pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>((*STORAGE).VertexAttribI3i.f)(index, x, y, z) } else { gl::VertexAttribI3i(index, x, y, z) } }
pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttribI3iv.f)(index, v) } else { gl::VertexAttribI3iv(index, v) } }
pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).VertexAttribI3ui.f)(index, x, y, z) } else { gl::VertexAttribI3ui(index, x, y, z) } }
pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttribI3uiv.f)(index, v) } else { gl::VertexAttribI3uiv(index, v) } }
pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>((*STORAGE).VertexAttribI4bv.f)(index, v) } else { gl::VertexAttribI4bv(index, v) } }
pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>((*STORAGE).VertexAttribI4i.f)(index, x, y, z, w) } else { gl::VertexAttribI4i(index, x, y, z, w) } }
pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>((*STORAGE).VertexAttribI4iv.f)(index, v) } else { gl::VertexAttribI4iv(index, v) } }
pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>((*STORAGE).VertexAttribI4sv.f)(index, v) } else { gl::VertexAttribI4sv(index, v) } }
pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>((*STORAGE).VertexAttribI4ubv.f)(index, v) } else { gl::VertexAttribI4ubv(index, v) } }
pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>((*STORAGE).VertexAttribI4ui.f)(index, x, y, z, w) } else { gl::VertexAttribI4ui(index, x, y, z, w) } }
pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>((*STORAGE).VertexAttribI4uiv.f)(index, v) } else { gl::VertexAttribI4uiv(index, v) } }
pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>((*STORAGE).VertexAttribI4usv.f)(index, v) } else { gl::VertexAttribI4usv(index, v) } }
pub unsafe fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>((*STORAGE).VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) } else { gl::VertexAttribIFormat(attribindex, size, type_, relativeoffset) } }
pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).VertexAttribIPointer.f)(index, size, type_, stride, pointer) } else { gl::VertexAttribIPointer(index, size, type_, stride, pointer) } }
pub unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>((*STORAGE).VertexAttribL1d.f)(index, x) } else { gl::VertexAttribL1d(index, x) } }
pub unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttribL1dv.f)(index, v) } else { gl::VertexAttribL1dv(index, v) } }
pub unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttribL2d.f)(index, x, y) } else { gl::VertexAttribL2d(index, x, y) } }
pub unsafe fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttribL2dv.f)(index, v) } else { gl::VertexAttribL2dv(index, v) } }
pub unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttribL3d.f)(index, x, y, z) } else { gl::VertexAttribL3d(index, x, y, z) } }
pub unsafe fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttribL3dv.f)(index, v) } else { gl::VertexAttribL3dv(index, v) } }
pub unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>((*STORAGE).VertexAttribL4d.f)(index, x, y, z, w) } else { gl::VertexAttribL4d(index, x, y, z, w) } }
pub unsafe fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>((*STORAGE).VertexAttribL4dv.f)(index, v) } else { gl::VertexAttribL4dv(index, v) } }
pub unsafe fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>((*STORAGE).VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) } else { gl::VertexAttribLFormat(attribindex, size, type_, relativeoffset) } }
pub unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>((*STORAGE).VertexAttribLPointer.f)(index, size, type_, stride, pointer) } else { gl::VertexAttribLPointer(index, size, type_, stride, pointer) } }
pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexAttribP1ui.f)(index, type_, normalized, value) } else { gl::VertexAttribP1ui(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>((*STORAGE).VertexAttribP1uiv.f)(index, type_, normalized, value) } else { gl::VertexAttribP1uiv(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexAttribP2ui.f)(index, type_, normalized, value) } else { gl::VertexAttribP2ui(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>((*STORAGE).VertexAttribP2uiv.f)(index, type_, normalized, value) } else { gl::VertexAttribP2uiv(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexAttribP3ui.f)(index, type_, normalized, value) } else { gl::VertexAttribP3ui(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>((*STORAGE).VertexAttribP3uiv.f)(index, type_, normalized, value) } else { gl::VertexAttribP3uiv(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>((*STORAGE).VertexAttribP4ui.f)(index, type_, normalized, value) } else { gl::VertexAttribP4ui(index, type_, normalized, value) } }
pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>((*STORAGE).VertexAttribP4uiv.f)(index, type_, normalized, value) } else { gl::VertexAttribP4uiv(index, type_, normalized, value) } }
pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) -> ()>((*STORAGE).VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) } else { gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer) } }
pub unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>((*STORAGE).VertexBindingDivisor.f)(bindingindex, divisor) } else { gl::VertexBindingDivisor(bindingindex, divisor) } }
pub unsafe fn VertexP2ui(type_: GLenum, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).VertexP2ui.f)(type_, value) } else { gl::VertexP2ui(type_, value) } }
pub unsafe fn VertexP2uiv(type_: GLenum, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).VertexP2uiv.f)(type_, value) } else { gl::VertexP2uiv(type_, value) } }
pub unsafe fn VertexP3ui(type_: GLenum, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).VertexP3ui.f)(type_, value) } else { gl::VertexP3ui(type_, value) } }
pub unsafe fn VertexP3uiv(type_: GLenum, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).VertexP3uiv.f)(type_, value) } else { gl::VertexP3uiv(type_, value) } }
pub unsafe fn VertexP4ui(type_: GLenum, value: GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>((*STORAGE).VertexP4ui.f)(type_, value) } else { gl::VertexP4ui(type_, value) } }
pub unsafe fn VertexP4uiv(type_: GLenum, value: *const GLuint) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>((*STORAGE).VertexP4uiv.f)(type_, value) } else { gl::VertexP4uiv(type_, value) } }
pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>((*STORAGE).Viewport.f)(x, y, width, height) } else { gl::Viewport(x, y, width, height) } }
pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLfloat) -> ()>((*STORAGE).ViewportArrayv.f)(first, count, v) } else { gl::ViewportArrayv(first, count, v) } }
pub unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>((*STORAGE).ViewportIndexedf.f)(index, x, y, w, h) } else { gl::ViewportIndexedf(index, x, y, w, h) } }
pub unsafe fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>((*STORAGE).ViewportIndexedfv.f)(index, v) } else { gl::ViewportIndexedfv(index, v) } }
pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> () { if EXTERNAL_POINTERS { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>((*STORAGE).WaitSync.f)(sync, flags, timeout) } else { gl::WaitSync(sync, flags, timeout) } }

pub mod ActiveShaderProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ActiveShaderProgram.is_loaded }
        else { gl::ActiveShaderProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ActiveShaderProgram = FnPtr::new(metaloadfn(&mut loadfn, "glActiveShaderProgram", &[]))
    } }
}
pub mod ActiveTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ActiveTexture.is_loaded }
        else { gl::ActiveTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ActiveTexture = FnPtr::new(metaloadfn(&mut loadfn, "glActiveTexture", &["glActiveTextureARB"]))
    } }
}
pub mod AttachShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).AttachShader.is_loaded }
        else { gl::AttachShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).AttachShader = FnPtr::new(metaloadfn(&mut loadfn, "glAttachShader", &["glAttachObjectARB"]))
    } }
}
pub mod BeginConditionalRender {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BeginConditionalRender.is_loaded }
        else { gl::BeginConditionalRender::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BeginConditionalRender = FnPtr::new(metaloadfn(&mut loadfn, "glBeginConditionalRender", &["glBeginConditionalRenderNV"]))
    } }
}
pub mod BeginQuery {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BeginQuery.is_loaded }
        else { gl::BeginQuery::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BeginQuery = FnPtr::new(metaloadfn(&mut loadfn, "glBeginQuery", &["glBeginQueryARB"]))
    } }
}
pub mod BeginQueryIndexed {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BeginQueryIndexed.is_loaded }
        else { gl::BeginQueryIndexed::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BeginQueryIndexed = FnPtr::new(metaloadfn(&mut loadfn, "glBeginQueryIndexed", &[]))
    } }
}
pub mod BeginTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BeginTransformFeedback.is_loaded }
        else { gl::BeginTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BeginTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glBeginTransformFeedback", &["glBeginTransformFeedbackEXT", "glBeginTransformFeedbackNV"]))
    } }
}
pub mod BindAttribLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindAttribLocation.is_loaded }
        else { gl::BindAttribLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, "glBindAttribLocation", &["glBindAttribLocationARB"]))
    } }
}
pub mod BindBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindBuffer.is_loaded }
        else { gl::BindBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindBuffer", &["glBindBufferARB"]))
    } }
}
pub mod BindBufferBase {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindBufferBase.is_loaded }
        else { gl::BindBufferBase::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindBufferBase = FnPtr::new(metaloadfn(&mut loadfn, "glBindBufferBase", &["glBindBufferBaseEXT", "glBindBufferBaseNV"]))
    } }
}
pub mod BindBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindBufferRange.is_loaded }
        else { gl::BindBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glBindBufferRange", &["glBindBufferRangeEXT", "glBindBufferRangeNV"]))
    } }
}
pub mod BindBuffersBase {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindBuffersBase.is_loaded }
        else { gl::BindBuffersBase::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindBuffersBase = FnPtr::new(metaloadfn(&mut loadfn, "glBindBuffersBase", &[]))
    } }
}
pub mod BindBuffersRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindBuffersRange.is_loaded }
        else { gl::BindBuffersRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindBuffersRange = FnPtr::new(metaloadfn(&mut loadfn, "glBindBuffersRange", &[]))
    } }
}
pub mod BindFragDataLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindFragDataLocation.is_loaded }
        else { gl::BindFragDataLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindFragDataLocation = FnPtr::new(metaloadfn(&mut loadfn, "glBindFragDataLocation", &["glBindFragDataLocationEXT"]))
    } }
}
pub mod BindFragDataLocationIndexed {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindFragDataLocationIndexed.is_loaded }
        else { gl::BindFragDataLocationIndexed::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindFragDataLocationIndexed = FnPtr::new(metaloadfn(&mut loadfn, "glBindFragDataLocationIndexed", &["glBindFragDataLocationIndexedEXT"]))
    } }
}
pub mod BindFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindFramebuffer.is_loaded }
        else { gl::BindFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindFramebuffer", &[]))
    } }
}
pub mod BindImageTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindImageTexture.is_loaded }
        else { gl::BindImageTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindImageTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindImageTexture", &[]))
    } }
}
pub mod BindImageTextures {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindImageTextures.is_loaded }
        else { gl::BindImageTextures::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindImageTextures = FnPtr::new(metaloadfn(&mut loadfn, "glBindImageTextures", &[]))
    } }
}
pub mod BindProgramPipeline {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindProgramPipeline.is_loaded }
        else { gl::BindProgramPipeline::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glBindProgramPipeline", &[]))
    } }
}
pub mod BindRenderbuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindRenderbuffer.is_loaded }
        else { gl::BindRenderbuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindRenderbuffer", &[]))
    } }
}
pub mod BindSampler {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindSampler.is_loaded }
        else { gl::BindSampler::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindSampler = FnPtr::new(metaloadfn(&mut loadfn, "glBindSampler", &[]))
    } }
}
pub mod BindSamplers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindSamplers.is_loaded }
        else { gl::BindSamplers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glBindSamplers", &[]))
    } }
}
pub mod BindTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindTexture.is_loaded }
        else { gl::BindTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindTexture", &["glBindTextureEXT"]))
    } }
}
pub mod BindTextureUnit {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindTextureUnit.is_loaded }
        else { gl::BindTextureUnit::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindTextureUnit = FnPtr::new(metaloadfn(&mut loadfn, "glBindTextureUnit", &[]))
    } }
}
pub mod BindTextures {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindTextures.is_loaded }
        else { gl::BindTextures::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindTextures = FnPtr::new(metaloadfn(&mut loadfn, "glBindTextures", &[]))
    } }
}
pub mod BindTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindTransformFeedback.is_loaded }
        else { gl::BindTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glBindTransformFeedback", &[]))
    } }
}
pub mod BindVertexArray {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindVertexArray.is_loaded }
        else { gl::BindVertexArray::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindVertexArray = FnPtr::new(metaloadfn(&mut loadfn, "glBindVertexArray", &["glBindVertexArrayOES"]))
    } }
}
pub mod BindVertexBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindVertexBuffer.is_loaded }
        else { gl::BindVertexBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindVertexBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindVertexBuffer", &[]))
    } }
}
pub mod BindVertexBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BindVertexBuffers.is_loaded }
        else { gl::BindVertexBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BindVertexBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glBindVertexBuffers", &[]))
    } }
}
pub mod BlendColor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendColor.is_loaded }
        else { gl::BlendColor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendColor = FnPtr::new(metaloadfn(&mut loadfn, "glBlendColor", &["glBlendColorEXT"]))
    } }
}
pub mod BlendEquation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendEquation.is_loaded }
        else { gl::BlendEquation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendEquation = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquation", &["glBlendEquationEXT"]))
    } }
}
pub mod BlendEquationSeparate {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendEquationSeparate.is_loaded }
        else { gl::BlendEquationSeparate::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendEquationSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationSeparate", &["glBlendEquationSeparateEXT"]))
    } }
}
pub mod BlendEquationSeparatei {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendEquationSeparatei.is_loaded }
        else { gl::BlendEquationSeparatei::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendEquationSeparatei = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationSeparatei", &["glBlendEquationSeparateIndexedAMD", "glBlendEquationSeparateiARB", "glBlendEquationSeparateiEXT", "glBlendEquationSeparateiOES"]))
    } }
}
pub mod BlendEquationi {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendEquationi.is_loaded }
        else { gl::BlendEquationi::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendEquationi = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationi", &["glBlendEquationIndexedAMD", "glBlendEquationiARB", "glBlendEquationiEXT", "glBlendEquationiOES"]))
    } }
}
pub mod BlendFunc {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendFunc.is_loaded }
        else { gl::BlendFunc::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendFunc = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunc", &[]))
    } }
}
pub mod BlendFuncSeparate {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendFuncSeparate.is_loaded }
        else { gl::BlendFuncSeparate::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFuncSeparate", &["glBlendFuncSeparateEXT", "glBlendFuncSeparateINGR"]))
    } }
}
pub mod BlendFuncSeparatei {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendFuncSeparatei.is_loaded }
        else { gl::BlendFuncSeparatei::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendFuncSeparatei = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFuncSeparatei", &["glBlendFuncSeparateIndexedAMD", "glBlendFuncSeparateiARB", "glBlendFuncSeparateiEXT", "glBlendFuncSeparateiOES"]))
    } }
}
pub mod BlendFunci {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlendFunci.is_loaded }
        else { gl::BlendFunci::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlendFunci = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunci", &["glBlendFuncIndexedAMD", "glBlendFunciARB", "glBlendFunciEXT", "glBlendFunciOES"]))
    } }
}
pub mod BlitFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlitFramebuffer.is_loaded }
        else { gl::BlitFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlitFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBlitFramebuffer", &["glBlitFramebufferEXT", "glBlitFramebufferNV"]))
    } }
}
pub mod BlitNamedFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BlitNamedFramebuffer.is_loaded }
        else { gl::BlitNamedFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BlitNamedFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBlitNamedFramebuffer", &[]))
    } }
}
pub mod BufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BufferData.is_loaded }
        else { gl::BufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BufferData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferData", &["glBufferDataARB"]))
    } }
}
pub mod BufferStorage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BufferStorage.is_loaded }
        else { gl::BufferStorage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BufferStorage = FnPtr::new(metaloadfn(&mut loadfn, "glBufferStorage", &["glBufferStorageEXT"]))
    } }
}
pub mod BufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).BufferSubData.is_loaded }
        else { gl::BufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).BufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferSubData", &["glBufferSubDataARB"]))
    } }
}
pub mod CheckFramebufferStatus {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CheckFramebufferStatus.is_loaded }
        else { gl::CheckFramebufferStatus::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CheckFramebufferStatus = FnPtr::new(metaloadfn(&mut loadfn, "glCheckFramebufferStatus", &["glCheckFramebufferStatusEXT"]))
    } }
}
pub mod CheckNamedFramebufferStatus {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CheckNamedFramebufferStatus.is_loaded }
        else { gl::CheckNamedFramebufferStatus::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CheckNamedFramebufferStatus = FnPtr::new(metaloadfn(&mut loadfn, "glCheckNamedFramebufferStatus", &[]))
    } }
}
pub mod ClampColor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClampColor.is_loaded }
        else { gl::ClampColor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClampColor = FnPtr::new(metaloadfn(&mut loadfn, "glClampColor", &["glClampColorARB"]))
    } }
}
pub mod Clear {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Clear.is_loaded }
        else { gl::Clear::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Clear = FnPtr::new(metaloadfn(&mut loadfn, "glClear", &[]))
    } }
}
pub mod ClearBufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferData.is_loaded }
        else { gl::ClearBufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferData = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferData", &[]))
    } }
}
pub mod ClearBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferSubData.is_loaded }
        else { gl::ClearBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferSubData", &[]))
    } }
}
pub mod ClearBufferfi {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferfi.is_loaded }
        else { gl::ClearBufferfi::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferfi = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferfi", &[]))
    } }
}
pub mod ClearBufferfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferfv.is_loaded }
        else { gl::ClearBufferfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferfv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferfv", &[]))
    } }
}
pub mod ClearBufferiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferiv.is_loaded }
        else { gl::ClearBufferiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferiv", &[]))
    } }
}
pub mod ClearBufferuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearBufferuiv.is_loaded }
        else { gl::ClearBufferuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearBufferuiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearBufferuiv", &[]))
    } }
}
pub mod ClearColor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearColor.is_loaded }
        else { gl::ClearColor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearColor = FnPtr::new(metaloadfn(&mut loadfn, "glClearColor", &[]))
    } }
}
pub mod ClearDepth {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearDepth.is_loaded }
        else { gl::ClearDepth::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearDepth = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepth", &[]))
    } }
}
pub mod ClearDepthf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearDepthf.is_loaded }
        else { gl::ClearDepthf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearDepthf = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepthf", &["glClearDepthfOES"]))
    } }
}
pub mod ClearNamedBufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedBufferData.is_loaded }
        else { gl::ClearNamedBufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedBufferData = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedBufferData", &[]))
    } }
}
pub mod ClearNamedBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedBufferSubData.is_loaded }
        else { gl::ClearNamedBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedBufferSubData", &[]))
    } }
}
pub mod ClearNamedFramebufferfi {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedFramebufferfi.is_loaded }
        else { gl::ClearNamedFramebufferfi::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedFramebufferfi = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedFramebufferfi", &[]))
    } }
}
pub mod ClearNamedFramebufferfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedFramebufferfv.is_loaded }
        else { gl::ClearNamedFramebufferfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedFramebufferfv = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedFramebufferfv", &[]))
    } }
}
pub mod ClearNamedFramebufferiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedFramebufferiv.is_loaded }
        else { gl::ClearNamedFramebufferiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedFramebufferiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedFramebufferiv", &[]))
    } }
}
pub mod ClearNamedFramebufferuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearNamedFramebufferuiv.is_loaded }
        else { gl::ClearNamedFramebufferuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearNamedFramebufferuiv = FnPtr::new(metaloadfn(&mut loadfn, "glClearNamedFramebufferuiv", &[]))
    } }
}
pub mod ClearStencil {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearStencil.is_loaded }
        else { gl::ClearStencil::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearStencil = FnPtr::new(metaloadfn(&mut loadfn, "glClearStencil", &[]))
    } }
}
pub mod ClearTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearTexImage.is_loaded }
        else { gl::ClearTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glClearTexImage", &["glClearTexImageEXT"]))
    } }
}
pub mod ClearTexSubImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClearTexSubImage.is_loaded }
        else { gl::ClearTexSubImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClearTexSubImage = FnPtr::new(metaloadfn(&mut loadfn, "glClearTexSubImage", &["glClearTexSubImageEXT"]))
    } }
}
pub mod ClientWaitSync {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClientWaitSync.is_loaded }
        else { gl::ClientWaitSync::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClientWaitSync = FnPtr::new(metaloadfn(&mut loadfn, "glClientWaitSync", &["glClientWaitSyncAPPLE"]))
    } }
}
pub mod ClipControl {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ClipControl.is_loaded }
        else { gl::ClipControl::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ClipControl = FnPtr::new(metaloadfn(&mut loadfn, "glClipControl", &["glClipControlEXT"]))
    } }
}
pub mod ColorMask {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorMask.is_loaded }
        else { gl::ColorMask::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorMask = FnPtr::new(metaloadfn(&mut loadfn, "glColorMask", &[]))
    } }
}
pub mod ColorMaski {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorMaski.is_loaded }
        else { gl::ColorMaski::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorMaski = FnPtr::new(metaloadfn(&mut loadfn, "glColorMaski", &["glColorMaskIndexedEXT", "glColorMaskiEXT", "glColorMaskiOES"]))
    } }
}
pub mod ColorP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorP3ui.is_loaded }
        else { gl::ColorP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glColorP3ui", &[]))
    } }
}
pub mod ColorP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorP3uiv.is_loaded }
        else { gl::ColorP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glColorP3uiv", &[]))
    } }
}
pub mod ColorP4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorP4ui.is_loaded }
        else { gl::ColorP4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorP4ui = FnPtr::new(metaloadfn(&mut loadfn, "glColorP4ui", &[]))
    } }
}
pub mod ColorP4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ColorP4uiv.is_loaded }
        else { gl::ColorP4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ColorP4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glColorP4uiv", &[]))
    } }
}
pub mod CompileShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompileShader.is_loaded }
        else { gl::CompileShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompileShader = FnPtr::new(metaloadfn(&mut loadfn, "glCompileShader", &["glCompileShaderARB"]))
    } }
}
pub mod CompressedTexImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexImage1D.is_loaded }
        else { gl::CompressedTexImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage1D", &["glCompressedTexImage1DARB"]))
    } }
}
pub mod CompressedTexImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexImage2D.is_loaded }
        else { gl::CompressedTexImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage2D", &["glCompressedTexImage2DARB"]))
    } }
}
pub mod CompressedTexImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexImage3D.is_loaded }
        else { gl::CompressedTexImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage3D", &["glCompressedTexImage3DARB"]))
    } }
}
pub mod CompressedTexSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexSubImage1D.is_loaded }
        else { gl::CompressedTexSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage1D", &["glCompressedTexSubImage1DARB"]))
    } }
}
pub mod CompressedTexSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexSubImage2D.is_loaded }
        else { gl::CompressedTexSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage2D", &["glCompressedTexSubImage2DARB"]))
    } }
}
pub mod CompressedTexSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTexSubImage3D.is_loaded }
        else { gl::CompressedTexSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage3D", &["glCompressedTexSubImage3DARB"]))
    } }
}
pub mod CompressedTextureSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTextureSubImage1D.is_loaded }
        else { gl::CompressedTextureSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTextureSubImage1D", &[]))
    } }
}
pub mod CompressedTextureSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTextureSubImage2D.is_loaded }
        else { gl::CompressedTextureSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTextureSubImage2D", &[]))
    } }
}
pub mod CompressedTextureSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CompressedTextureSubImage3D.is_loaded }
        else { gl::CompressedTextureSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CompressedTextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTextureSubImage3D", &[]))
    } }
}
pub mod CopyBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyBufferSubData.is_loaded }
        else { gl::CopyBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glCopyBufferSubData", &["glCopyBufferSubDataNV"]))
    } }
}
pub mod CopyImageSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyImageSubData.is_loaded }
        else { gl::CopyImageSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyImageSubData = FnPtr::new(metaloadfn(&mut loadfn, "glCopyImageSubData", &["glCopyImageSubDataEXT", "glCopyImageSubDataOES"]))
    } }
}
pub mod CopyNamedBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyNamedBufferSubData.is_loaded }
        else { gl::CopyNamedBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glCopyNamedBufferSubData", &[]))
    } }
}
pub mod CopyTexImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTexImage1D.is_loaded }
        else { gl::CopyTexImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTexImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage1D", &["glCopyTexImage1DEXT"]))
    } }
}
pub mod CopyTexImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTexImage2D.is_loaded }
        else { gl::CopyTexImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage2D", &["glCopyTexImage2DEXT"]))
    } }
}
pub mod CopyTexSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTexSubImage1D.is_loaded }
        else { gl::CopyTexSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage1D", &["glCopyTexSubImage1DEXT"]))
    } }
}
pub mod CopyTexSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTexSubImage2D.is_loaded }
        else { gl::CopyTexSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"]))
    } }
}
pub mod CopyTexSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTexSubImage3D.is_loaded }
        else { gl::CopyTexSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage3D", &["glCopyTexSubImage3DEXT"]))
    } }
}
pub mod CopyTextureSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTextureSubImage1D.is_loaded }
        else { gl::CopyTextureSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTextureSubImage1D", &[]))
    } }
}
pub mod CopyTextureSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTextureSubImage2D.is_loaded }
        else { gl::CopyTextureSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTextureSubImage2D", &[]))
    } }
}
pub mod CopyTextureSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CopyTextureSubImage3D.is_loaded }
        else { gl::CopyTextureSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CopyTextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTextureSubImage3D", &[]))
    } }
}
pub mod CreateBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateBuffers.is_loaded }
        else { gl::CreateBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glCreateBuffers", &[]))
    } }
}
pub mod CreateFramebuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateFramebuffers.is_loaded }
        else { gl::CreateFramebuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, "glCreateFramebuffers", &[]))
    } }
}
pub mod CreateProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateProgram.is_loaded }
        else { gl::CreateProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateProgram = FnPtr::new(metaloadfn(&mut loadfn, "glCreateProgram", &["glCreateProgramObjectARB"]))
    } }
}
pub mod CreateProgramPipelines {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateProgramPipelines.is_loaded }
        else { gl::CreateProgramPipelines::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, "glCreateProgramPipelines", &[]))
    } }
}
pub mod CreateQueries {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateQueries.is_loaded }
        else { gl::CreateQueries::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateQueries = FnPtr::new(metaloadfn(&mut loadfn, "glCreateQueries", &[]))
    } }
}
pub mod CreateRenderbuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateRenderbuffers.is_loaded }
        else { gl::CreateRenderbuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, "glCreateRenderbuffers", &[]))
    } }
}
pub mod CreateSamplers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateSamplers.is_loaded }
        else { gl::CreateSamplers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glCreateSamplers", &[]))
    } }
}
pub mod CreateShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateShader.is_loaded }
        else { gl::CreateShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateShader = FnPtr::new(metaloadfn(&mut loadfn, "glCreateShader", &["glCreateShaderObjectARB"]))
    } }
}
pub mod CreateShaderProgramv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateShaderProgramv.is_loaded }
        else { gl::CreateShaderProgramv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateShaderProgramv = FnPtr::new(metaloadfn(&mut loadfn, "glCreateShaderProgramv", &[]))
    } }
}
pub mod CreateTextures {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateTextures.is_loaded }
        else { gl::CreateTextures::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateTextures = FnPtr::new(metaloadfn(&mut loadfn, "glCreateTextures", &[]))
    } }
}
pub mod CreateTransformFeedbacks {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateTransformFeedbacks.is_loaded }
        else { gl::CreateTransformFeedbacks::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, "glCreateTransformFeedbacks", &[]))
    } }
}
pub mod CreateVertexArrays {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CreateVertexArrays.is_loaded }
        else { gl::CreateVertexArrays::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CreateVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, "glCreateVertexArrays", &[]))
    } }
}
pub mod CullFace {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).CullFace.is_loaded }
        else { gl::CullFace::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).CullFace = FnPtr::new(metaloadfn(&mut loadfn, "glCullFace", &[]))
    } }
}
pub mod DebugMessageCallback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DebugMessageCallback.is_loaded }
        else { gl::DebugMessageCallback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DebugMessageCallback = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageCallback", &["glDebugMessageCallbackARB", "glDebugMessageCallbackKHR"]))
    } }
}
pub mod DebugMessageControl {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DebugMessageControl.is_loaded }
        else { gl::DebugMessageControl::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DebugMessageControl = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageControl", &["glDebugMessageControlARB", "glDebugMessageControlKHR"]))
    } }
}
pub mod DebugMessageInsert {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DebugMessageInsert.is_loaded }
        else { gl::DebugMessageInsert::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DebugMessageInsert = FnPtr::new(metaloadfn(&mut loadfn, "glDebugMessageInsert", &["glDebugMessageInsertARB", "glDebugMessageInsertKHR"]))
    } }
}
pub mod DeleteBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteBuffers.is_loaded }
        else { gl::DeleteBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteBuffers", &["glDeleteBuffersARB"]))
    } }
}
pub mod DeleteFramebuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteFramebuffers.is_loaded }
        else { gl::DeleteFramebuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteFramebuffers", &["glDeleteFramebuffersEXT"]))
    } }
}
pub mod DeleteProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteProgram.is_loaded }
        else { gl::DeleteProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteProgram = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteProgram", &[]))
    } }
}
pub mod DeleteProgramPipelines {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteProgramPipelines.is_loaded }
        else { gl::DeleteProgramPipelines::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteProgramPipelines", &[]))
    } }
}
pub mod DeleteQueries {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteQueries.is_loaded }
        else { gl::DeleteQueries::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteQueries = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteQueries", &["glDeleteQueriesARB"]))
    } }
}
pub mod DeleteRenderbuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteRenderbuffers.is_loaded }
        else { gl::DeleteRenderbuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteRenderbuffers", &["glDeleteRenderbuffersEXT"]))
    } }
}
pub mod DeleteSamplers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteSamplers.is_loaded }
        else { gl::DeleteSamplers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteSamplers", &[]))
    } }
}
pub mod DeleteShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteShader.is_loaded }
        else { gl::DeleteShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteShader = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteShader", &[]))
    } }
}
pub mod DeleteSync {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteSync.is_loaded }
        else { gl::DeleteSync::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteSync = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteSync", &["glDeleteSyncAPPLE"]))
    } }
}
pub mod DeleteTextures {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteTextures.is_loaded }
        else { gl::DeleteTextures::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteTextures = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTextures", &[]))
    } }
}
pub mod DeleteTransformFeedbacks {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteTransformFeedbacks.is_loaded }
        else { gl::DeleteTransformFeedbacks::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTransformFeedbacks", &["glDeleteTransformFeedbacksNV"]))
    } }
}
pub mod DeleteVertexArrays {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DeleteVertexArrays.is_loaded }
        else { gl::DeleteVertexArrays::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DeleteVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteVertexArrays", &["glDeleteVertexArraysAPPLE", "glDeleteVertexArraysOES"]))
    } }
}
pub mod DepthFunc {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthFunc.is_loaded }
        else { gl::DepthFunc::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthFunc = FnPtr::new(metaloadfn(&mut loadfn, "glDepthFunc", &[]))
    } }
}
pub mod DepthMask {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthMask.is_loaded }
        else { gl::DepthMask::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthMask = FnPtr::new(metaloadfn(&mut loadfn, "glDepthMask", &[]))
    } }
}
pub mod DepthRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthRange.is_loaded }
        else { gl::DepthRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthRange = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRange", &[]))
    } }
}
pub mod DepthRangeArrayv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthRangeArrayv.is_loaded }
        else { gl::DepthRangeArrayv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthRangeArrayv = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangeArrayv", &[]))
    } }
}
pub mod DepthRangeIndexed {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthRangeIndexed.is_loaded }
        else { gl::DepthRangeIndexed::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthRangeIndexed = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangeIndexed", &[]))
    } }
}
pub mod DepthRangef {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DepthRangef.is_loaded }
        else { gl::DepthRangef::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DepthRangef = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangef", &["glDepthRangefOES"]))
    } }
}
pub mod DetachShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DetachShader.is_loaded }
        else { gl::DetachShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DetachShader = FnPtr::new(metaloadfn(&mut loadfn, "glDetachShader", &["glDetachObjectARB"]))
    } }
}
pub mod Disable {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Disable.is_loaded }
        else { gl::Disable::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Disable = FnPtr::new(metaloadfn(&mut loadfn, "glDisable", &[]))
    } }
}
pub mod DisableVertexArrayAttrib {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DisableVertexArrayAttrib.is_loaded }
        else { gl::DisableVertexArrayAttrib::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DisableVertexArrayAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glDisableVertexArrayAttrib", &[]))
    } }
}
pub mod DisableVertexAttribArray {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DisableVertexAttribArray.is_loaded }
        else { gl::DisableVertexAttribArray::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DisableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, "glDisableVertexAttribArray", &["glDisableVertexAttribArrayARB"]))
    } }
}
pub mod Disablei {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Disablei.is_loaded }
        else { gl::Disablei::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Disablei = FnPtr::new(metaloadfn(&mut loadfn, "glDisablei", &["glDisableIndexedEXT", "glDisableiEXT", "glDisableiNV", "glDisableiOES"]))
    } }
}
pub mod DispatchCompute {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DispatchCompute.is_loaded }
        else { gl::DispatchCompute::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DispatchCompute = FnPtr::new(metaloadfn(&mut loadfn, "glDispatchCompute", &[]))
    } }
}
pub mod DispatchComputeIndirect {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DispatchComputeIndirect.is_loaded }
        else { gl::DispatchComputeIndirect::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DispatchComputeIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDispatchComputeIndirect", &[]))
    } }
}
pub mod DrawArrays {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawArrays.is_loaded }
        else { gl::DrawArrays::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArrays", &["glDrawArraysEXT"]))
    } }
}
pub mod DrawArraysIndirect {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawArraysIndirect.is_loaded }
        else { gl::DrawArraysIndirect::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawArraysIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArraysIndirect", &[]))
    } }
}
pub mod DrawArraysInstanced {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawArraysInstanced.is_loaded }
        else { gl::DrawArraysInstanced::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawArraysInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArraysInstanced", &["glDrawArraysInstancedANGLE", "glDrawArraysInstancedARB", "glDrawArraysInstancedEXT", "glDrawArraysInstancedNV"]))
    } }
}
pub mod DrawArraysInstancedBaseInstance {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawArraysInstancedBaseInstance.is_loaded }
        else { gl::DrawArraysInstancedBaseInstance::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawArraysInstancedBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArraysInstancedBaseInstance", &["glDrawArraysInstancedBaseInstanceEXT"]))
    } }
}
pub mod DrawBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawBuffer.is_loaded }
        else { gl::DrawBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glDrawBuffer", &[]))
    } }
}
pub mod DrawBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawBuffers.is_loaded }
        else { gl::DrawBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDrawBuffers", &["glDrawBuffersARB", "glDrawBuffersATI", "glDrawBuffersEXT"]))
    } }
}
pub mod DrawElements {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElements.is_loaded }
        else { gl::DrawElements::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElements", &[]))
    } }
}
pub mod DrawElementsBaseVertex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsBaseVertex.is_loaded }
        else { gl::DrawElementsBaseVertex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsBaseVertex", &["glDrawElementsBaseVertexEXT", "glDrawElementsBaseVertexOES"]))
    } }
}
pub mod DrawElementsIndirect {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsIndirect.is_loaded }
        else { gl::DrawElementsIndirect::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsIndirect", &[]))
    } }
}
pub mod DrawElementsInstanced {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsInstanced.is_loaded }
        else { gl::DrawElementsInstanced::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstanced", &["glDrawElementsInstancedANGLE", "glDrawElementsInstancedARB", "glDrawElementsInstancedEXT", "glDrawElementsInstancedNV"]))
    } }
}
pub mod DrawElementsInstancedBaseInstance {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsInstancedBaseInstance.is_loaded }
        else { gl::DrawElementsInstancedBaseInstance::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsInstancedBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstancedBaseInstance", &["glDrawElementsInstancedBaseInstanceEXT"]))
    } }
}
pub mod DrawElementsInstancedBaseVertex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsInstancedBaseVertex.is_loaded }
        else { gl::DrawElementsInstancedBaseVertex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsInstancedBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstancedBaseVertex", &["glDrawElementsInstancedBaseVertexEXT", "glDrawElementsInstancedBaseVertexOES"]))
    } }
}
pub mod DrawElementsInstancedBaseVertexBaseInstance {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawElementsInstancedBaseVertexBaseInstance.is_loaded }
        else { gl::DrawElementsInstancedBaseVertexBaseInstance::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawElementsInstancedBaseVertexBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElementsInstancedBaseVertexBaseInstance", &["glDrawElementsInstancedBaseVertexBaseInstanceEXT"]))
    } }
}
pub mod DrawRangeElements {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawRangeElements.is_loaded }
        else { gl::DrawRangeElements::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawRangeElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawRangeElements", &["glDrawRangeElementsEXT"]))
    } }
}
pub mod DrawRangeElementsBaseVertex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawRangeElementsBaseVertex.is_loaded }
        else { gl::DrawRangeElementsBaseVertex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawRangeElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glDrawRangeElementsBaseVertex", &["glDrawRangeElementsBaseVertexEXT", "glDrawRangeElementsBaseVertexOES"]))
    } }
}
pub mod DrawTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawTransformFeedback.is_loaded }
        else { gl::DrawTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTransformFeedback", &["glDrawTransformFeedbackEXT", "glDrawTransformFeedbackNV"]))
    } }
}
pub mod DrawTransformFeedbackInstanced {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawTransformFeedbackInstanced.is_loaded }
        else { gl::DrawTransformFeedbackInstanced::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawTransformFeedbackInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTransformFeedbackInstanced", &["glDrawTransformFeedbackInstancedEXT"]))
    } }
}
pub mod DrawTransformFeedbackStream {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawTransformFeedbackStream.is_loaded }
        else { gl::DrawTransformFeedbackStream::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawTransformFeedbackStream = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTransformFeedbackStream", &[]))
    } }
}
pub mod DrawTransformFeedbackStreamInstanced {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).DrawTransformFeedbackStreamInstanced.is_loaded }
        else { gl::DrawTransformFeedbackStreamInstanced::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).DrawTransformFeedbackStreamInstanced = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTransformFeedbackStreamInstanced", &[]))
    } }
}
pub mod Enable {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Enable.is_loaded }
        else { gl::Enable::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Enable = FnPtr::new(metaloadfn(&mut loadfn, "glEnable", &[]))
    } }
}
pub mod EnableVertexArrayAttrib {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EnableVertexArrayAttrib.is_loaded }
        else { gl::EnableVertexArrayAttrib::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EnableVertexArrayAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glEnableVertexArrayAttrib", &[]))
    } }
}
pub mod EnableVertexAttribArray {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EnableVertexAttribArray.is_loaded }
        else { gl::EnableVertexAttribArray::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EnableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, "glEnableVertexAttribArray", &["glEnableVertexAttribArrayARB"]))
    } }
}
pub mod Enablei {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Enablei.is_loaded }
        else { gl::Enablei::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Enablei = FnPtr::new(metaloadfn(&mut loadfn, "glEnablei", &["glEnableIndexedEXT", "glEnableiEXT", "glEnableiNV", "glEnableiOES"]))
    } }
}
pub mod EndConditionalRender {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EndConditionalRender.is_loaded }
        else { gl::EndConditionalRender::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EndConditionalRender = FnPtr::new(metaloadfn(&mut loadfn, "glEndConditionalRender", &["glEndConditionalRenderNV", "glEndConditionalRenderNVX"]))
    } }
}
pub mod EndQuery {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EndQuery.is_loaded }
        else { gl::EndQuery::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EndQuery = FnPtr::new(metaloadfn(&mut loadfn, "glEndQuery", &["glEndQueryARB"]))
    } }
}
pub mod EndQueryIndexed {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EndQueryIndexed.is_loaded }
        else { gl::EndQueryIndexed::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EndQueryIndexed = FnPtr::new(metaloadfn(&mut loadfn, "glEndQueryIndexed", &[]))
    } }
}
pub mod EndTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).EndTransformFeedback.is_loaded }
        else { gl::EndTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).EndTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glEndTransformFeedback", &["glEndTransformFeedbackEXT", "glEndTransformFeedbackNV"]))
    } }
}
pub mod FenceSync {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FenceSync.is_loaded }
        else { gl::FenceSync::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FenceSync = FnPtr::new(metaloadfn(&mut loadfn, "glFenceSync", &["glFenceSyncAPPLE"]))
    } }
}
pub mod Finish {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Finish.is_loaded }
        else { gl::Finish::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Finish = FnPtr::new(metaloadfn(&mut loadfn, "glFinish", &[]))
    } }
}
pub mod Flush {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Flush.is_loaded }
        else { gl::Flush::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Flush = FnPtr::new(metaloadfn(&mut loadfn, "glFlush", &[]))
    } }
}
pub mod FlushMappedBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FlushMappedBufferRange.is_loaded }
        else { gl::FlushMappedBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FlushMappedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glFlushMappedBufferRange", &["glFlushMappedBufferRangeAPPLE", "glFlushMappedBufferRangeEXT"]))
    } }
}
pub mod FlushMappedNamedBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FlushMappedNamedBufferRange.is_loaded }
        else { gl::FlushMappedNamedBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FlushMappedNamedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glFlushMappedNamedBufferRange", &[]))
    } }
}
pub mod FramebufferParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferParameteri.is_loaded }
        else { gl::FramebufferParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferParameteri", &[]))
    } }
}
pub mod FramebufferRenderbuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferRenderbuffer.is_loaded }
        else { gl::FramebufferRenderbuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferRenderbuffer", &["glFramebufferRenderbufferEXT"]))
    } }
}
pub mod FramebufferTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferTexture.is_loaded }
        else { gl::FramebufferTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferTexture = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture", &["glFramebufferTextureARB", "glFramebufferTextureEXT", "glFramebufferTextureOES"]))
    } }
}
pub mod FramebufferTexture1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferTexture1D.is_loaded }
        else { gl::FramebufferTexture1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferTexture1D = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture1D", &["glFramebufferTexture1DEXT"]))
    } }
}
pub mod FramebufferTexture2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferTexture2D.is_loaded }
        else { gl::FramebufferTexture2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferTexture2D = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture2D", &["glFramebufferTexture2DEXT"]))
    } }
}
pub mod FramebufferTexture3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferTexture3D.is_loaded }
        else { gl::FramebufferTexture3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferTexture3D = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture3D", &["glFramebufferTexture3DEXT"]))
    } }
}
pub mod FramebufferTextureLayer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FramebufferTextureLayer.is_loaded }
        else { gl::FramebufferTextureLayer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FramebufferTextureLayer = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTextureLayer", &["glFramebufferTextureLayerARB", "glFramebufferTextureLayerEXT"]))
    } }
}
pub mod FrontFace {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).FrontFace.is_loaded }
        else { gl::FrontFace::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).FrontFace = FnPtr::new(metaloadfn(&mut loadfn, "glFrontFace", &[]))
    } }
}
pub mod GenBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenBuffers.is_loaded }
        else { gl::GenBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenBuffers", &["glGenBuffersARB"]))
    } }
}
pub mod GenFramebuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenFramebuffers.is_loaded }
        else { gl::GenFramebuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenFramebuffers", &["glGenFramebuffersEXT"]))
    } }
}
pub mod GenProgramPipelines {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenProgramPipelines.is_loaded }
        else { gl::GenProgramPipelines::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, "glGenProgramPipelines", &[]))
    } }
}
pub mod GenQueries {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenQueries.is_loaded }
        else { gl::GenQueries::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenQueries = FnPtr::new(metaloadfn(&mut loadfn, "glGenQueries", &["glGenQueriesARB"]))
    } }
}
pub mod GenRenderbuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenRenderbuffers.is_loaded }
        else { gl::GenRenderbuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenRenderbuffers", &["glGenRenderbuffersEXT"]))
    } }
}
pub mod GenSamplers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenSamplers.is_loaded }
        else { gl::GenSamplers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenSamplers = FnPtr::new(metaloadfn(&mut loadfn, "glGenSamplers", &[]))
    } }
}
pub mod GenTextures {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenTextures.is_loaded }
        else { gl::GenTextures::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenTextures = FnPtr::new(metaloadfn(&mut loadfn, "glGenTextures", &[]))
    } }
}
pub mod GenTransformFeedbacks {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenTransformFeedbacks.is_loaded }
        else { gl::GenTransformFeedbacks::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, "glGenTransformFeedbacks", &["glGenTransformFeedbacksNV"]))
    } }
}
pub mod GenVertexArrays {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenVertexArrays.is_loaded }
        else { gl::GenVertexArrays::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, "glGenVertexArrays", &["glGenVertexArraysAPPLE", "glGenVertexArraysOES"]))
    } }
}
pub mod GenerateMipmap {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenerateMipmap.is_loaded }
        else { gl::GenerateMipmap::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenerateMipmap = FnPtr::new(metaloadfn(&mut loadfn, "glGenerateMipmap", &["glGenerateMipmapEXT"]))
    } }
}
pub mod GenerateTextureMipmap {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GenerateTextureMipmap.is_loaded }
        else { gl::GenerateTextureMipmap::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GenerateTextureMipmap = FnPtr::new(metaloadfn(&mut loadfn, "glGenerateTextureMipmap", &[]))
    } }
}
pub mod GetActiveAtomicCounterBufferiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveAtomicCounterBufferiv.is_loaded }
        else { gl::GetActiveAtomicCounterBufferiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveAtomicCounterBufferiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveAtomicCounterBufferiv", &[]))
    } }
}
pub mod GetActiveAttrib {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveAttrib.is_loaded }
        else { gl::GetActiveAttrib::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveAttrib = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveAttrib", &["glGetActiveAttribARB"]))
    } }
}
pub mod GetActiveSubroutineName {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveSubroutineName.is_loaded }
        else { gl::GetActiveSubroutineName::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveSubroutineName = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveSubroutineName", &[]))
    } }
}
pub mod GetActiveSubroutineUniformName {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveSubroutineUniformName.is_loaded }
        else { gl::GetActiveSubroutineUniformName::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveSubroutineUniformName = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveSubroutineUniformName", &[]))
    } }
}
pub mod GetActiveSubroutineUniformiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveSubroutineUniformiv.is_loaded }
        else { gl::GetActiveSubroutineUniformiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveSubroutineUniformiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveSubroutineUniformiv", &[]))
    } }
}
pub mod GetActiveUniform {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveUniform.is_loaded }
        else { gl::GetActiveUniform::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveUniform = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniform", &["glGetActiveUniformARB"]))
    } }
}
pub mod GetActiveUniformBlockName {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveUniformBlockName.is_loaded }
        else { gl::GetActiveUniformBlockName::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveUniformBlockName = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformBlockName", &[]))
    } }
}
pub mod GetActiveUniformBlockiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveUniformBlockiv.is_loaded }
        else { gl::GetActiveUniformBlockiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveUniformBlockiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformBlockiv", &[]))
    } }
}
pub mod GetActiveUniformName {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveUniformName.is_loaded }
        else { gl::GetActiveUniformName::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveUniformName = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformName", &[]))
    } }
}
pub mod GetActiveUniformsiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetActiveUniformsiv.is_loaded }
        else { gl::GetActiveUniformsiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetActiveUniformsiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetActiveUniformsiv", &[]))
    } }
}
pub mod GetAttachedShaders {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetAttachedShaders.is_loaded }
        else { gl::GetAttachedShaders::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetAttachedShaders = FnPtr::new(metaloadfn(&mut loadfn, "glGetAttachedShaders", &[]))
    } }
}
pub mod GetAttribLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetAttribLocation.is_loaded }
        else { gl::GetAttribLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetAttribLocation", &["glGetAttribLocationARB"]))
    } }
}
pub mod GetBooleani_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBooleani_v.is_loaded }
        else { gl::GetBooleani_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBooleani_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleani_v", &["glGetBooleanIndexedvEXT"]))
    } }
}
pub mod GetBooleanv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBooleanv.is_loaded }
        else { gl::GetBooleanv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBooleanv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleanv", &[]))
    } }
}
pub mod GetBufferParameteri64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBufferParameteri64v.is_loaded }
        else { gl::GetBufferParameteri64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBufferParameteri64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferParameteri64v", &[]))
    } }
}
pub mod GetBufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBufferParameteriv.is_loaded }
        else { gl::GetBufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferParameteriv", &["glGetBufferParameterivARB"]))
    } }
}
pub mod GetBufferPointerv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBufferPointerv.is_loaded }
        else { gl::GetBufferPointerv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBufferPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferPointerv", &["glGetBufferPointervARB", "glGetBufferPointervOES"]))
    } }
}
pub mod GetBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetBufferSubData.is_loaded }
        else { gl::GetBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferSubData", &["glGetBufferSubDataARB"]))
    } }
}
pub mod GetCompressedTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetCompressedTexImage.is_loaded }
        else { gl::GetCompressedTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetCompressedTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetCompressedTexImage", &["glGetCompressedTexImageARB"]))
    } }
}
pub mod GetCompressedTextureImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetCompressedTextureImage.is_loaded }
        else { gl::GetCompressedTextureImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetCompressedTextureImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetCompressedTextureImage", &[]))
    } }
}
pub mod GetCompressedTextureSubImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetCompressedTextureSubImage.is_loaded }
        else { gl::GetCompressedTextureSubImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetCompressedTextureSubImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetCompressedTextureSubImage", &[]))
    } }
}
pub mod GetDebugMessageLog {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetDebugMessageLog.is_loaded }
        else { gl::GetDebugMessageLog::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetDebugMessageLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetDebugMessageLog", &["glGetDebugMessageLogARB", "glGetDebugMessageLogKHR"]))
    } }
}
pub mod GetDoublei_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetDoublei_v.is_loaded }
        else { gl::GetDoublei_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetDoublei_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetDoublei_v", &["glGetDoubleIndexedvEXT", "glGetDoublei_vEXT"]))
    } }
}
pub mod GetDoublev {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetDoublev.is_loaded }
        else { gl::GetDoublev::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetDoublev = FnPtr::new(metaloadfn(&mut loadfn, "glGetDoublev", &[]))
    } }
}
pub mod GetError {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetError.is_loaded }
        else { gl::GetError::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetError = FnPtr::new(metaloadfn(&mut loadfn, "glGetError", &[]))
    } }
}
pub mod GetFloati_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFloati_v.is_loaded }
        else { gl::GetFloati_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFloati_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetFloati_v", &["glGetFloatIndexedvEXT", "glGetFloati_vEXT", "glGetFloati_vNV", "glGetFloati_vOES"]))
    } }
}
pub mod GetFloatv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFloatv.is_loaded }
        else { gl::GetFloatv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFloatv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFloatv", &[]))
    } }
}
pub mod GetFragDataIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFragDataIndex.is_loaded }
        else { gl::GetFragDataIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFragDataIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetFragDataIndex", &["glGetFragDataIndexEXT"]))
    } }
}
pub mod GetFragDataLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFragDataLocation.is_loaded }
        else { gl::GetFragDataLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFragDataLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetFragDataLocation", &["glGetFragDataLocationEXT"]))
    } }
}
pub mod GetFramebufferAttachmentParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFramebufferAttachmentParameteriv.is_loaded }
        else { gl::GetFramebufferAttachmentParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFramebufferAttachmentParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFramebufferAttachmentParameteriv", &["glGetFramebufferAttachmentParameterivEXT"]))
    } }
}
pub mod GetFramebufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetFramebufferParameteriv.is_loaded }
        else { gl::GetFramebufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetFramebufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFramebufferParameteriv", &[]))
    } }
}
pub mod GetGraphicsResetStatus {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetGraphicsResetStatus.is_loaded }
        else { gl::GetGraphicsResetStatus::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetGraphicsResetStatus = FnPtr::new(metaloadfn(&mut loadfn, "glGetGraphicsResetStatus", &["glGetGraphicsResetStatusEXT", "glGetGraphicsResetStatusKHR"]))
    } }
}
pub mod GetInteger64i_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetInteger64i_v.is_loaded }
        else { gl::GetInteger64i_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetInteger64i_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetInteger64i_v", &[]))
    } }
}
pub mod GetInteger64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetInteger64v.is_loaded }
        else { gl::GetInteger64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetInteger64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetInteger64v", &["glGetInteger64vAPPLE"]))
    } }
}
pub mod GetIntegeri_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetIntegeri_v.is_loaded }
        else { gl::GetIntegeri_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetIntegeri_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegeri_v", &["glGetIntegerIndexedvEXT"]))
    } }
}
pub mod GetIntegerv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetIntegerv.is_loaded }
        else { gl::GetIntegerv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetIntegerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegerv", &[]))
    } }
}
pub mod GetInternalformati64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetInternalformati64v.is_loaded }
        else { gl::GetInternalformati64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetInternalformati64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetInternalformati64v", &[]))
    } }
}
pub mod GetInternalformativ {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetInternalformativ.is_loaded }
        else { gl::GetInternalformativ::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetInternalformativ = FnPtr::new(metaloadfn(&mut loadfn, "glGetInternalformativ", &[]))
    } }
}
pub mod GetMultisamplefv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetMultisamplefv.is_loaded }
        else { gl::GetMultisamplefv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetMultisamplefv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMultisamplefv", &["glGetMultisamplefvNV"]))
    } }
}
pub mod GetNamedBufferParameteri64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedBufferParameteri64v.is_loaded }
        else { gl::GetNamedBufferParameteri64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedBufferParameteri64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedBufferParameteri64v", &[]))
    } }
}
pub mod GetNamedBufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedBufferParameteriv.is_loaded }
        else { gl::GetNamedBufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedBufferParameteriv", &[]))
    } }
}
pub mod GetNamedBufferPointerv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedBufferPointerv.is_loaded }
        else { gl::GetNamedBufferPointerv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedBufferPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedBufferPointerv", &[]))
    } }
}
pub mod GetNamedBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedBufferSubData.is_loaded }
        else { gl::GetNamedBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedBufferSubData", &[]))
    } }
}
pub mod GetNamedFramebufferAttachmentParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedFramebufferAttachmentParameteriv.is_loaded }
        else { gl::GetNamedFramebufferAttachmentParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedFramebufferAttachmentParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedFramebufferAttachmentParameteriv", &[]))
    } }
}
pub mod GetNamedFramebufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedFramebufferParameteriv.is_loaded }
        else { gl::GetNamedFramebufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedFramebufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedFramebufferParameteriv", &[]))
    } }
}
pub mod GetNamedRenderbufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetNamedRenderbufferParameteriv.is_loaded }
        else { gl::GetNamedRenderbufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetNamedRenderbufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetNamedRenderbufferParameteriv", &[]))
    } }
}
pub mod GetObjectLabel {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetObjectLabel.is_loaded }
        else { gl::GetObjectLabel::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, "glGetObjectLabel", &["glGetObjectLabelKHR"]))
    } }
}
pub mod GetObjectPtrLabel {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetObjectPtrLabel.is_loaded }
        else { gl::GetObjectPtrLabel::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, "glGetObjectPtrLabel", &["glGetObjectPtrLabelKHR"]))
    } }
}
pub mod GetPointerv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetPointerv.is_loaded }
        else { gl::GetPointerv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"]))
    } }
}
pub mod GetProgramBinary {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramBinary.is_loaded }
        else { gl::GetProgramBinary::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramBinary", &["glGetProgramBinaryOES"]))
    } }
}
pub mod GetProgramInfoLog {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramInfoLog.is_loaded }
        else { gl::GetProgramInfoLog::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramInfoLog", &[]))
    } }
}
pub mod GetProgramInterfaceiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramInterfaceiv.is_loaded }
        else { gl::GetProgramInterfaceiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramInterfaceiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramInterfaceiv", &[]))
    } }
}
pub mod GetProgramPipelineInfoLog {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramPipelineInfoLog.is_loaded }
        else { gl::GetProgramPipelineInfoLog::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramPipelineInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramPipelineInfoLog", &[]))
    } }
}
pub mod GetProgramPipelineiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramPipelineiv.is_loaded }
        else { gl::GetProgramPipelineiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramPipelineiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramPipelineiv", &[]))
    } }
}
pub mod GetProgramResourceIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramResourceIndex.is_loaded }
        else { gl::GetProgramResourceIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramResourceIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceIndex", &[]))
    } }
}
pub mod GetProgramResourceLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramResourceLocation.is_loaded }
        else { gl::GetProgramResourceLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramResourceLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceLocation", &[]))
    } }
}
pub mod GetProgramResourceLocationIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramResourceLocationIndex.is_loaded }
        else { gl::GetProgramResourceLocationIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramResourceLocationIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceLocationIndex", &[]))
    } }
}
pub mod GetProgramResourceName {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramResourceName.is_loaded }
        else { gl::GetProgramResourceName::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramResourceName = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceName", &[]))
    } }
}
pub mod GetProgramResourceiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramResourceiv.is_loaded }
        else { gl::GetProgramResourceiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramResourceiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramResourceiv", &[]))
    } }
}
pub mod GetProgramStageiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramStageiv.is_loaded }
        else { gl::GetProgramStageiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramStageiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramStageiv", &[]))
    } }
}
pub mod GetProgramiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetProgramiv.is_loaded }
        else { gl::GetProgramiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetProgramiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetProgramiv", &[]))
    } }
}
pub mod GetQueryBufferObjecti64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryBufferObjecti64v.is_loaded }
        else { gl::GetQueryBufferObjecti64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryBufferObjecti64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryBufferObjecti64v", &[]))
    } }
}
pub mod GetQueryBufferObjectiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryBufferObjectiv.is_loaded }
        else { gl::GetQueryBufferObjectiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryBufferObjectiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryBufferObjectiv", &[]))
    } }
}
pub mod GetQueryBufferObjectui64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryBufferObjectui64v.is_loaded }
        else { gl::GetQueryBufferObjectui64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryBufferObjectui64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryBufferObjectui64v", &[]))
    } }
}
pub mod GetQueryBufferObjectuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryBufferObjectuiv.is_loaded }
        else { gl::GetQueryBufferObjectuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryBufferObjectuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryBufferObjectuiv", &[]))
    } }
}
pub mod GetQueryIndexediv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryIndexediv.is_loaded }
        else { gl::GetQueryIndexediv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryIndexediv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryIndexediv", &[]))
    } }
}
pub mod GetQueryObjecti64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryObjecti64v.is_loaded }
        else { gl::GetQueryObjecti64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryObjecti64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryObjecti64v", &["glGetQueryObjecti64vEXT"]))
    } }
}
pub mod GetQueryObjectiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryObjectiv.is_loaded }
        else { gl::GetQueryObjectiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryObjectiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryObjectiv", &["glGetQueryObjectivARB", "glGetQueryObjectivEXT"]))
    } }
}
pub mod GetQueryObjectui64v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryObjectui64v.is_loaded }
        else { gl::GetQueryObjectui64v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryObjectui64v = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryObjectui64v", &["glGetQueryObjectui64vEXT"]))
    } }
}
pub mod GetQueryObjectuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryObjectuiv.is_loaded }
        else { gl::GetQueryObjectuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryObjectuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryObjectuiv", &["glGetQueryObjectuivARB"]))
    } }
}
pub mod GetQueryiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetQueryiv.is_loaded }
        else { gl::GetQueryiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetQueryiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetQueryiv", &["glGetQueryivARB"]))
    } }
}
pub mod GetRenderbufferParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetRenderbufferParameteriv.is_loaded }
        else { gl::GetRenderbufferParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetRenderbufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetRenderbufferParameteriv", &["glGetRenderbufferParameterivEXT"]))
    } }
}
pub mod GetSamplerParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSamplerParameterIiv.is_loaded }
        else { gl::GetSamplerParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterIiv", &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIivOES"]))
    } }
}
pub mod GetSamplerParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSamplerParameterIuiv.is_loaded }
        else { gl::GetSamplerParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterIuiv", &["glGetSamplerParameterIuivEXT", "glGetSamplerParameterIuivOES"]))
    } }
}
pub mod GetSamplerParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSamplerParameterfv.is_loaded }
        else { gl::GetSamplerParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameterfv", &[]))
    } }
}
pub mod GetSamplerParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSamplerParameteriv.is_loaded }
        else { gl::GetSamplerParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSamplerParameteriv", &[]))
    } }
}
pub mod GetShaderInfoLog {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetShaderInfoLog.is_loaded }
        else { gl::GetShaderInfoLog::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetShaderInfoLog = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderInfoLog", &[]))
    } }
}
pub mod GetShaderPrecisionFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetShaderPrecisionFormat.is_loaded }
        else { gl::GetShaderPrecisionFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetShaderPrecisionFormat = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderPrecisionFormat", &[]))
    } }
}
pub mod GetShaderSource {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetShaderSource.is_loaded }
        else { gl::GetShaderSource::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetShaderSource = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderSource", &["glGetShaderSourceARB"]))
    } }
}
pub mod GetShaderiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetShaderiv.is_loaded }
        else { gl::GetShaderiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetShaderiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetShaderiv", &[]))
    } }
}
pub mod GetString {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetString.is_loaded }
        else { gl::GetString::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetString = FnPtr::new(metaloadfn(&mut loadfn, "glGetString", &[]))
    } }
}
pub mod GetStringi {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetStringi.is_loaded }
        else { gl::GetStringi::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetStringi = FnPtr::new(metaloadfn(&mut loadfn, "glGetStringi", &[]))
    } }
}
pub mod GetSubroutineIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSubroutineIndex.is_loaded }
        else { gl::GetSubroutineIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSubroutineIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetSubroutineIndex", &[]))
    } }
}
pub mod GetSubroutineUniformLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSubroutineUniformLocation.is_loaded }
        else { gl::GetSubroutineUniformLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSubroutineUniformLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetSubroutineUniformLocation", &[]))
    } }
}
pub mod GetSynciv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetSynciv.is_loaded }
        else { gl::GetSynciv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetSynciv = FnPtr::new(metaloadfn(&mut loadfn, "glGetSynciv", &["glGetSyncivAPPLE"]))
    } }
}
pub mod GetTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexImage.is_loaded }
        else { gl::GetTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexImage", &[]))
    } }
}
pub mod GetTexLevelParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexLevelParameterfv.is_loaded }
        else { gl::GetTexLevelParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameterfv", &[]))
    } }
}
pub mod GetTexLevelParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexLevelParameteriv.is_loaded }
        else { gl::GetTexLevelParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexLevelParameteriv", &[]))
    } }
}
pub mod GetTexParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexParameterIiv.is_loaded }
        else { gl::GetTexParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterIiv", &["glGetTexParameterIivEXT", "glGetTexParameterIivOES"]))
    } }
}
pub mod GetTexParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexParameterIuiv.is_loaded }
        else { gl::GetTexParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterIuiv", &["glGetTexParameterIuivEXT", "glGetTexParameterIuivOES"]))
    } }
}
pub mod GetTexParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexParameterfv.is_loaded }
        else { gl::GetTexParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterfv", &[]))
    } }
}
pub mod GetTexParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTexParameteriv.is_loaded }
        else { gl::GetTexParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameteriv", &[]))
    } }
}
pub mod GetTextureImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureImage.is_loaded }
        else { gl::GetTextureImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureImage", &[]))
    } }
}
pub mod GetTextureLevelParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureLevelParameterfv.is_loaded }
        else { gl::GetTextureLevelParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureLevelParameterfv", &[]))
    } }
}
pub mod GetTextureLevelParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureLevelParameteriv.is_loaded }
        else { gl::GetTextureLevelParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureLevelParameteriv", &[]))
    } }
}
pub mod GetTextureParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureParameterIiv.is_loaded }
        else { gl::GetTextureParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureParameterIiv", &[]))
    } }
}
pub mod GetTextureParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureParameterIuiv.is_loaded }
        else { gl::GetTextureParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureParameterIuiv", &[]))
    } }
}
pub mod GetTextureParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureParameterfv.is_loaded }
        else { gl::GetTextureParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureParameterfv", &[]))
    } }
}
pub mod GetTextureParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureParameteriv.is_loaded }
        else { gl::GetTextureParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureParameteriv", &[]))
    } }
}
pub mod GetTextureSubImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTextureSubImage.is_loaded }
        else { gl::GetTextureSubImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTextureSubImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetTextureSubImage", &[]))
    } }
}
pub mod GetTransformFeedbackVarying {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTransformFeedbackVarying.is_loaded }
        else { gl::GetTransformFeedbackVarying::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTransformFeedbackVarying = FnPtr::new(metaloadfn(&mut loadfn, "glGetTransformFeedbackVarying", &["glGetTransformFeedbackVaryingEXT"]))
    } }
}
pub mod GetTransformFeedbacki64_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTransformFeedbacki64_v.is_loaded }
        else { gl::GetTransformFeedbacki64_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTransformFeedbacki64_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetTransformFeedbacki64_v", &[]))
    } }
}
pub mod GetTransformFeedbacki_v {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTransformFeedbacki_v.is_loaded }
        else { gl::GetTransformFeedbacki_v::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTransformFeedbacki_v = FnPtr::new(metaloadfn(&mut loadfn, "glGetTransformFeedbacki_v", &[]))
    } }
}
pub mod GetTransformFeedbackiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetTransformFeedbackiv.is_loaded }
        else { gl::GetTransformFeedbackiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetTransformFeedbackiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTransformFeedbackiv", &[]))
    } }
}
pub mod GetUniformBlockIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformBlockIndex.is_loaded }
        else { gl::GetUniformBlockIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformBlockIndex = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformBlockIndex", &[]))
    } }
}
pub mod GetUniformIndices {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformIndices.is_loaded }
        else { gl::GetUniformIndices::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformIndices = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformIndices", &[]))
    } }
}
pub mod GetUniformLocation {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformLocation.is_loaded }
        else { gl::GetUniformLocation::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformLocation = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformLocation", &["glGetUniformLocationARB"]))
    } }
}
pub mod GetUniformSubroutineuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformSubroutineuiv.is_loaded }
        else { gl::GetUniformSubroutineuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformSubroutineuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformSubroutineuiv", &[]))
    } }
}
pub mod GetUniformdv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformdv.is_loaded }
        else { gl::GetUniformdv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformdv", &[]))
    } }
}
pub mod GetUniformfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformfv.is_loaded }
        else { gl::GetUniformfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformfv", &["glGetUniformfvARB"]))
    } }
}
pub mod GetUniformiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformiv.is_loaded }
        else { gl::GetUniformiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformiv", &["glGetUniformivARB"]))
    } }
}
pub mod GetUniformuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetUniformuiv.is_loaded }
        else { gl::GetUniformuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetUniformuiv", &["glGetUniformuivEXT"]))
    } }
}
pub mod GetVertexArrayIndexed64iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexArrayIndexed64iv.is_loaded }
        else { gl::GetVertexArrayIndexed64iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexArrayIndexed64iv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexArrayIndexed64iv", &[]))
    } }
}
pub mod GetVertexArrayIndexediv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexArrayIndexediv.is_loaded }
        else { gl::GetVertexArrayIndexediv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexArrayIndexediv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexArrayIndexediv", &[]))
    } }
}
pub mod GetVertexArrayiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexArrayiv.is_loaded }
        else { gl::GetVertexArrayiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexArrayiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexArrayiv", &[]))
    } }
}
pub mod GetVertexAttribIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribIiv.is_loaded }
        else { gl::GetVertexAttribIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribIiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribIiv", &["glGetVertexAttribIivEXT"]))
    } }
}
pub mod GetVertexAttribIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribIuiv.is_loaded }
        else { gl::GetVertexAttribIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribIuiv", &["glGetVertexAttribIuivEXT"]))
    } }
}
pub mod GetVertexAttribLdv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribLdv.is_loaded }
        else { gl::GetVertexAttribLdv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribLdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribLdv", &["glGetVertexAttribLdvEXT"]))
    } }
}
pub mod GetVertexAttribPointerv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribPointerv.is_loaded }
        else { gl::GetVertexAttribPointerv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribPointerv", &["glGetVertexAttribPointervARB", "glGetVertexAttribPointervNV"]))
    } }
}
pub mod GetVertexAttribdv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribdv.is_loaded }
        else { gl::GetVertexAttribdv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribdv", &["glGetVertexAttribdvARB", "glGetVertexAttribdvNV"]))
    } }
}
pub mod GetVertexAttribfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribfv.is_loaded }
        else { gl::GetVertexAttribfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribfv", &["glGetVertexAttribfvARB", "glGetVertexAttribfvNV"]))
    } }
}
pub mod GetVertexAttribiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetVertexAttribiv.is_loaded }
        else { gl::GetVertexAttribiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetVertexAttribiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetVertexAttribiv", &["glGetVertexAttribivARB", "glGetVertexAttribivNV"]))
    } }
}
pub mod GetnColorTable {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnColorTable.is_loaded }
        else { gl::GetnColorTable::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnColorTable = FnPtr::new(metaloadfn(&mut loadfn, "glGetnColorTable", &[]))
    } }
}
pub mod GetnCompressedTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnCompressedTexImage.is_loaded }
        else { gl::GetnCompressedTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnCompressedTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetnCompressedTexImage", &[]))
    } }
}
pub mod GetnConvolutionFilter {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnConvolutionFilter.is_loaded }
        else { gl::GetnConvolutionFilter::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnConvolutionFilter = FnPtr::new(metaloadfn(&mut loadfn, "glGetnConvolutionFilter", &[]))
    } }
}
pub mod GetnHistogram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnHistogram.is_loaded }
        else { gl::GetnHistogram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnHistogram = FnPtr::new(metaloadfn(&mut loadfn, "glGetnHistogram", &[]))
    } }
}
pub mod GetnMapdv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnMapdv.is_loaded }
        else { gl::GetnMapdv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnMapdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnMapdv", &[]))
    } }
}
pub mod GetnMapfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnMapfv.is_loaded }
        else { gl::GetnMapfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnMapfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnMapfv", &[]))
    } }
}
pub mod GetnMapiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnMapiv.is_loaded }
        else { gl::GetnMapiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnMapiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnMapiv", &[]))
    } }
}
pub mod GetnMinmax {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnMinmax.is_loaded }
        else { gl::GetnMinmax::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnMinmax = FnPtr::new(metaloadfn(&mut loadfn, "glGetnMinmax", &[]))
    } }
}
pub mod GetnPixelMapfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnPixelMapfv.is_loaded }
        else { gl::GetnPixelMapfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnPixelMapfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnPixelMapfv", &[]))
    } }
}
pub mod GetnPixelMapuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnPixelMapuiv.is_loaded }
        else { gl::GetnPixelMapuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnPixelMapuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnPixelMapuiv", &[]))
    } }
}
pub mod GetnPixelMapusv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnPixelMapusv.is_loaded }
        else { gl::GetnPixelMapusv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnPixelMapusv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnPixelMapusv", &[]))
    } }
}
pub mod GetnPolygonStipple {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnPolygonStipple.is_loaded }
        else { gl::GetnPolygonStipple::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnPolygonStipple = FnPtr::new(metaloadfn(&mut loadfn, "glGetnPolygonStipple", &[]))
    } }
}
pub mod GetnSeparableFilter {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnSeparableFilter.is_loaded }
        else { gl::GetnSeparableFilter::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnSeparableFilter = FnPtr::new(metaloadfn(&mut loadfn, "glGetnSeparableFilter", &[]))
    } }
}
pub mod GetnTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnTexImage.is_loaded }
        else { gl::GetnTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glGetnTexImage", &[]))
    } }
}
pub mod GetnUniformdv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnUniformdv.is_loaded }
        else { gl::GetnUniformdv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnUniformdv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformdv", &[]))
    } }
}
pub mod GetnUniformfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnUniformfv.is_loaded }
        else { gl::GetnUniformfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnUniformfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformfv", &["glGetnUniformfvEXT", "glGetnUniformfvKHR"]))
    } }
}
pub mod GetnUniformiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnUniformiv.is_loaded }
        else { gl::GetnUniformiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnUniformiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformiv", &["glGetnUniformivEXT", "glGetnUniformivKHR"]))
    } }
}
pub mod GetnUniformuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).GetnUniformuiv.is_loaded }
        else { gl::GetnUniformuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).GetnUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, "glGetnUniformuiv", &["glGetnUniformuivKHR"]))
    } }
}
pub mod Hint {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Hint.is_loaded }
        else { gl::Hint::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Hint = FnPtr::new(metaloadfn(&mut loadfn, "glHint", &[]))
    } }
}
pub mod InvalidateBufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateBufferData.is_loaded }
        else { gl::InvalidateBufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateBufferData = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateBufferData", &[]))
    } }
}
pub mod InvalidateBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateBufferSubData.is_loaded }
        else { gl::InvalidateBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateBufferSubData", &[]))
    } }
}
pub mod InvalidateFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateFramebuffer.is_loaded }
        else { gl::InvalidateFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateFramebuffer", &[]))
    } }
}
pub mod InvalidateNamedFramebufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateNamedFramebufferData.is_loaded }
        else { gl::InvalidateNamedFramebufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateNamedFramebufferData = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateNamedFramebufferData", &[]))
    } }
}
pub mod InvalidateNamedFramebufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateNamedFramebufferSubData.is_loaded }
        else { gl::InvalidateNamedFramebufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateNamedFramebufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateNamedFramebufferSubData", &[]))
    } }
}
pub mod InvalidateSubFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateSubFramebuffer.is_loaded }
        else { gl::InvalidateSubFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateSubFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateSubFramebuffer", &[]))
    } }
}
pub mod InvalidateTexImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateTexImage.is_loaded }
        else { gl::InvalidateTexImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateTexImage = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateTexImage", &[]))
    } }
}
pub mod InvalidateTexSubImage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).InvalidateTexSubImage.is_loaded }
        else { gl::InvalidateTexSubImage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).InvalidateTexSubImage = FnPtr::new(metaloadfn(&mut loadfn, "glInvalidateTexSubImage", &[]))
    } }
}
pub mod IsBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsBuffer.is_loaded }
        else { gl::IsBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsBuffer", &["glIsBufferARB"]))
    } }
}
pub mod IsEnabled {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsEnabled.is_loaded }
        else { gl::IsEnabled::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsEnabled = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabled", &[]))
    } }
}
pub mod IsEnabledi {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsEnabledi.is_loaded }
        else { gl::IsEnabledi::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsEnabledi = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabledi", &["glIsEnabledIndexedEXT", "glIsEnablediEXT", "glIsEnablediNV", "glIsEnablediOES"]))
    } }
}
pub mod IsFramebuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsFramebuffer.is_loaded }
        else { gl::IsFramebuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsFramebuffer", &["glIsFramebufferEXT"]))
    } }
}
pub mod IsProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsProgram.is_loaded }
        else { gl::IsProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsProgram = FnPtr::new(metaloadfn(&mut loadfn, "glIsProgram", &[]))
    } }
}
pub mod IsProgramPipeline {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsProgramPipeline.is_loaded }
        else { gl::IsProgramPipeline::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glIsProgramPipeline", &[]))
    } }
}
pub mod IsQuery {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsQuery.is_loaded }
        else { gl::IsQuery::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsQuery = FnPtr::new(metaloadfn(&mut loadfn, "glIsQuery", &["glIsQueryARB"]))
    } }
}
pub mod IsRenderbuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsRenderbuffer.is_loaded }
        else { gl::IsRenderbuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsRenderbuffer", &["glIsRenderbufferEXT"]))
    } }
}
pub mod IsSampler {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsSampler.is_loaded }
        else { gl::IsSampler::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsSampler = FnPtr::new(metaloadfn(&mut loadfn, "glIsSampler", &[]))
    } }
}
pub mod IsShader {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsShader.is_loaded }
        else { gl::IsShader::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsShader = FnPtr::new(metaloadfn(&mut loadfn, "glIsShader", &[]))
    } }
}
pub mod IsSync {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsSync.is_loaded }
        else { gl::IsSync::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsSync = FnPtr::new(metaloadfn(&mut loadfn, "glIsSync", &["glIsSyncAPPLE"]))
    } }
}
pub mod IsTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsTexture.is_loaded }
        else { gl::IsTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsTexture = FnPtr::new(metaloadfn(&mut loadfn, "glIsTexture", &[]))
    } }
}
pub mod IsTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsTransformFeedback.is_loaded }
        else { gl::IsTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glIsTransformFeedback", &["glIsTransformFeedbackNV"]))
    } }
}
pub mod IsVertexArray {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).IsVertexArray.is_loaded }
        else { gl::IsVertexArray::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).IsVertexArray = FnPtr::new(metaloadfn(&mut loadfn, "glIsVertexArray", &["glIsVertexArrayAPPLE", "glIsVertexArrayOES"]))
    } }
}
pub mod LineWidth {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).LineWidth.is_loaded }
        else { gl::LineWidth::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).LineWidth = FnPtr::new(metaloadfn(&mut loadfn, "glLineWidth", &[]))
    } }
}
pub mod LinkProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).LinkProgram.is_loaded }
        else { gl::LinkProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).LinkProgram = FnPtr::new(metaloadfn(&mut loadfn, "glLinkProgram", &["glLinkProgramARB"]))
    } }
}
pub mod LogicOp {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).LogicOp.is_loaded }
        else { gl::LogicOp::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).LogicOp = FnPtr::new(metaloadfn(&mut loadfn, "glLogicOp", &[]))
    } }
}
pub mod MapBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MapBuffer.is_loaded }
        else { gl::MapBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MapBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glMapBuffer", &["glMapBufferARB", "glMapBufferOES"]))
    } }
}
pub mod MapBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MapBufferRange.is_loaded }
        else { gl::MapBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MapBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glMapBufferRange", &["glMapBufferRangeEXT"]))
    } }
}
pub mod MapNamedBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MapNamedBuffer.is_loaded }
        else { gl::MapNamedBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MapNamedBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glMapNamedBuffer", &[]))
    } }
}
pub mod MapNamedBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MapNamedBufferRange.is_loaded }
        else { gl::MapNamedBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MapNamedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glMapNamedBufferRange", &[]))
    } }
}
pub mod MemoryBarrier {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MemoryBarrier.is_loaded }
        else { gl::MemoryBarrier::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MemoryBarrier = FnPtr::new(metaloadfn(&mut loadfn, "glMemoryBarrier", &["glMemoryBarrierEXT"]))
    } }
}
pub mod MemoryBarrierByRegion {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MemoryBarrierByRegion.is_loaded }
        else { gl::MemoryBarrierByRegion::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MemoryBarrierByRegion = FnPtr::new(metaloadfn(&mut loadfn, "glMemoryBarrierByRegion", &[]))
    } }
}
pub mod MinSampleShading {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MinSampleShading.is_loaded }
        else { gl::MinSampleShading::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MinSampleShading = FnPtr::new(metaloadfn(&mut loadfn, "glMinSampleShading", &["glMinSampleShadingARB", "glMinSampleShadingOES"]))
    } }
}
pub mod MultiDrawArrays {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiDrawArrays.is_loaded }
        else { gl::MultiDrawArrays::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiDrawArrays = FnPtr::new(metaloadfn(&mut loadfn, "glMultiDrawArrays", &["glMultiDrawArraysEXT"]))
    } }
}
pub mod MultiDrawArraysIndirect {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiDrawArraysIndirect.is_loaded }
        else { gl::MultiDrawArraysIndirect::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiDrawArraysIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glMultiDrawArraysIndirect", &["glMultiDrawArraysIndirectAMD", "glMultiDrawArraysIndirectEXT"]))
    } }
}
pub mod MultiDrawElements {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiDrawElements.is_loaded }
        else { gl::MultiDrawElements::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiDrawElements = FnPtr::new(metaloadfn(&mut loadfn, "glMultiDrawElements", &["glMultiDrawElementsEXT"]))
    } }
}
pub mod MultiDrawElementsBaseVertex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiDrawElementsBaseVertex.is_loaded }
        else { gl::MultiDrawElementsBaseVertex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiDrawElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, "glMultiDrawElementsBaseVertex", &["glMultiDrawElementsBaseVertexEXT"]))
    } }
}
pub mod MultiDrawElementsIndirect {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiDrawElementsIndirect.is_loaded }
        else { gl::MultiDrawElementsIndirect::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiDrawElementsIndirect = FnPtr::new(metaloadfn(&mut loadfn, "glMultiDrawElementsIndirect", &["glMultiDrawElementsIndirectAMD", "glMultiDrawElementsIndirectEXT"]))
    } }
}
pub mod MultiTexCoordP1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP1ui.is_loaded }
        else { gl::MultiTexCoordP1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP1ui = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP1ui", &[]))
    } }
}
pub mod MultiTexCoordP1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP1uiv.is_loaded }
        else { gl::MultiTexCoordP1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP1uiv", &[]))
    } }
}
pub mod MultiTexCoordP2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP2ui.is_loaded }
        else { gl::MultiTexCoordP2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP2ui = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP2ui", &[]))
    } }
}
pub mod MultiTexCoordP2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP2uiv.is_loaded }
        else { gl::MultiTexCoordP2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP2uiv", &[]))
    } }
}
pub mod MultiTexCoordP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP3ui.is_loaded }
        else { gl::MultiTexCoordP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP3ui", &[]))
    } }
}
pub mod MultiTexCoordP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP3uiv.is_loaded }
        else { gl::MultiTexCoordP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP3uiv", &[]))
    } }
}
pub mod MultiTexCoordP4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP4ui.is_loaded }
        else { gl::MultiTexCoordP4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP4ui = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP4ui", &[]))
    } }
}
pub mod MultiTexCoordP4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).MultiTexCoordP4uiv.is_loaded }
        else { gl::MultiTexCoordP4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).MultiTexCoordP4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoordP4uiv", &[]))
    } }
}
pub mod NamedBufferData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedBufferData.is_loaded }
        else { gl::NamedBufferData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedBufferData = FnPtr::new(metaloadfn(&mut loadfn, "glNamedBufferData", &[]))
    } }
}
pub mod NamedBufferStorage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedBufferStorage.is_loaded }
        else { gl::NamedBufferStorage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedBufferStorage = FnPtr::new(metaloadfn(&mut loadfn, "glNamedBufferStorage", &["glNamedBufferStorageEXT"]))
    } }
}
pub mod NamedBufferSubData {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedBufferSubData.is_loaded }
        else { gl::NamedBufferSubData::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glNamedBufferSubData", &["glNamedBufferSubDataEXT"]))
    } }
}
pub mod NamedFramebufferDrawBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferDrawBuffer.is_loaded }
        else { gl::NamedFramebufferDrawBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferDrawBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferDrawBuffer", &[]))
    } }
}
pub mod NamedFramebufferDrawBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferDrawBuffers.is_loaded }
        else { gl::NamedFramebufferDrawBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferDrawBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferDrawBuffers", &[]))
    } }
}
pub mod NamedFramebufferParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferParameteri.is_loaded }
        else { gl::NamedFramebufferParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferParameteri", &[]))
    } }
}
pub mod NamedFramebufferReadBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferReadBuffer.is_loaded }
        else { gl::NamedFramebufferReadBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferReadBuffer", &[]))
    } }
}
pub mod NamedFramebufferRenderbuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferRenderbuffer.is_loaded }
        else { gl::NamedFramebufferRenderbuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferRenderbuffer", &[]))
    } }
}
pub mod NamedFramebufferTexture {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferTexture.is_loaded }
        else { gl::NamedFramebufferTexture::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferTexture = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferTexture", &[]))
    } }
}
pub mod NamedFramebufferTextureLayer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedFramebufferTextureLayer.is_loaded }
        else { gl::NamedFramebufferTextureLayer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedFramebufferTextureLayer = FnPtr::new(metaloadfn(&mut loadfn, "glNamedFramebufferTextureLayer", &[]))
    } }
}
pub mod NamedRenderbufferStorage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedRenderbufferStorage.is_loaded }
        else { gl::NamedRenderbufferStorage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedRenderbufferStorage = FnPtr::new(metaloadfn(&mut loadfn, "glNamedRenderbufferStorage", &[]))
    } }
}
pub mod NamedRenderbufferStorageMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NamedRenderbufferStorageMultisample.is_loaded }
        else { gl::NamedRenderbufferStorageMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NamedRenderbufferStorageMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glNamedRenderbufferStorageMultisample", &[]))
    } }
}
pub mod NormalP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NormalP3ui.is_loaded }
        else { gl::NormalP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NormalP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glNormalP3ui", &[]))
    } }
}
pub mod NormalP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).NormalP3uiv.is_loaded }
        else { gl::NormalP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).NormalP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glNormalP3uiv", &[]))
    } }
}
pub mod ObjectLabel {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ObjectLabel.is_loaded }
        else { gl::ObjectLabel::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, "glObjectLabel", &["glObjectLabelKHR"]))
    } }
}
pub mod ObjectPtrLabel {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ObjectPtrLabel.is_loaded }
        else { gl::ObjectPtrLabel::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, "glObjectPtrLabel", &["glObjectPtrLabelKHR"]))
    } }
}
pub mod PatchParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PatchParameterfv.is_loaded }
        else { gl::PatchParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PatchParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glPatchParameterfv", &[]))
    } }
}
pub mod PatchParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PatchParameteri.is_loaded }
        else { gl::PatchParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PatchParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glPatchParameteri", &["glPatchParameteriEXT", "glPatchParameteriOES"]))
    } }
}
pub mod PauseTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PauseTransformFeedback.is_loaded }
        else { gl::PauseTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PauseTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glPauseTransformFeedback", &["glPauseTransformFeedbackNV"]))
    } }
}
pub mod PixelStoref {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PixelStoref.is_loaded }
        else { gl::PixelStoref::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PixelStoref = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStoref", &[]))
    } }
}
pub mod PixelStorei {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PixelStorei.is_loaded }
        else { gl::PixelStorei::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PixelStorei = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStorei", &[]))
    } }
}
pub mod PointParameterf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PointParameterf.is_loaded }
        else { gl::PointParameterf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PointParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterf", &["glPointParameterfARB", "glPointParameterfEXT", "glPointParameterfSGIS"]))
    } }
}
pub mod PointParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PointParameterfv.is_loaded }
        else { gl::PointParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PointParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterfv", &["glPointParameterfvARB", "glPointParameterfvEXT", "glPointParameterfvSGIS"]))
    } }
}
pub mod PointParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PointParameteri.is_loaded }
        else { gl::PointParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PointParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameteri", &["glPointParameteriNV"]))
    } }
}
pub mod PointParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PointParameteriv.is_loaded }
        else { gl::PointParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PointParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameteriv", &["glPointParameterivNV"]))
    } }
}
pub mod PointSize {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PointSize.is_loaded }
        else { gl::PointSize::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PointSize = FnPtr::new(metaloadfn(&mut loadfn, "glPointSize", &[]))
    } }
}
pub mod PolygonMode {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PolygonMode.is_loaded }
        else { gl::PolygonMode::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PolygonMode = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonMode", &["glPolygonModeNV"]))
    } }
}
pub mod PolygonOffset {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PolygonOffset.is_loaded }
        else { gl::PolygonOffset::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PolygonOffset = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonOffset", &[]))
    } }
}
pub mod PopDebugGroup {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PopDebugGroup.is_loaded }
        else { gl::PopDebugGroup::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PopDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, "glPopDebugGroup", &["glPopDebugGroupKHR"]))
    } }
}
pub mod PrimitiveRestartIndex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PrimitiveRestartIndex.is_loaded }
        else { gl::PrimitiveRestartIndex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PrimitiveRestartIndex = FnPtr::new(metaloadfn(&mut loadfn, "glPrimitiveRestartIndex", &[]))
    } }
}
pub mod ProgramBinary {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramBinary.is_loaded }
        else { gl::ProgramBinary::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, "glProgramBinary", &["glProgramBinaryOES"]))
    } }
}
pub mod ProgramParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramParameteri.is_loaded }
        else { gl::ProgramParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glProgramParameteri", &["glProgramParameteriARB", "glProgramParameteriEXT"]))
    } }
}
pub mod ProgramUniform1d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1d.is_loaded }
        else { gl::ProgramUniform1d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1d = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1d", &[]))
    } }
}
pub mod ProgramUniform1dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1dv.is_loaded }
        else { gl::ProgramUniform1dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1dv", &[]))
    } }
}
pub mod ProgramUniform1f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1f.is_loaded }
        else { gl::ProgramUniform1f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1f", &["glProgramUniform1fEXT"]))
    } }
}
pub mod ProgramUniform1fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1fv.is_loaded }
        else { gl::ProgramUniform1fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1fv", &["glProgramUniform1fvEXT"]))
    } }
}
pub mod ProgramUniform1i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1i.is_loaded }
        else { gl::ProgramUniform1i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1i", &["glProgramUniform1iEXT"]))
    } }
}
pub mod ProgramUniform1iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1iv.is_loaded }
        else { gl::ProgramUniform1iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1iv", &["glProgramUniform1ivEXT"]))
    } }
}
pub mod ProgramUniform1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1ui.is_loaded }
        else { gl::ProgramUniform1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1ui", &["glProgramUniform1uiEXT"]))
    } }
}
pub mod ProgramUniform1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform1uiv.is_loaded }
        else { gl::ProgramUniform1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform1uiv", &["glProgramUniform1uivEXT"]))
    } }
}
pub mod ProgramUniform2d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2d.is_loaded }
        else { gl::ProgramUniform2d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2d = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2d", &[]))
    } }
}
pub mod ProgramUniform2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2dv.is_loaded }
        else { gl::ProgramUniform2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2dv", &[]))
    } }
}
pub mod ProgramUniform2f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2f.is_loaded }
        else { gl::ProgramUniform2f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2f", &["glProgramUniform2fEXT"]))
    } }
}
pub mod ProgramUniform2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2fv.is_loaded }
        else { gl::ProgramUniform2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2fv", &["glProgramUniform2fvEXT"]))
    } }
}
pub mod ProgramUniform2i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2i.is_loaded }
        else { gl::ProgramUniform2i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2i", &["glProgramUniform2iEXT"]))
    } }
}
pub mod ProgramUniform2iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2iv.is_loaded }
        else { gl::ProgramUniform2iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2iv", &["glProgramUniform2ivEXT"]))
    } }
}
pub mod ProgramUniform2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2ui.is_loaded }
        else { gl::ProgramUniform2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2ui", &["glProgramUniform2uiEXT"]))
    } }
}
pub mod ProgramUniform2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform2uiv.is_loaded }
        else { gl::ProgramUniform2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform2uiv", &["glProgramUniform2uivEXT"]))
    } }
}
pub mod ProgramUniform3d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3d.is_loaded }
        else { gl::ProgramUniform3d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3d = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3d", &[]))
    } }
}
pub mod ProgramUniform3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3dv.is_loaded }
        else { gl::ProgramUniform3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3dv", &[]))
    } }
}
pub mod ProgramUniform3f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3f.is_loaded }
        else { gl::ProgramUniform3f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3f", &["glProgramUniform3fEXT"]))
    } }
}
pub mod ProgramUniform3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3fv.is_loaded }
        else { gl::ProgramUniform3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3fv", &["glProgramUniform3fvEXT"]))
    } }
}
pub mod ProgramUniform3i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3i.is_loaded }
        else { gl::ProgramUniform3i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3i", &["glProgramUniform3iEXT"]))
    } }
}
pub mod ProgramUniform3iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3iv.is_loaded }
        else { gl::ProgramUniform3iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3iv", &["glProgramUniform3ivEXT"]))
    } }
}
pub mod ProgramUniform3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3ui.is_loaded }
        else { gl::ProgramUniform3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3ui", &["glProgramUniform3uiEXT"]))
    } }
}
pub mod ProgramUniform3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform3uiv.is_loaded }
        else { gl::ProgramUniform3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform3uiv", &["glProgramUniform3uivEXT"]))
    } }
}
pub mod ProgramUniform4d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4d.is_loaded }
        else { gl::ProgramUniform4d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4d = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4d", &[]))
    } }
}
pub mod ProgramUniform4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4dv.is_loaded }
        else { gl::ProgramUniform4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4dv", &[]))
    } }
}
pub mod ProgramUniform4f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4f.is_loaded }
        else { gl::ProgramUniform4f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4f = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4f", &["glProgramUniform4fEXT"]))
    } }
}
pub mod ProgramUniform4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4fv.is_loaded }
        else { gl::ProgramUniform4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4fv", &["glProgramUniform4fvEXT"]))
    } }
}
pub mod ProgramUniform4i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4i.is_loaded }
        else { gl::ProgramUniform4i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4i = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4i", &["glProgramUniform4iEXT"]))
    } }
}
pub mod ProgramUniform4iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4iv.is_loaded }
        else { gl::ProgramUniform4iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4iv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4iv", &["glProgramUniform4ivEXT"]))
    } }
}
pub mod ProgramUniform4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4ui.is_loaded }
        else { gl::ProgramUniform4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4ui = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4ui", &["glProgramUniform4uiEXT"]))
    } }
}
pub mod ProgramUniform4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniform4uiv.is_loaded }
        else { gl::ProgramUniform4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniform4uiv", &["glProgramUniform4uivEXT"]))
    } }
}
pub mod ProgramUniformMatrix2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2dv.is_loaded }
        else { gl::ProgramUniformMatrix2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2dv", &[]))
    } }
}
pub mod ProgramUniformMatrix2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2fv.is_loaded }
        else { gl::ProgramUniformMatrix2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2fv", &["glProgramUniformMatrix2fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix2x3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2x3dv.is_loaded }
        else { gl::ProgramUniformMatrix2x3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2x3dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x3dv", &[]))
    } }
}
pub mod ProgramUniformMatrix2x3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2x3fv.is_loaded }
        else { gl::ProgramUniformMatrix2x3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x3fv", &["glProgramUniformMatrix2x3fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix2x4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2x4dv.is_loaded }
        else { gl::ProgramUniformMatrix2x4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2x4dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x4dv", &[]))
    } }
}
pub mod ProgramUniformMatrix2x4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix2x4fv.is_loaded }
        else { gl::ProgramUniformMatrix2x4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix2x4fv", &["glProgramUniformMatrix2x4fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3dv.is_loaded }
        else { gl::ProgramUniformMatrix3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3dv", &[]))
    } }
}
pub mod ProgramUniformMatrix3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3fv.is_loaded }
        else { gl::ProgramUniformMatrix3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3fv", &["glProgramUniformMatrix3fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix3x2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3x2dv.is_loaded }
        else { gl::ProgramUniformMatrix3x2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3x2dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x2dv", &[]))
    } }
}
pub mod ProgramUniformMatrix3x2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3x2fv.is_loaded }
        else { gl::ProgramUniformMatrix3x2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x2fv", &["glProgramUniformMatrix3x2fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix3x4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3x4dv.is_loaded }
        else { gl::ProgramUniformMatrix3x4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3x4dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x4dv", &[]))
    } }
}
pub mod ProgramUniformMatrix3x4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix3x4fv.is_loaded }
        else { gl::ProgramUniformMatrix3x4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix3x4fv", &["glProgramUniformMatrix3x4fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4dv.is_loaded }
        else { gl::ProgramUniformMatrix4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4dv", &[]))
    } }
}
pub mod ProgramUniformMatrix4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4fv.is_loaded }
        else { gl::ProgramUniformMatrix4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4fv", &["glProgramUniformMatrix4fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix4x2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4x2dv.is_loaded }
        else { gl::ProgramUniformMatrix4x2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4x2dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x2dv", &[]))
    } }
}
pub mod ProgramUniformMatrix4x2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4x2fv.is_loaded }
        else { gl::ProgramUniformMatrix4x2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x2fv", &["glProgramUniformMatrix4x2fvEXT"]))
    } }
}
pub mod ProgramUniformMatrix4x3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4x3dv.is_loaded }
        else { gl::ProgramUniformMatrix4x3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4x3dv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x3dv", &[]))
    } }
}
pub mod ProgramUniformMatrix4x3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProgramUniformMatrix4x3fv.is_loaded }
        else { gl::ProgramUniformMatrix4x3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProgramUniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glProgramUniformMatrix4x3fv", &["glProgramUniformMatrix4x3fvEXT"]))
    } }
}
pub mod ProvokingVertex {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ProvokingVertex.is_loaded }
        else { gl::ProvokingVertex::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ProvokingVertex = FnPtr::new(metaloadfn(&mut loadfn, "glProvokingVertex", &["glProvokingVertexEXT"]))
    } }
}
pub mod PushDebugGroup {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).PushDebugGroup.is_loaded }
        else { gl::PushDebugGroup::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).PushDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, "glPushDebugGroup", &["glPushDebugGroupKHR"]))
    } }
}
pub mod QueryCounter {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).QueryCounter.is_loaded }
        else { gl::QueryCounter::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).QueryCounter = FnPtr::new(metaloadfn(&mut loadfn, "glQueryCounter", &["glQueryCounterEXT"]))
    } }
}
pub mod ReadBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ReadBuffer.is_loaded }
        else { gl::ReadBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glReadBuffer", &[]))
    } }
}
pub mod ReadPixels {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ReadPixels.is_loaded }
        else { gl::ReadPixels::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ReadPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadPixels", &[]))
    } }
}
pub mod ReadnPixels {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ReadnPixels.is_loaded }
        else { gl::ReadnPixels::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ReadnPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadnPixels", &["glReadnPixelsARB", "glReadnPixelsEXT", "glReadnPixelsKHR"]))
    } }
}
pub mod ReleaseShaderCompiler {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ReleaseShaderCompiler.is_loaded }
        else { gl::ReleaseShaderCompiler::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ReleaseShaderCompiler = FnPtr::new(metaloadfn(&mut loadfn, "glReleaseShaderCompiler", &[]))
    } }
}
pub mod RenderbufferStorage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).RenderbufferStorage.is_loaded }
        else { gl::RenderbufferStorage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).RenderbufferStorage = FnPtr::new(metaloadfn(&mut loadfn, "glRenderbufferStorage", &["glRenderbufferStorageEXT"]))
    } }
}
pub mod RenderbufferStorageMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).RenderbufferStorageMultisample.is_loaded }
        else { gl::RenderbufferStorageMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).RenderbufferStorageMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glRenderbufferStorageMultisample", &["glRenderbufferStorageMultisampleEXT", "glRenderbufferStorageMultisampleNV"]))
    } }
}
pub mod ResumeTransformFeedback {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ResumeTransformFeedback.is_loaded }
        else { gl::ResumeTransformFeedback::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ResumeTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, "glResumeTransformFeedback", &["glResumeTransformFeedbackNV"]))
    } }
}
pub mod SampleCoverage {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SampleCoverage.is_loaded }
        else { gl::SampleCoverage::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SampleCoverage = FnPtr::new(metaloadfn(&mut loadfn, "glSampleCoverage", &["glSampleCoverageARB"]))
    } }
}
pub mod SampleMaski {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SampleMaski.is_loaded }
        else { gl::SampleMaski::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SampleMaski = FnPtr::new(metaloadfn(&mut loadfn, "glSampleMaski", &[]))
    } }
}
pub mod SamplerParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameterIiv.is_loaded }
        else { gl::SamplerParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterIiv", &["glSamplerParameterIivEXT", "glSamplerParameterIivOES"]))
    } }
}
pub mod SamplerParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameterIuiv.is_loaded }
        else { gl::SamplerParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterIuiv", &["glSamplerParameterIuivEXT", "glSamplerParameterIuivOES"]))
    } }
}
pub mod SamplerParameterf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameterf.is_loaded }
        else { gl::SamplerParameterf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterf", &[]))
    } }
}
pub mod SamplerParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameterfv.is_loaded }
        else { gl::SamplerParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameterfv", &[]))
    } }
}
pub mod SamplerParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameteri.is_loaded }
        else { gl::SamplerParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameteri", &[]))
    } }
}
pub mod SamplerParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SamplerParameteriv.is_loaded }
        else { gl::SamplerParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glSamplerParameteriv", &[]))
    } }
}
pub mod Scissor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Scissor.is_loaded }
        else { gl::Scissor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Scissor = FnPtr::new(metaloadfn(&mut loadfn, "glScissor", &[]))
    } }
}
pub mod ScissorArrayv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ScissorArrayv.is_loaded }
        else { gl::ScissorArrayv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ScissorArrayv = FnPtr::new(metaloadfn(&mut loadfn, "glScissorArrayv", &["glScissorArrayvNV", "glScissorArrayvOES"]))
    } }
}
pub mod ScissorIndexed {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ScissorIndexed.is_loaded }
        else { gl::ScissorIndexed::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ScissorIndexed = FnPtr::new(metaloadfn(&mut loadfn, "glScissorIndexed", &["glScissorIndexedNV", "glScissorIndexedOES"]))
    } }
}
pub mod ScissorIndexedv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ScissorIndexedv.is_loaded }
        else { gl::ScissorIndexedv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ScissorIndexedv = FnPtr::new(metaloadfn(&mut loadfn, "glScissorIndexedv", &["glScissorIndexedvNV", "glScissorIndexedvOES"]))
    } }
}
pub mod SecondaryColorP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SecondaryColorP3ui.is_loaded }
        else { gl::SecondaryColorP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SecondaryColorP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glSecondaryColorP3ui", &[]))
    } }
}
pub mod SecondaryColorP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).SecondaryColorP3uiv.is_loaded }
        else { gl::SecondaryColorP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).SecondaryColorP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glSecondaryColorP3uiv", &[]))
    } }
}
pub mod ShaderBinary {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ShaderBinary.is_loaded }
        else { gl::ShaderBinary::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ShaderBinary = FnPtr::new(metaloadfn(&mut loadfn, "glShaderBinary", &[]))
    } }
}
pub mod ShaderSource {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ShaderSource.is_loaded }
        else { gl::ShaderSource::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ShaderSource = FnPtr::new(metaloadfn(&mut loadfn, "glShaderSource", &["glShaderSourceARB"]))
    } }
}
pub mod ShaderStorageBlockBinding {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ShaderStorageBlockBinding.is_loaded }
        else { gl::ShaderStorageBlockBinding::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ShaderStorageBlockBinding = FnPtr::new(metaloadfn(&mut loadfn, "glShaderStorageBlockBinding", &[]))
    } }
}
pub mod StencilFunc {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilFunc.is_loaded }
        else { gl::StencilFunc::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilFunc = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFunc", &[]))
    } }
}
pub mod StencilFuncSeparate {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilFuncSeparate.is_loaded }
        else { gl::StencilFuncSeparate::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFuncSeparate", &[]))
    } }
}
pub mod StencilMask {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilMask.is_loaded }
        else { gl::StencilMask::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilMask = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMask", &[]))
    } }
}
pub mod StencilMaskSeparate {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilMaskSeparate.is_loaded }
        else { gl::StencilMaskSeparate::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilMaskSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMaskSeparate", &[]))
    } }
}
pub mod StencilOp {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilOp.is_loaded }
        else { gl::StencilOp::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilOp = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOp", &[]))
    } }
}
pub mod StencilOpSeparate {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).StencilOpSeparate.is_loaded }
        else { gl::StencilOpSeparate::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).StencilOpSeparate = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOpSeparate", &["glStencilOpSeparateATI"]))
    } }
}
pub mod TexBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexBuffer.is_loaded }
        else { gl::TexBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glTexBuffer", &["glTexBufferARB", "glTexBufferEXT", "glTexBufferOES"]))
    } }
}
pub mod TexBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexBufferRange.is_loaded }
        else { gl::TexBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glTexBufferRange", &["glTexBufferRangeEXT", "glTexBufferRangeOES"]))
    } }
}
pub mod TexCoordP1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP1ui.is_loaded }
        else { gl::TexCoordP1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP1ui = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP1ui", &[]))
    } }
}
pub mod TexCoordP1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP1uiv.is_loaded }
        else { gl::TexCoordP1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP1uiv", &[]))
    } }
}
pub mod TexCoordP2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP2ui.is_loaded }
        else { gl::TexCoordP2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP2ui = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP2ui", &[]))
    } }
}
pub mod TexCoordP2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP2uiv.is_loaded }
        else { gl::TexCoordP2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP2uiv", &[]))
    } }
}
pub mod TexCoordP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP3ui.is_loaded }
        else { gl::TexCoordP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP3ui", &[]))
    } }
}
pub mod TexCoordP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP3uiv.is_loaded }
        else { gl::TexCoordP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP3uiv", &[]))
    } }
}
pub mod TexCoordP4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP4ui.is_loaded }
        else { gl::TexCoordP4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP4ui = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP4ui", &[]))
    } }
}
pub mod TexCoordP4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexCoordP4uiv.is_loaded }
        else { gl::TexCoordP4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexCoordP4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordP4uiv", &[]))
    } }
}
pub mod TexImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexImage1D.is_loaded }
        else { gl::TexImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage1D", &[]))
    } }
}
pub mod TexImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexImage2D.is_loaded }
        else { gl::TexImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage2D", &[]))
    } }
}
pub mod TexImage2DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexImage2DMultisample.is_loaded }
        else { gl::TexImage2DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexImage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage2DMultisample", &[]))
    } }
}
pub mod TexImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexImage3D.is_loaded }
        else { gl::TexImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage3D", &["glTexImage3DEXT"]))
    } }
}
pub mod TexImage3DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexImage3DMultisample.is_loaded }
        else { gl::TexImage3DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexImage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage3DMultisample", &[]))
    } }
}
pub mod TexParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameterIiv.is_loaded }
        else { gl::TexParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterIiv", &["glTexParameterIivEXT", "glTexParameterIivOES"]))
    } }
}
pub mod TexParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameterIuiv.is_loaded }
        else { gl::TexParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterIuiv", &["glTexParameterIuivEXT", "glTexParameterIuivOES"]))
    } }
}
pub mod TexParameterf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameterf.is_loaded }
        else { gl::TexParameterf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterf", &[]))
    } }
}
pub mod TexParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameterfv.is_loaded }
        else { gl::TexParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterfv", &[]))
    } }
}
pub mod TexParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameteri.is_loaded }
        else { gl::TexParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteri", &[]))
    } }
}
pub mod TexParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexParameteriv.is_loaded }
        else { gl::TexParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteriv", &[]))
    } }
}
pub mod TexStorage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexStorage1D.is_loaded }
        else { gl::TexStorage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexStorage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage1D", &["glTexStorage1DEXT"]))
    } }
}
pub mod TexStorage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexStorage2D.is_loaded }
        else { gl::TexStorage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexStorage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage2D", &["glTexStorage2DEXT"]))
    } }
}
pub mod TexStorage2DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexStorage2DMultisample.is_loaded }
        else { gl::TexStorage2DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexStorage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage2DMultisample", &[]))
    } }
}
pub mod TexStorage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexStorage3D.is_loaded }
        else { gl::TexStorage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexStorage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage3D", &["glTexStorage3DEXT"]))
    } }
}
pub mod TexStorage3DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexStorage3DMultisample.is_loaded }
        else { gl::TexStorage3DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexStorage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTexStorage3DMultisample", &["glTexStorage3DMultisampleOES"]))
    } }
}
pub mod TexSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexSubImage1D.is_loaded }
        else { gl::TexSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage1D", &["glTexSubImage1DEXT"]))
    } }
}
pub mod TexSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexSubImage2D.is_loaded }
        else { gl::TexSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage2D", &["glTexSubImage2DEXT"]))
    } }
}
pub mod TexSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TexSubImage3D.is_loaded }
        else { gl::TexSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage3D", &["glTexSubImage3DEXT"]))
    } }
}
pub mod TextureBarrier {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureBarrier.is_loaded }
        else { gl::TextureBarrier::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureBarrier = FnPtr::new(metaloadfn(&mut loadfn, "glTextureBarrier", &[]))
    } }
}
pub mod TextureBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureBuffer.is_loaded }
        else { gl::TextureBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glTextureBuffer", &[]))
    } }
}
pub mod TextureBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureBufferRange.is_loaded }
        else { gl::TextureBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glTextureBufferRange", &[]))
    } }
}
pub mod TextureParameterIiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameterIiv.is_loaded }
        else { gl::TextureParameterIiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameterIiv", &[]))
    } }
}
pub mod TextureParameterIuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameterIuiv.is_loaded }
        else { gl::TextureParameterIuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameterIuiv", &[]))
    } }
}
pub mod TextureParameterf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameterf.is_loaded }
        else { gl::TextureParameterf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameterf", &[]))
    } }
}
pub mod TextureParameterfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameterfv.is_loaded }
        else { gl::TextureParameterfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameterfv", &[]))
    } }
}
pub mod TextureParameteri {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameteri.is_loaded }
        else { gl::TextureParameteri::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameteri", &[]))
    } }
}
pub mod TextureParameteriv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureParameteriv.is_loaded }
        else { gl::TextureParameteriv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glTextureParameteriv", &[]))
    } }
}
pub mod TextureStorage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureStorage1D.is_loaded }
        else { gl::TextureStorage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureStorage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureStorage1D", &[]))
    } }
}
pub mod TextureStorage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureStorage2D.is_loaded }
        else { gl::TextureStorage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureStorage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureStorage2D", &[]))
    } }
}
pub mod TextureStorage2DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureStorage2DMultisample.is_loaded }
        else { gl::TextureStorage2DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureStorage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTextureStorage2DMultisample", &[]))
    } }
}
pub mod TextureStorage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureStorage3D.is_loaded }
        else { gl::TextureStorage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureStorage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureStorage3D", &[]))
    } }
}
pub mod TextureStorage3DMultisample {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureStorage3DMultisample.is_loaded }
        else { gl::TextureStorage3DMultisample::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureStorage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, "glTextureStorage3DMultisample", &[]))
    } }
}
pub mod TextureSubImage1D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureSubImage1D.is_loaded }
        else { gl::TextureSubImage1D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureSubImage1D", &[]))
    } }
}
pub mod TextureSubImage2D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureSubImage2D.is_loaded }
        else { gl::TextureSubImage2D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureSubImage2D", &[]))
    } }
}
pub mod TextureSubImage3D {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureSubImage3D.is_loaded }
        else { gl::TextureSubImage3D::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, "glTextureSubImage3D", &[]))
    } }
}
pub mod TextureView {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TextureView.is_loaded }
        else { gl::TextureView::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TextureView = FnPtr::new(metaloadfn(&mut loadfn, "glTextureView", &["glTextureViewEXT", "glTextureViewOES"]))
    } }
}
pub mod TransformFeedbackBufferBase {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TransformFeedbackBufferBase.is_loaded }
        else { gl::TransformFeedbackBufferBase::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TransformFeedbackBufferBase = FnPtr::new(metaloadfn(&mut loadfn, "glTransformFeedbackBufferBase", &[]))
    } }
}
pub mod TransformFeedbackBufferRange {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TransformFeedbackBufferRange.is_loaded }
        else { gl::TransformFeedbackBufferRange::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TransformFeedbackBufferRange = FnPtr::new(metaloadfn(&mut loadfn, "glTransformFeedbackBufferRange", &[]))
    } }
}
pub mod TransformFeedbackVaryings {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).TransformFeedbackVaryings.is_loaded }
        else { gl::TransformFeedbackVaryings::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).TransformFeedbackVaryings = FnPtr::new(metaloadfn(&mut loadfn, "glTransformFeedbackVaryings", &["glTransformFeedbackVaryingsEXT"]))
    } }
}
pub mod Uniform1d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1d.is_loaded }
        else { gl::Uniform1d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1d = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1d", &[]))
    } }
}
pub mod Uniform1dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1dv.is_loaded }
        else { gl::Uniform1dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1dv", &[]))
    } }
}
pub mod Uniform1f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1f.is_loaded }
        else { gl::Uniform1f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1f", &["glUniform1fARB"]))
    } }
}
pub mod Uniform1fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1fv.is_loaded }
        else { gl::Uniform1fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1fv", &["glUniform1fvARB"]))
    } }
}
pub mod Uniform1i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1i.is_loaded }
        else { gl::Uniform1i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1i", &["glUniform1iARB"]))
    } }
}
pub mod Uniform1iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1iv.is_loaded }
        else { gl::Uniform1iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1iv", &["glUniform1ivARB"]))
    } }
}
pub mod Uniform1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1ui.is_loaded }
        else { gl::Uniform1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1ui", &["glUniform1uiEXT"]))
    } }
}
pub mod Uniform1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform1uiv.is_loaded }
        else { gl::Uniform1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform1uiv", &["glUniform1uivEXT"]))
    } }
}
pub mod Uniform2d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2d.is_loaded }
        else { gl::Uniform2d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2d = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2d", &[]))
    } }
}
pub mod Uniform2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2dv.is_loaded }
        else { gl::Uniform2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2dv", &[]))
    } }
}
pub mod Uniform2f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2f.is_loaded }
        else { gl::Uniform2f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2f", &["glUniform2fARB"]))
    } }
}
pub mod Uniform2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2fv.is_loaded }
        else { gl::Uniform2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2fv", &["glUniform2fvARB"]))
    } }
}
pub mod Uniform2i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2i.is_loaded }
        else { gl::Uniform2i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2i", &["glUniform2iARB"]))
    } }
}
pub mod Uniform2iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2iv.is_loaded }
        else { gl::Uniform2iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2iv", &["glUniform2ivARB"]))
    } }
}
pub mod Uniform2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2ui.is_loaded }
        else { gl::Uniform2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2ui", &["glUniform2uiEXT"]))
    } }
}
pub mod Uniform2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform2uiv.is_loaded }
        else { gl::Uniform2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform2uiv", &["glUniform2uivEXT"]))
    } }
}
pub mod Uniform3d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3d.is_loaded }
        else { gl::Uniform3d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3d = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3d", &[]))
    } }
}
pub mod Uniform3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3dv.is_loaded }
        else { gl::Uniform3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3dv", &[]))
    } }
}
pub mod Uniform3f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3f.is_loaded }
        else { gl::Uniform3f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3f", &["glUniform3fARB"]))
    } }
}
pub mod Uniform3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3fv.is_loaded }
        else { gl::Uniform3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3fv", &["glUniform3fvARB"]))
    } }
}
pub mod Uniform3i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3i.is_loaded }
        else { gl::Uniform3i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3i", &["glUniform3iARB"]))
    } }
}
pub mod Uniform3iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3iv.is_loaded }
        else { gl::Uniform3iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3iv", &["glUniform3ivARB"]))
    } }
}
pub mod Uniform3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3ui.is_loaded }
        else { gl::Uniform3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3ui", &["glUniform3uiEXT"]))
    } }
}
pub mod Uniform3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform3uiv.is_loaded }
        else { gl::Uniform3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform3uiv", &["glUniform3uivEXT"]))
    } }
}
pub mod Uniform4d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4d.is_loaded }
        else { gl::Uniform4d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4d = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4d", &[]))
    } }
}
pub mod Uniform4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4dv.is_loaded }
        else { gl::Uniform4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4dv", &[]))
    } }
}
pub mod Uniform4f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4f.is_loaded }
        else { gl::Uniform4f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4f = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4f", &["glUniform4fARB"]))
    } }
}
pub mod Uniform4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4fv.is_loaded }
        else { gl::Uniform4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4fv", &["glUniform4fvARB"]))
    } }
}
pub mod Uniform4i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4i.is_loaded }
        else { gl::Uniform4i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4i = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4i", &["glUniform4iARB"]))
    } }
}
pub mod Uniform4iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4iv.is_loaded }
        else { gl::Uniform4iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4iv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4iv", &["glUniform4ivARB"]))
    } }
}
pub mod Uniform4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4ui.is_loaded }
        else { gl::Uniform4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4ui = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4ui", &["glUniform4uiEXT"]))
    } }
}
pub mod Uniform4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Uniform4uiv.is_loaded }
        else { gl::Uniform4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Uniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniform4uiv", &["glUniform4uivEXT"]))
    } }
}
pub mod UniformBlockBinding {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformBlockBinding.is_loaded }
        else { gl::UniformBlockBinding::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformBlockBinding = FnPtr::new(metaloadfn(&mut loadfn, "glUniformBlockBinding", &[]))
    } }
}
pub mod UniformMatrix2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2dv.is_loaded }
        else { gl::UniformMatrix2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2dv", &[]))
    } }
}
pub mod UniformMatrix2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2fv.is_loaded }
        else { gl::UniformMatrix2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2fv", &["glUniformMatrix2fvARB"]))
    } }
}
pub mod UniformMatrix2x3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2x3dv.is_loaded }
        else { gl::UniformMatrix2x3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2x3dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x3dv", &[]))
    } }
}
pub mod UniformMatrix2x3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2x3fv.is_loaded }
        else { gl::UniformMatrix2x3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x3fv", &["glUniformMatrix2x3fvNV"]))
    } }
}
pub mod UniformMatrix2x4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2x4dv.is_loaded }
        else { gl::UniformMatrix2x4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2x4dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x4dv", &[]))
    } }
}
pub mod UniformMatrix2x4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix2x4fv.is_loaded }
        else { gl::UniformMatrix2x4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix2x4fv", &["glUniformMatrix2x4fvNV"]))
    } }
}
pub mod UniformMatrix3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3dv.is_loaded }
        else { gl::UniformMatrix3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3dv", &[]))
    } }
}
pub mod UniformMatrix3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3fv.is_loaded }
        else { gl::UniformMatrix3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3fv", &["glUniformMatrix3fvARB"]))
    } }
}
pub mod UniformMatrix3x2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3x2dv.is_loaded }
        else { gl::UniformMatrix3x2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3x2dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x2dv", &[]))
    } }
}
pub mod UniformMatrix3x2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3x2fv.is_loaded }
        else { gl::UniformMatrix3x2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x2fv", &["glUniformMatrix3x2fvNV"]))
    } }
}
pub mod UniformMatrix3x4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3x4dv.is_loaded }
        else { gl::UniformMatrix3x4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3x4dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x4dv", &[]))
    } }
}
pub mod UniformMatrix3x4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix3x4fv.is_loaded }
        else { gl::UniformMatrix3x4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix3x4fv", &["glUniformMatrix3x4fvNV"]))
    } }
}
pub mod UniformMatrix4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4dv.is_loaded }
        else { gl::UniformMatrix4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4dv", &[]))
    } }
}
pub mod UniformMatrix4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4fv.is_loaded }
        else { gl::UniformMatrix4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4fv", &["glUniformMatrix4fvARB"]))
    } }
}
pub mod UniformMatrix4x2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4x2dv.is_loaded }
        else { gl::UniformMatrix4x2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4x2dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x2dv", &[]))
    } }
}
pub mod UniformMatrix4x2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4x2fv.is_loaded }
        else { gl::UniformMatrix4x2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x2fv", &["glUniformMatrix4x2fvNV"]))
    } }
}
pub mod UniformMatrix4x3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4x3dv.is_loaded }
        else { gl::UniformMatrix4x3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4x3dv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x3dv", &[]))
    } }
}
pub mod UniformMatrix4x3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformMatrix4x3fv.is_loaded }
        else { gl::UniformMatrix4x3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformMatrix4x3fv", &["glUniformMatrix4x3fvNV"]))
    } }
}
pub mod UniformSubroutinesuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UniformSubroutinesuiv.is_loaded }
        else { gl::UniformSubroutinesuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UniformSubroutinesuiv = FnPtr::new(metaloadfn(&mut loadfn, "glUniformSubroutinesuiv", &[]))
    } }
}
pub mod UnmapBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UnmapBuffer.is_loaded }
        else { gl::UnmapBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UnmapBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glUnmapBuffer", &["glUnmapBufferARB", "glUnmapBufferOES"]))
    } }
}
pub mod UnmapNamedBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UnmapNamedBuffer.is_loaded }
        else { gl::UnmapNamedBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UnmapNamedBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glUnmapNamedBuffer", &[]))
    } }
}
pub mod UseProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UseProgram.is_loaded }
        else { gl::UseProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UseProgram = FnPtr::new(metaloadfn(&mut loadfn, "glUseProgram", &["glUseProgramObjectARB"]))
    } }
}
pub mod UseProgramStages {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).UseProgramStages.is_loaded }
        else { gl::UseProgramStages::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).UseProgramStages = FnPtr::new(metaloadfn(&mut loadfn, "glUseProgramStages", &[]))
    } }
}
pub mod ValidateProgram {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ValidateProgram.is_loaded }
        else { gl::ValidateProgram::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ValidateProgram = FnPtr::new(metaloadfn(&mut loadfn, "glValidateProgram", &["glValidateProgramARB"]))
    } }
}
pub mod ValidateProgramPipeline {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ValidateProgramPipeline.is_loaded }
        else { gl::ValidateProgramPipeline::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ValidateProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, "glValidateProgramPipeline", &[]))
    } }
}
pub mod VertexArrayAttribBinding {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayAttribBinding.is_loaded }
        else { gl::VertexArrayAttribBinding::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayAttribBinding = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayAttribBinding", &[]))
    } }
}
pub mod VertexArrayAttribFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayAttribFormat.is_loaded }
        else { gl::VertexArrayAttribFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayAttribFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayAttribFormat", &[]))
    } }
}
pub mod VertexArrayAttribIFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayAttribIFormat.is_loaded }
        else { gl::VertexArrayAttribIFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayAttribIFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayAttribIFormat", &[]))
    } }
}
pub mod VertexArrayAttribLFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayAttribLFormat.is_loaded }
        else { gl::VertexArrayAttribLFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayAttribLFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayAttribLFormat", &[]))
    } }
}
pub mod VertexArrayBindingDivisor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayBindingDivisor.is_loaded }
        else { gl::VertexArrayBindingDivisor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayBindingDivisor = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayBindingDivisor", &[]))
    } }
}
pub mod VertexArrayElementBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayElementBuffer.is_loaded }
        else { gl::VertexArrayElementBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayElementBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayElementBuffer", &[]))
    } }
}
pub mod VertexArrayVertexBuffer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayVertexBuffer.is_loaded }
        else { gl::VertexArrayVertexBuffer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayVertexBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayVertexBuffer", &[]))
    } }
}
pub mod VertexArrayVertexBuffers {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexArrayVertexBuffers.is_loaded }
        else { gl::VertexArrayVertexBuffers::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexArrayVertexBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glVertexArrayVertexBuffers", &[]))
    } }
}
pub mod VertexAttrib1d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1d.is_loaded }
        else { gl::VertexAttrib1d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1d", &["glVertexAttrib1dARB", "glVertexAttrib1dNV"]))
    } }
}
pub mod VertexAttrib1dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1dv.is_loaded }
        else { gl::VertexAttrib1dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1dv", &["glVertexAttrib1dvARB", "glVertexAttrib1dvNV"]))
    } }
}
pub mod VertexAttrib1f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1f.is_loaded }
        else { gl::VertexAttrib1f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1f", &["glVertexAttrib1fARB", "glVertexAttrib1fNV"]))
    } }
}
pub mod VertexAttrib1fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1fv.is_loaded }
        else { gl::VertexAttrib1fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1fv", &["glVertexAttrib1fvARB", "glVertexAttrib1fvNV"]))
    } }
}
pub mod VertexAttrib1s {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1s.is_loaded }
        else { gl::VertexAttrib1s::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1s = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1s", &["glVertexAttrib1sARB", "glVertexAttrib1sNV"]))
    } }
}
pub mod VertexAttrib1sv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib1sv.is_loaded }
        else { gl::VertexAttrib1sv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib1sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib1sv", &["glVertexAttrib1svARB", "glVertexAttrib1svNV"]))
    } }
}
pub mod VertexAttrib2d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2d.is_loaded }
        else { gl::VertexAttrib2d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2d", &["glVertexAttrib2dARB", "glVertexAttrib2dNV"]))
    } }
}
pub mod VertexAttrib2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2dv.is_loaded }
        else { gl::VertexAttrib2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2dv", &["glVertexAttrib2dvARB", "glVertexAttrib2dvNV"]))
    } }
}
pub mod VertexAttrib2f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2f.is_loaded }
        else { gl::VertexAttrib2f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2f", &["glVertexAttrib2fARB", "glVertexAttrib2fNV"]))
    } }
}
pub mod VertexAttrib2fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2fv.is_loaded }
        else { gl::VertexAttrib2fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2fv", &["glVertexAttrib2fvARB", "glVertexAttrib2fvNV"]))
    } }
}
pub mod VertexAttrib2s {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2s.is_loaded }
        else { gl::VertexAttrib2s::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2s = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2s", &["glVertexAttrib2sARB", "glVertexAttrib2sNV"]))
    } }
}
pub mod VertexAttrib2sv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib2sv.is_loaded }
        else { gl::VertexAttrib2sv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib2sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib2sv", &["glVertexAttrib2svARB", "glVertexAttrib2svNV"]))
    } }
}
pub mod VertexAttrib3d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3d.is_loaded }
        else { gl::VertexAttrib3d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3d", &["glVertexAttrib3dARB", "glVertexAttrib3dNV"]))
    } }
}
pub mod VertexAttrib3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3dv.is_loaded }
        else { gl::VertexAttrib3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3dv", &["glVertexAttrib3dvARB", "glVertexAttrib3dvNV"]))
    } }
}
pub mod VertexAttrib3f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3f.is_loaded }
        else { gl::VertexAttrib3f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3f", &["glVertexAttrib3fARB", "glVertexAttrib3fNV"]))
    } }
}
pub mod VertexAttrib3fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3fv.is_loaded }
        else { gl::VertexAttrib3fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3fv", &["glVertexAttrib3fvARB", "glVertexAttrib3fvNV"]))
    } }
}
pub mod VertexAttrib3s {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3s.is_loaded }
        else { gl::VertexAttrib3s::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3s = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3s", &["glVertexAttrib3sARB", "glVertexAttrib3sNV"]))
    } }
}
pub mod VertexAttrib3sv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib3sv.is_loaded }
        else { gl::VertexAttrib3sv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib3sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib3sv", &["glVertexAttrib3svARB", "glVertexAttrib3svNV"]))
    } }
}
pub mod VertexAttrib4Nbv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nbv.is_loaded }
        else { gl::VertexAttrib4Nbv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nbv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nbv", &["glVertexAttrib4NbvARB"]))
    } }
}
pub mod VertexAttrib4Niv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Niv.is_loaded }
        else { gl::VertexAttrib4Niv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Niv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Niv", &["glVertexAttrib4NivARB"]))
    } }
}
pub mod VertexAttrib4Nsv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nsv.is_loaded }
        else { gl::VertexAttrib4Nsv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nsv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nsv", &["glVertexAttrib4NsvARB"]))
    } }
}
pub mod VertexAttrib4Nub {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nub.is_loaded }
        else { gl::VertexAttrib4Nub::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nub = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nub", &["glVertexAttrib4NubARB", "glVertexAttrib4ubNV"]))
    } }
}
pub mod VertexAttrib4Nubv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nubv.is_loaded }
        else { gl::VertexAttrib4Nubv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nubv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nubv", &["glVertexAttrib4NubvARB", "glVertexAttrib4ubvNV"]))
    } }
}
pub mod VertexAttrib4Nuiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nuiv.is_loaded }
        else { gl::VertexAttrib4Nuiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nuiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nuiv", &["glVertexAttrib4NuivARB"]))
    } }
}
pub mod VertexAttrib4Nusv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4Nusv.is_loaded }
        else { gl::VertexAttrib4Nusv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4Nusv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4Nusv", &["glVertexAttrib4NusvARB"]))
    } }
}
pub mod VertexAttrib4bv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4bv.is_loaded }
        else { gl::VertexAttrib4bv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4bv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4bv", &["glVertexAttrib4bvARB"]))
    } }
}
pub mod VertexAttrib4d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4d.is_loaded }
        else { gl::VertexAttrib4d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4d", &["glVertexAttrib4dARB", "glVertexAttrib4dNV"]))
    } }
}
pub mod VertexAttrib4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4dv.is_loaded }
        else { gl::VertexAttrib4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4dv", &["glVertexAttrib4dvARB", "glVertexAttrib4dvNV"]))
    } }
}
pub mod VertexAttrib4f {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4f.is_loaded }
        else { gl::VertexAttrib4f::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4f = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4f", &["glVertexAttrib4fARB", "glVertexAttrib4fNV"]))
    } }
}
pub mod VertexAttrib4fv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4fv.is_loaded }
        else { gl::VertexAttrib4fv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4fv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4fv", &["glVertexAttrib4fvARB", "glVertexAttrib4fvNV"]))
    } }
}
pub mod VertexAttrib4iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4iv.is_loaded }
        else { gl::VertexAttrib4iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4iv", &["glVertexAttrib4ivARB"]))
    } }
}
pub mod VertexAttrib4s {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4s.is_loaded }
        else { gl::VertexAttrib4s::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4s = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4s", &["glVertexAttrib4sARB", "glVertexAttrib4sNV"]))
    } }
}
pub mod VertexAttrib4sv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4sv.is_loaded }
        else { gl::VertexAttrib4sv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4sv", &["glVertexAttrib4svARB", "glVertexAttrib4svNV"]))
    } }
}
pub mod VertexAttrib4ubv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4ubv.is_loaded }
        else { gl::VertexAttrib4ubv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4ubv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4ubv", &["glVertexAttrib4ubvARB"]))
    } }
}
pub mod VertexAttrib4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4uiv.is_loaded }
        else { gl::VertexAttrib4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4uiv", &["glVertexAttrib4uivARB"]))
    } }
}
pub mod VertexAttrib4usv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttrib4usv.is_loaded }
        else { gl::VertexAttrib4usv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttrib4usv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttrib4usv", &["glVertexAttrib4usvARB"]))
    } }
}
pub mod VertexAttribBinding {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribBinding.is_loaded }
        else { gl::VertexAttribBinding::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribBinding = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribBinding", &[]))
    } }
}
pub mod VertexAttribDivisor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribDivisor.is_loaded }
        else { gl::VertexAttribDivisor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribDivisor = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribDivisor", &["glVertexAttribDivisorANGLE", "glVertexAttribDivisorARB", "glVertexAttribDivisorEXT", "glVertexAttribDivisorNV"]))
    } }
}
pub mod VertexAttribFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribFormat.is_loaded }
        else { gl::VertexAttribFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribFormat", &[]))
    } }
}
pub mod VertexAttribI1i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI1i.is_loaded }
        else { gl::VertexAttribI1i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI1i = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI1i", &["glVertexAttribI1iEXT"]))
    } }
}
pub mod VertexAttribI1iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI1iv.is_loaded }
        else { gl::VertexAttribI1iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI1iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI1iv", &["glVertexAttribI1ivEXT"]))
    } }
}
pub mod VertexAttribI1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI1ui.is_loaded }
        else { gl::VertexAttribI1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI1ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI1ui", &["glVertexAttribI1uiEXT"]))
    } }
}
pub mod VertexAttribI1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI1uiv.is_loaded }
        else { gl::VertexAttribI1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI1uiv", &["glVertexAttribI1uivEXT"]))
    } }
}
pub mod VertexAttribI2i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI2i.is_loaded }
        else { gl::VertexAttribI2i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI2i = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI2i", &["glVertexAttribI2iEXT"]))
    } }
}
pub mod VertexAttribI2iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI2iv.is_loaded }
        else { gl::VertexAttribI2iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI2iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI2iv", &["glVertexAttribI2ivEXT"]))
    } }
}
pub mod VertexAttribI2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI2ui.is_loaded }
        else { gl::VertexAttribI2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI2ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI2ui", &["glVertexAttribI2uiEXT"]))
    } }
}
pub mod VertexAttribI2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI2uiv.is_loaded }
        else { gl::VertexAttribI2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI2uiv", &["glVertexAttribI2uivEXT"]))
    } }
}
pub mod VertexAttribI3i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI3i.is_loaded }
        else { gl::VertexAttribI3i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI3i = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI3i", &["glVertexAttribI3iEXT"]))
    } }
}
pub mod VertexAttribI3iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI3iv.is_loaded }
        else { gl::VertexAttribI3iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI3iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI3iv", &["glVertexAttribI3ivEXT"]))
    } }
}
pub mod VertexAttribI3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI3ui.is_loaded }
        else { gl::VertexAttribI3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI3ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI3ui", &["glVertexAttribI3uiEXT"]))
    } }
}
pub mod VertexAttribI3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI3uiv.is_loaded }
        else { gl::VertexAttribI3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI3uiv", &["glVertexAttribI3uivEXT"]))
    } }
}
pub mod VertexAttribI4bv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4bv.is_loaded }
        else { gl::VertexAttribI4bv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4bv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4bv", &["glVertexAttribI4bvEXT"]))
    } }
}
pub mod VertexAttribI4i {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4i.is_loaded }
        else { gl::VertexAttribI4i::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4i = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4i", &["glVertexAttribI4iEXT"]))
    } }
}
pub mod VertexAttribI4iv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4iv.is_loaded }
        else { gl::VertexAttribI4iv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4iv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4iv", &["glVertexAttribI4ivEXT"]))
    } }
}
pub mod VertexAttribI4sv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4sv.is_loaded }
        else { gl::VertexAttribI4sv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4sv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4sv", &["glVertexAttribI4svEXT"]))
    } }
}
pub mod VertexAttribI4ubv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4ubv.is_loaded }
        else { gl::VertexAttribI4ubv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4ubv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4ubv", &["glVertexAttribI4ubvEXT"]))
    } }
}
pub mod VertexAttribI4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4ui.is_loaded }
        else { gl::VertexAttribI4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4ui", &["glVertexAttribI4uiEXT"]))
    } }
}
pub mod VertexAttribI4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4uiv.is_loaded }
        else { gl::VertexAttribI4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4uiv", &["glVertexAttribI4uivEXT"]))
    } }
}
pub mod VertexAttribI4usv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribI4usv.is_loaded }
        else { gl::VertexAttribI4usv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribI4usv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribI4usv", &["glVertexAttribI4usvEXT"]))
    } }
}
pub mod VertexAttribIFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribIFormat.is_loaded }
        else { gl::VertexAttribIFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribIFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribIFormat", &[]))
    } }
}
pub mod VertexAttribIPointer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribIPointer.is_loaded }
        else { gl::VertexAttribIPointer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribIPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribIPointer", &["glVertexAttribIPointerEXT"]))
    } }
}
pub mod VertexAttribL1d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL1d.is_loaded }
        else { gl::VertexAttribL1d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL1d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL1d", &["glVertexAttribL1dEXT"]))
    } }
}
pub mod VertexAttribL1dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL1dv.is_loaded }
        else { gl::VertexAttribL1dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL1dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL1dv", &["glVertexAttribL1dvEXT"]))
    } }
}
pub mod VertexAttribL2d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL2d.is_loaded }
        else { gl::VertexAttribL2d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL2d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL2d", &["glVertexAttribL2dEXT"]))
    } }
}
pub mod VertexAttribL2dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL2dv.is_loaded }
        else { gl::VertexAttribL2dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL2dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL2dv", &["glVertexAttribL2dvEXT"]))
    } }
}
pub mod VertexAttribL3d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL3d.is_loaded }
        else { gl::VertexAttribL3d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL3d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL3d", &["glVertexAttribL3dEXT"]))
    } }
}
pub mod VertexAttribL3dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL3dv.is_loaded }
        else { gl::VertexAttribL3dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL3dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL3dv", &["glVertexAttribL3dvEXT"]))
    } }
}
pub mod VertexAttribL4d {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL4d.is_loaded }
        else { gl::VertexAttribL4d::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL4d = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL4d", &["glVertexAttribL4dEXT"]))
    } }
}
pub mod VertexAttribL4dv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribL4dv.is_loaded }
        else { gl::VertexAttribL4dv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribL4dv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribL4dv", &["glVertexAttribL4dvEXT"]))
    } }
}
pub mod VertexAttribLFormat {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribLFormat.is_loaded }
        else { gl::VertexAttribLFormat::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribLFormat = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribLFormat", &[]))
    } }
}
pub mod VertexAttribLPointer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribLPointer.is_loaded }
        else { gl::VertexAttribLPointer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribLPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribLPointer", &["glVertexAttribLPointerEXT"]))
    } }
}
pub mod VertexAttribP1ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP1ui.is_loaded }
        else { gl::VertexAttribP1ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP1ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP1ui", &[]))
    } }
}
pub mod VertexAttribP1uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP1uiv.is_loaded }
        else { gl::VertexAttribP1uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP1uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP1uiv", &[]))
    } }
}
pub mod VertexAttribP2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP2ui.is_loaded }
        else { gl::VertexAttribP2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP2ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP2ui", &[]))
    } }
}
pub mod VertexAttribP2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP2uiv.is_loaded }
        else { gl::VertexAttribP2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP2uiv", &[]))
    } }
}
pub mod VertexAttribP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP3ui.is_loaded }
        else { gl::VertexAttribP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP3ui", &[]))
    } }
}
pub mod VertexAttribP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP3uiv.is_loaded }
        else { gl::VertexAttribP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP3uiv", &[]))
    } }
}
pub mod VertexAttribP4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP4ui.is_loaded }
        else { gl::VertexAttribP4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP4ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP4ui", &[]))
    } }
}
pub mod VertexAttribP4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribP4uiv.is_loaded }
        else { gl::VertexAttribP4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribP4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribP4uiv", &[]))
    } }
}
pub mod VertexAttribPointer {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexAttribPointer.is_loaded }
        else { gl::VertexAttribPointer::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexAttribPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexAttribPointer", &["glVertexAttribPointerARB"]))
    } }
}
pub mod VertexBindingDivisor {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexBindingDivisor.is_loaded }
        else { gl::VertexBindingDivisor::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexBindingDivisor = FnPtr::new(metaloadfn(&mut loadfn, "glVertexBindingDivisor", &[]))
    } }
}
pub mod VertexP2ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP2ui.is_loaded }
        else { gl::VertexP2ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP2ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP2ui", &[]))
    } }
}
pub mod VertexP2uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP2uiv.is_loaded }
        else { gl::VertexP2uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP2uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP2uiv", &[]))
    } }
}
pub mod VertexP3ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP3ui.is_loaded }
        else { gl::VertexP3ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP3ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP3ui", &[]))
    } }
}
pub mod VertexP3uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP3uiv.is_loaded }
        else { gl::VertexP3uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP3uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP3uiv", &[]))
    } }
}
pub mod VertexP4ui {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP4ui.is_loaded }
        else { gl::VertexP4ui::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP4ui = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP4ui", &[]))
    } }
}
pub mod VertexP4uiv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).VertexP4uiv.is_loaded }
        else { gl::VertexP4uiv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).VertexP4uiv = FnPtr::new(metaloadfn(&mut loadfn, "glVertexP4uiv", &[]))
    } }
}
pub mod Viewport {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).Viewport.is_loaded }
        else { gl::Viewport::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).Viewport = FnPtr::new(metaloadfn(&mut loadfn, "glViewport", &[]))
    } }
}
pub mod ViewportArrayv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ViewportArrayv.is_loaded }
        else { gl::ViewportArrayv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ViewportArrayv = FnPtr::new(metaloadfn(&mut loadfn, "glViewportArrayv", &["glViewportArrayvNV", "glViewportArrayvOES"]))
    } }
}
pub mod ViewportIndexedf {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ViewportIndexedf.is_loaded }
        else { gl::ViewportIndexedf::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ViewportIndexedf = FnPtr::new(metaloadfn(&mut loadfn, "glViewportIndexedf", &["glViewportIndexedfOES", "glViewportIndexedfNV"]))
    } }
}
pub mod ViewportIndexedfv {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).ViewportIndexedfv.is_loaded }
        else { gl::ViewportIndexedfv::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).ViewportIndexedfv = FnPtr::new(metaloadfn(&mut loadfn, "glViewportIndexedfv", &["glViewportIndexedfvOES", "glViewportIndexedfvNV"]))
    } }
}
pub mod WaitSync {
    use crate::gl_util::gl_wrapper::metaloadfn;
    use crate::gl_util::gl_wrapper::FnPtr;
    use crate::gl_util::gl_wrapper::STORAGE;
    use crate::gl_util::gl_wrapper::EXTERNAL_POINTERS;
    use std::ffi::c_void;
    pub fn is_loaded() -> bool { unsafe { 
        if EXTERNAL_POINTERS { (*STORAGE).WaitSync.is_loaded }
        else { gl::WaitSync::is_loaded() }
    } }

    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void { unsafe {
        (*STORAGE).WaitSync = FnPtr::new(metaloadfn(&mut loadfn, "glWaitSync", &["glWaitSyncAPPLE"]))
    } }
}

pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

impl FnPtr {
    /// Creates a `FnPtr` from a load attempt.
    pub fn new(info: (*const c_void, &'static str)) -> FnPtr { unsafe {
        let (ptr, symbol) = info;
        if ptr.is_null() {
            FnPtr { f: missing_fn_panic as *const c_void, is_loaded: false }
        } else {
            FnPtr { f: ptr, is_loaded: true }
        }
    } }
}

pub struct PointerStorage {
    pub ActiveShaderProgram: FnPtr,
    pub ActiveTexture: FnPtr,
    pub AttachShader: FnPtr,
    pub BeginConditionalRender: FnPtr,
    pub BeginQuery: FnPtr,
    pub BeginQueryIndexed: FnPtr,
    pub BeginTransformFeedback: FnPtr,
    pub BindAttribLocation: FnPtr,
    pub BindBuffer: FnPtr,
    pub BindBufferBase: FnPtr,
    pub BindBufferRange: FnPtr,
    pub BindBuffersBase: FnPtr,
    pub BindBuffersRange: FnPtr,
    pub BindFragDataLocation: FnPtr,
    pub BindFragDataLocationIndexed: FnPtr,
    pub BindFramebuffer: FnPtr,
    pub BindImageTexture: FnPtr,
    pub BindImageTextures: FnPtr,
    pub BindProgramPipeline: FnPtr,
    pub BindRenderbuffer: FnPtr,
    pub BindSampler: FnPtr,
    pub BindSamplers: FnPtr,
    pub BindTexture: FnPtr,
    pub BindTextureUnit: FnPtr,
    pub BindTextures: FnPtr,
    pub BindTransformFeedback: FnPtr,
    pub BindVertexArray: FnPtr,
    pub BindVertexBuffer: FnPtr,
    pub BindVertexBuffers: FnPtr,
    pub BlendColor: FnPtr,
    pub BlendEquation: FnPtr,
    pub BlendEquationSeparate: FnPtr,
    pub BlendEquationSeparatei: FnPtr,
    pub BlendEquationi: FnPtr,
    pub BlendFunc: FnPtr,
    pub BlendFuncSeparate: FnPtr,
    pub BlendFuncSeparatei: FnPtr,
    pub BlendFunci: FnPtr,
    pub BlitFramebuffer: FnPtr,
    pub BlitNamedFramebuffer: FnPtr,
    pub BufferData: FnPtr,
    pub BufferStorage: FnPtr,
    pub BufferSubData: FnPtr,
    pub CheckFramebufferStatus: FnPtr,
    pub CheckNamedFramebufferStatus: FnPtr,
    pub ClampColor: FnPtr,
    pub Clear: FnPtr,
    pub ClearBufferData: FnPtr,
    pub ClearBufferSubData: FnPtr,
    pub ClearBufferfi: FnPtr,
    pub ClearBufferfv: FnPtr,
    pub ClearBufferiv: FnPtr,
    pub ClearBufferuiv: FnPtr,
    pub ClearColor: FnPtr,
    pub ClearDepth: FnPtr,
    pub ClearDepthf: FnPtr,
    pub ClearNamedBufferData: FnPtr,
    pub ClearNamedBufferSubData: FnPtr,
    pub ClearNamedFramebufferfi: FnPtr,
    pub ClearNamedFramebufferfv: FnPtr,
    pub ClearNamedFramebufferiv: FnPtr,
    pub ClearNamedFramebufferuiv: FnPtr,
    pub ClearStencil: FnPtr,
    pub ClearTexImage: FnPtr,
    pub ClearTexSubImage: FnPtr,
    pub ClientWaitSync: FnPtr,
    pub ClipControl: FnPtr,
    pub ColorMask: FnPtr,
    pub ColorMaski: FnPtr,
    pub ColorP3ui: FnPtr,
    pub ColorP3uiv: FnPtr,
    pub ColorP4ui: FnPtr,
    pub ColorP4uiv: FnPtr,
    pub CompileShader: FnPtr,
    pub CompressedTexImage1D: FnPtr,
    pub CompressedTexImage2D: FnPtr,
    pub CompressedTexImage3D: FnPtr,
    pub CompressedTexSubImage1D: FnPtr,
    pub CompressedTexSubImage2D: FnPtr,
    pub CompressedTexSubImage3D: FnPtr,
    pub CompressedTextureSubImage1D: FnPtr,
    pub CompressedTextureSubImage2D: FnPtr,
    pub CompressedTextureSubImage3D: FnPtr,
    pub CopyBufferSubData: FnPtr,
    pub CopyImageSubData: FnPtr,
    pub CopyNamedBufferSubData: FnPtr,
    pub CopyTexImage1D: FnPtr,
    pub CopyTexImage2D: FnPtr,
    pub CopyTexSubImage1D: FnPtr,
    pub CopyTexSubImage2D: FnPtr,
    pub CopyTexSubImage3D: FnPtr,
    pub CopyTextureSubImage1D: FnPtr,
    pub CopyTextureSubImage2D: FnPtr,
    pub CopyTextureSubImage3D: FnPtr,
    pub CreateBuffers: FnPtr,
    pub CreateFramebuffers: FnPtr,
    pub CreateProgram: FnPtr,
    pub CreateProgramPipelines: FnPtr,
    pub CreateQueries: FnPtr,
    pub CreateRenderbuffers: FnPtr,
    pub CreateSamplers: FnPtr,
    pub CreateShader: FnPtr,
    pub CreateShaderProgramv: FnPtr,
    pub CreateTextures: FnPtr,
    pub CreateTransformFeedbacks: FnPtr,
    pub CreateVertexArrays: FnPtr,
    pub CullFace: FnPtr,
    pub DebugMessageCallback: FnPtr,
    pub DebugMessageControl: FnPtr,
    pub DebugMessageInsert: FnPtr,
    pub DeleteBuffers: FnPtr,
    pub DeleteFramebuffers: FnPtr,
    pub DeleteProgram: FnPtr,
    pub DeleteProgramPipelines: FnPtr,
    pub DeleteQueries: FnPtr,
    pub DeleteRenderbuffers: FnPtr,
    pub DeleteSamplers: FnPtr,
    pub DeleteShader: FnPtr,
    pub DeleteSync: FnPtr,
    pub DeleteTextures: FnPtr,
    pub DeleteTransformFeedbacks: FnPtr,
    pub DeleteVertexArrays: FnPtr,
    pub DepthFunc: FnPtr,
    pub DepthMask: FnPtr,
    pub DepthRange: FnPtr,
    pub DepthRangeArrayv: FnPtr,
    pub DepthRangeIndexed: FnPtr,
    pub DepthRangef: FnPtr,
    pub DetachShader: FnPtr,
    pub Disable: FnPtr,
    pub DisableVertexArrayAttrib: FnPtr,
    pub DisableVertexAttribArray: FnPtr,
    pub Disablei: FnPtr,
    pub DispatchCompute: FnPtr,
    pub DispatchComputeIndirect: FnPtr,
    pub DrawArrays: FnPtr,
    pub DrawArraysIndirect: FnPtr,
    pub DrawArraysInstanced: FnPtr,
    pub DrawArraysInstancedBaseInstance: FnPtr,
    pub DrawBuffer: FnPtr,
    pub DrawBuffers: FnPtr,
    pub DrawElements: FnPtr,
    pub DrawElementsBaseVertex: FnPtr,
    pub DrawElementsIndirect: FnPtr,
    pub DrawElementsInstanced: FnPtr,
    pub DrawElementsInstancedBaseInstance: FnPtr,
    pub DrawElementsInstancedBaseVertex: FnPtr,
    pub DrawElementsInstancedBaseVertexBaseInstance: FnPtr,
    pub DrawRangeElements: FnPtr,
    pub DrawRangeElementsBaseVertex: FnPtr,
    pub DrawTransformFeedback: FnPtr,
    pub DrawTransformFeedbackInstanced: FnPtr,
    pub DrawTransformFeedbackStream: FnPtr,
    pub DrawTransformFeedbackStreamInstanced: FnPtr,
    pub Enable: FnPtr,
    pub EnableVertexArrayAttrib: FnPtr,
    pub EnableVertexAttribArray: FnPtr,
    pub Enablei: FnPtr,
    pub EndConditionalRender: FnPtr,
    pub EndQuery: FnPtr,
    pub EndQueryIndexed: FnPtr,
    pub EndTransformFeedback: FnPtr,
    pub FenceSync: FnPtr,
    pub Finish: FnPtr,
    pub Flush: FnPtr,
    pub FlushMappedBufferRange: FnPtr,
    pub FlushMappedNamedBufferRange: FnPtr,
    pub FramebufferParameteri: FnPtr,
    pub FramebufferRenderbuffer: FnPtr,
    pub FramebufferTexture: FnPtr,
    pub FramebufferTexture1D: FnPtr,
    pub FramebufferTexture2D: FnPtr,
    pub FramebufferTexture3D: FnPtr,
    pub FramebufferTextureLayer: FnPtr,
    pub FrontFace: FnPtr,
    pub GenBuffers: FnPtr,
    pub GenFramebuffers: FnPtr,
    pub GenProgramPipelines: FnPtr,
    pub GenQueries: FnPtr,
    pub GenRenderbuffers: FnPtr,
    pub GenSamplers: FnPtr,
    pub GenTextures: FnPtr,
    pub GenTransformFeedbacks: FnPtr,
    pub GenVertexArrays: FnPtr,
    pub GenerateMipmap: FnPtr,
    pub GenerateTextureMipmap: FnPtr,
    pub GetActiveAtomicCounterBufferiv: FnPtr,
    pub GetActiveAttrib: FnPtr,
    pub GetActiveSubroutineName: FnPtr,
    pub GetActiveSubroutineUniformName: FnPtr,
    pub GetActiveSubroutineUniformiv: FnPtr,
    pub GetActiveUniform: FnPtr,
    pub GetActiveUniformBlockName: FnPtr,
    pub GetActiveUniformBlockiv: FnPtr,
    pub GetActiveUniformName: FnPtr,
    pub GetActiveUniformsiv: FnPtr,
    pub GetAttachedShaders: FnPtr,
    pub GetAttribLocation: FnPtr,
    pub GetBooleani_v: FnPtr,
    pub GetBooleanv: FnPtr,
    pub GetBufferParameteri64v: FnPtr,
    pub GetBufferParameteriv: FnPtr,
    pub GetBufferPointerv: FnPtr,
    pub GetBufferSubData: FnPtr,
    pub GetCompressedTexImage: FnPtr,
    pub GetCompressedTextureImage: FnPtr,
    pub GetCompressedTextureSubImage: FnPtr,
    pub GetDebugMessageLog: FnPtr,
    pub GetDoublei_v: FnPtr,
    pub GetDoublev: FnPtr,
    pub GetError: FnPtr,
    pub GetFloati_v: FnPtr,
    pub GetFloatv: FnPtr,
    pub GetFragDataIndex: FnPtr,
    pub GetFragDataLocation: FnPtr,
    pub GetFramebufferAttachmentParameteriv: FnPtr,
    pub GetFramebufferParameteriv: FnPtr,
    pub GetGraphicsResetStatus: FnPtr,
    pub GetInteger64i_v: FnPtr,
    pub GetInteger64v: FnPtr,
    pub GetIntegeri_v: FnPtr,
    pub GetIntegerv: FnPtr,
    pub GetInternalformati64v: FnPtr,
    pub GetInternalformativ: FnPtr,
    pub GetMultisamplefv: FnPtr,
    pub GetNamedBufferParameteri64v: FnPtr,
    pub GetNamedBufferParameteriv: FnPtr,
    pub GetNamedBufferPointerv: FnPtr,
    pub GetNamedBufferSubData: FnPtr,
    pub GetNamedFramebufferAttachmentParameteriv: FnPtr,
    pub GetNamedFramebufferParameteriv: FnPtr,
    pub GetNamedRenderbufferParameteriv: FnPtr,
    pub GetObjectLabel: FnPtr,
    pub GetObjectPtrLabel: FnPtr,
    pub GetPointerv: FnPtr,
    pub GetProgramBinary: FnPtr,
    pub GetProgramInfoLog: FnPtr,
    pub GetProgramInterfaceiv: FnPtr,
    pub GetProgramPipelineInfoLog: FnPtr,
    pub GetProgramPipelineiv: FnPtr,
    pub GetProgramResourceIndex: FnPtr,
    pub GetProgramResourceLocation: FnPtr,
    pub GetProgramResourceLocationIndex: FnPtr,
    pub GetProgramResourceName: FnPtr,
    pub GetProgramResourceiv: FnPtr,
    pub GetProgramStageiv: FnPtr,
    pub GetProgramiv: FnPtr,
    pub GetQueryBufferObjecti64v: FnPtr,
    pub GetQueryBufferObjectiv: FnPtr,
    pub GetQueryBufferObjectui64v: FnPtr,
    pub GetQueryBufferObjectuiv: FnPtr,
    pub GetQueryIndexediv: FnPtr,
    pub GetQueryObjecti64v: FnPtr,
    pub GetQueryObjectiv: FnPtr,
    pub GetQueryObjectui64v: FnPtr,
    pub GetQueryObjectuiv: FnPtr,
    pub GetQueryiv: FnPtr,
    pub GetRenderbufferParameteriv: FnPtr,
    pub GetSamplerParameterIiv: FnPtr,
    pub GetSamplerParameterIuiv: FnPtr,
    pub GetSamplerParameterfv: FnPtr,
    pub GetSamplerParameteriv: FnPtr,
    pub GetShaderInfoLog: FnPtr,
    pub GetShaderPrecisionFormat: FnPtr,
    pub GetShaderSource: FnPtr,
    pub GetShaderiv: FnPtr,
    pub GetString: FnPtr,
    pub GetStringi: FnPtr,
    pub GetSubroutineIndex: FnPtr,
    pub GetSubroutineUniformLocation: FnPtr,
    pub GetSynciv: FnPtr,
    pub GetTexImage: FnPtr,
    pub GetTexLevelParameterfv: FnPtr,
    pub GetTexLevelParameteriv: FnPtr,
    pub GetTexParameterIiv: FnPtr,
    pub GetTexParameterIuiv: FnPtr,
    pub GetTexParameterfv: FnPtr,
    pub GetTexParameteriv: FnPtr,
    pub GetTextureImage: FnPtr,
    pub GetTextureLevelParameterfv: FnPtr,
    pub GetTextureLevelParameteriv: FnPtr,
    pub GetTextureParameterIiv: FnPtr,
    pub GetTextureParameterIuiv: FnPtr,
    pub GetTextureParameterfv: FnPtr,
    pub GetTextureParameteriv: FnPtr,
    pub GetTextureSubImage: FnPtr,
    pub GetTransformFeedbackVarying: FnPtr,
    pub GetTransformFeedbacki64_v: FnPtr,
    pub GetTransformFeedbacki_v: FnPtr,
    pub GetTransformFeedbackiv: FnPtr,
    pub GetUniformBlockIndex: FnPtr,
    pub GetUniformIndices: FnPtr,
    pub GetUniformLocation: FnPtr,
    pub GetUniformSubroutineuiv: FnPtr,
    pub GetUniformdv: FnPtr,
    pub GetUniformfv: FnPtr,
    pub GetUniformiv: FnPtr,
    pub GetUniformuiv: FnPtr,
    pub GetVertexArrayIndexed64iv: FnPtr,
    pub GetVertexArrayIndexediv: FnPtr,
    pub GetVertexArrayiv: FnPtr,
    pub GetVertexAttribIiv: FnPtr,
    pub GetVertexAttribIuiv: FnPtr,
    pub GetVertexAttribLdv: FnPtr,
    pub GetVertexAttribPointerv: FnPtr,
    pub GetVertexAttribdv: FnPtr,
    pub GetVertexAttribfv: FnPtr,
    pub GetVertexAttribiv: FnPtr,
    pub GetnColorTable: FnPtr,
    pub GetnCompressedTexImage: FnPtr,
    pub GetnConvolutionFilter: FnPtr,
    pub GetnHistogram: FnPtr,
    pub GetnMapdv: FnPtr,
    pub GetnMapfv: FnPtr,
    pub GetnMapiv: FnPtr,
    pub GetnMinmax: FnPtr,
    pub GetnPixelMapfv: FnPtr,
    pub GetnPixelMapuiv: FnPtr,
    pub GetnPixelMapusv: FnPtr,
    pub GetnPolygonStipple: FnPtr,
    pub GetnSeparableFilter: FnPtr,
    pub GetnTexImage: FnPtr,
    pub GetnUniformdv: FnPtr,
    pub GetnUniformfv: FnPtr,
    pub GetnUniformiv: FnPtr,
    pub GetnUniformuiv: FnPtr,
    pub Hint: FnPtr,
    pub InvalidateBufferData: FnPtr,
    pub InvalidateBufferSubData: FnPtr,
    pub InvalidateFramebuffer: FnPtr,
    pub InvalidateNamedFramebufferData: FnPtr,
    pub InvalidateNamedFramebufferSubData: FnPtr,
    pub InvalidateSubFramebuffer: FnPtr,
    pub InvalidateTexImage: FnPtr,
    pub InvalidateTexSubImage: FnPtr,
    pub IsBuffer: FnPtr,
    pub IsEnabled: FnPtr,
    pub IsEnabledi: FnPtr,
    pub IsFramebuffer: FnPtr,
    pub IsProgram: FnPtr,
    pub IsProgramPipeline: FnPtr,
    pub IsQuery: FnPtr,
    pub IsRenderbuffer: FnPtr,
    pub IsSampler: FnPtr,
    pub IsShader: FnPtr,
    pub IsSync: FnPtr,
    pub IsTexture: FnPtr,
    pub IsTransformFeedback: FnPtr,
    pub IsVertexArray: FnPtr,
    pub LineWidth: FnPtr,
    pub LinkProgram: FnPtr,
    pub LogicOp: FnPtr,
    pub MapBuffer: FnPtr,
    pub MapBufferRange: FnPtr,
    pub MapNamedBuffer: FnPtr,
    pub MapNamedBufferRange: FnPtr,
    pub MemoryBarrier: FnPtr,
    pub MemoryBarrierByRegion: FnPtr,
    pub MinSampleShading: FnPtr,
    pub MultiDrawArrays: FnPtr,
    pub MultiDrawArraysIndirect: FnPtr,
    pub MultiDrawElements: FnPtr,
    pub MultiDrawElementsBaseVertex: FnPtr,
    pub MultiDrawElementsIndirect: FnPtr,
    pub MultiTexCoordP1ui: FnPtr,
    pub MultiTexCoordP1uiv: FnPtr,
    pub MultiTexCoordP2ui: FnPtr,
    pub MultiTexCoordP2uiv: FnPtr,
    pub MultiTexCoordP3ui: FnPtr,
    pub MultiTexCoordP3uiv: FnPtr,
    pub MultiTexCoordP4ui: FnPtr,
    pub MultiTexCoordP4uiv: FnPtr,
    pub NamedBufferData: FnPtr,
    pub NamedBufferStorage: FnPtr,
    pub NamedBufferSubData: FnPtr,
    pub NamedFramebufferDrawBuffer: FnPtr,
    pub NamedFramebufferDrawBuffers: FnPtr,
    pub NamedFramebufferParameteri: FnPtr,
    pub NamedFramebufferReadBuffer: FnPtr,
    pub NamedFramebufferRenderbuffer: FnPtr,
    pub NamedFramebufferTexture: FnPtr,
    pub NamedFramebufferTextureLayer: FnPtr,
    pub NamedRenderbufferStorage: FnPtr,
    pub NamedRenderbufferStorageMultisample: FnPtr,
    pub NormalP3ui: FnPtr,
    pub NormalP3uiv: FnPtr,
    pub ObjectLabel: FnPtr,
    pub ObjectPtrLabel: FnPtr,
    pub PatchParameterfv: FnPtr,
    pub PatchParameteri: FnPtr,
    pub PauseTransformFeedback: FnPtr,
    pub PixelStoref: FnPtr,
    pub PixelStorei: FnPtr,
    pub PointParameterf: FnPtr,
    pub PointParameterfv: FnPtr,
    pub PointParameteri: FnPtr,
    pub PointParameteriv: FnPtr,
    pub PointSize: FnPtr,
    pub PolygonMode: FnPtr,
    pub PolygonOffset: FnPtr,
    pub PopDebugGroup: FnPtr,
    pub PrimitiveRestartIndex: FnPtr,
    pub ProgramBinary: FnPtr,
    pub ProgramParameteri: FnPtr,
    pub ProgramUniform1d: FnPtr,
    pub ProgramUniform1dv: FnPtr,
    pub ProgramUniform1f: FnPtr,
    pub ProgramUniform1fv: FnPtr,
    pub ProgramUniform1i: FnPtr,
    pub ProgramUniform1iv: FnPtr,
    pub ProgramUniform1ui: FnPtr,
    pub ProgramUniform1uiv: FnPtr,
    pub ProgramUniform2d: FnPtr,
    pub ProgramUniform2dv: FnPtr,
    pub ProgramUniform2f: FnPtr,
    pub ProgramUniform2fv: FnPtr,
    pub ProgramUniform2i: FnPtr,
    pub ProgramUniform2iv: FnPtr,
    pub ProgramUniform2ui: FnPtr,
    pub ProgramUniform2uiv: FnPtr,
    pub ProgramUniform3d: FnPtr,
    pub ProgramUniform3dv: FnPtr,
    pub ProgramUniform3f: FnPtr,
    pub ProgramUniform3fv: FnPtr,
    pub ProgramUniform3i: FnPtr,
    pub ProgramUniform3iv: FnPtr,
    pub ProgramUniform3ui: FnPtr,
    pub ProgramUniform3uiv: FnPtr,
    pub ProgramUniform4d: FnPtr,
    pub ProgramUniform4dv: FnPtr,
    pub ProgramUniform4f: FnPtr,
    pub ProgramUniform4fv: FnPtr,
    pub ProgramUniform4i: FnPtr,
    pub ProgramUniform4iv: FnPtr,
    pub ProgramUniform4ui: FnPtr,
    pub ProgramUniform4uiv: FnPtr,
    pub ProgramUniformMatrix2dv: FnPtr,
    pub ProgramUniformMatrix2fv: FnPtr,
    pub ProgramUniformMatrix2x3dv: FnPtr,
    pub ProgramUniformMatrix2x3fv: FnPtr,
    pub ProgramUniformMatrix2x4dv: FnPtr,
    pub ProgramUniformMatrix2x4fv: FnPtr,
    pub ProgramUniformMatrix3dv: FnPtr,
    pub ProgramUniformMatrix3fv: FnPtr,
    pub ProgramUniformMatrix3x2dv: FnPtr,
    pub ProgramUniformMatrix3x2fv: FnPtr,
    pub ProgramUniformMatrix3x4dv: FnPtr,
    pub ProgramUniformMatrix3x4fv: FnPtr,
    pub ProgramUniformMatrix4dv: FnPtr,
    pub ProgramUniformMatrix4fv: FnPtr,
    pub ProgramUniformMatrix4x2dv: FnPtr,
    pub ProgramUniformMatrix4x2fv: FnPtr,
    pub ProgramUniformMatrix4x3dv: FnPtr,
    pub ProgramUniformMatrix4x3fv: FnPtr,
    pub ProvokingVertex: FnPtr,
    pub PushDebugGroup: FnPtr,
    pub QueryCounter: FnPtr,
    pub ReadBuffer: FnPtr,
    pub ReadPixels: FnPtr,
    pub ReadnPixels: FnPtr,
    pub ReleaseShaderCompiler: FnPtr,
    pub RenderbufferStorage: FnPtr,
    pub RenderbufferStorageMultisample: FnPtr,
    pub ResumeTransformFeedback: FnPtr,
    pub SampleCoverage: FnPtr,
    pub SampleMaski: FnPtr,
    pub SamplerParameterIiv: FnPtr,
    pub SamplerParameterIuiv: FnPtr,
    pub SamplerParameterf: FnPtr,
    pub SamplerParameterfv: FnPtr,
    pub SamplerParameteri: FnPtr,
    pub SamplerParameteriv: FnPtr,
    pub Scissor: FnPtr,
    pub ScissorArrayv: FnPtr,
    pub ScissorIndexed: FnPtr,
    pub ScissorIndexedv: FnPtr,
    pub SecondaryColorP3ui: FnPtr,
    pub SecondaryColorP3uiv: FnPtr,
    pub ShaderBinary: FnPtr,
    pub ShaderSource: FnPtr,
    pub ShaderStorageBlockBinding: FnPtr,
    pub StencilFunc: FnPtr,
    pub StencilFuncSeparate: FnPtr,
    pub StencilMask: FnPtr,
    pub StencilMaskSeparate: FnPtr,
    pub StencilOp: FnPtr,
    pub StencilOpSeparate: FnPtr,
    pub TexBuffer: FnPtr,
    pub TexBufferRange: FnPtr,
    pub TexCoordP1ui: FnPtr,
    pub TexCoordP1uiv: FnPtr,
    pub TexCoordP2ui: FnPtr,
    pub TexCoordP2uiv: FnPtr,
    pub TexCoordP3ui: FnPtr,
    pub TexCoordP3uiv: FnPtr,
    pub TexCoordP4ui: FnPtr,
    pub TexCoordP4uiv: FnPtr,
    pub TexImage1D: FnPtr,
    pub TexImage2D: FnPtr,
    pub TexImage2DMultisample: FnPtr,
    pub TexImage3D: FnPtr,
    pub TexImage3DMultisample: FnPtr,
    pub TexParameterIiv: FnPtr,
    pub TexParameterIuiv: FnPtr,
    pub TexParameterf: FnPtr,
    pub TexParameterfv: FnPtr,
    pub TexParameteri: FnPtr,
    pub TexParameteriv: FnPtr,
    pub TexStorage1D: FnPtr,
    pub TexStorage2D: FnPtr,
    pub TexStorage2DMultisample: FnPtr,
    pub TexStorage3D: FnPtr,
    pub TexStorage3DMultisample: FnPtr,
    pub TexSubImage1D: FnPtr,
    pub TexSubImage2D: FnPtr,
    pub TexSubImage3D: FnPtr,
    pub TextureBarrier: FnPtr,
    pub TextureBuffer: FnPtr,
    pub TextureBufferRange: FnPtr,
    pub TextureParameterIiv: FnPtr,
    pub TextureParameterIuiv: FnPtr,
    pub TextureParameterf: FnPtr,
    pub TextureParameterfv: FnPtr,
    pub TextureParameteri: FnPtr,
    pub TextureParameteriv: FnPtr,
    pub TextureStorage1D: FnPtr,
    pub TextureStorage2D: FnPtr,
    pub TextureStorage2DMultisample: FnPtr,
    pub TextureStorage3D: FnPtr,
    pub TextureStorage3DMultisample: FnPtr,
    pub TextureSubImage1D: FnPtr,
    pub TextureSubImage2D: FnPtr,
    pub TextureSubImage3D: FnPtr,
    pub TextureView: FnPtr,
    pub TransformFeedbackBufferBase: FnPtr,
    pub TransformFeedbackBufferRange: FnPtr,
    pub TransformFeedbackVaryings: FnPtr,
    pub Uniform1d: FnPtr,
    pub Uniform1dv: FnPtr,
    pub Uniform1f: FnPtr,
    pub Uniform1fv: FnPtr,
    pub Uniform1i: FnPtr,
    pub Uniform1iv: FnPtr,
    pub Uniform1ui: FnPtr,
    pub Uniform1uiv: FnPtr,
    pub Uniform2d: FnPtr,
    pub Uniform2dv: FnPtr,
    pub Uniform2f: FnPtr,
    pub Uniform2fv: FnPtr,
    pub Uniform2i: FnPtr,
    pub Uniform2iv: FnPtr,
    pub Uniform2ui: FnPtr,
    pub Uniform2uiv: FnPtr,
    pub Uniform3d: FnPtr,
    pub Uniform3dv: FnPtr,
    pub Uniform3f: FnPtr,
    pub Uniform3fv: FnPtr,
    pub Uniform3i: FnPtr,
    pub Uniform3iv: FnPtr,
    pub Uniform3ui: FnPtr,
    pub Uniform3uiv: FnPtr,
    pub Uniform4d: FnPtr,
    pub Uniform4dv: FnPtr,
    pub Uniform4f: FnPtr,
    pub Uniform4fv: FnPtr,
    pub Uniform4i: FnPtr,
    pub Uniform4iv: FnPtr,
    pub Uniform4ui: FnPtr,
    pub Uniform4uiv: FnPtr,
    pub UniformBlockBinding: FnPtr,
    pub UniformMatrix2dv: FnPtr,
    pub UniformMatrix2fv: FnPtr,
    pub UniformMatrix2x3dv: FnPtr,
    pub UniformMatrix2x3fv: FnPtr,
    pub UniformMatrix2x4dv: FnPtr,
    pub UniformMatrix2x4fv: FnPtr,
    pub UniformMatrix3dv: FnPtr,
    pub UniformMatrix3fv: FnPtr,
    pub UniformMatrix3x2dv: FnPtr,
    pub UniformMatrix3x2fv: FnPtr,
    pub UniformMatrix3x4dv: FnPtr,
    pub UniformMatrix3x4fv: FnPtr,
    pub UniformMatrix4dv: FnPtr,
    pub UniformMatrix4fv: FnPtr,
    pub UniformMatrix4x2dv: FnPtr,
    pub UniformMatrix4x2fv: FnPtr,
    pub UniformMatrix4x3dv: FnPtr,
    pub UniformMatrix4x3fv: FnPtr,
    pub UniformSubroutinesuiv: FnPtr,
    pub UnmapBuffer: FnPtr,
    pub UnmapNamedBuffer: FnPtr,
    pub UseProgram: FnPtr,
    pub UseProgramStages: FnPtr,
    pub ValidateProgram: FnPtr,
    pub ValidateProgramPipeline: FnPtr,
    pub VertexArrayAttribBinding: FnPtr,
    pub VertexArrayAttribFormat: FnPtr,
    pub VertexArrayAttribIFormat: FnPtr,
    pub VertexArrayAttribLFormat: FnPtr,
    pub VertexArrayBindingDivisor: FnPtr,
    pub VertexArrayElementBuffer: FnPtr,
    pub VertexArrayVertexBuffer: FnPtr,
    pub VertexArrayVertexBuffers: FnPtr,
    pub VertexAttrib1d: FnPtr,
    pub VertexAttrib1dv: FnPtr,
    pub VertexAttrib1f: FnPtr,
    pub VertexAttrib1fv: FnPtr,
    pub VertexAttrib1s: FnPtr,
    pub VertexAttrib1sv: FnPtr,
    pub VertexAttrib2d: FnPtr,
    pub VertexAttrib2dv: FnPtr,
    pub VertexAttrib2f: FnPtr,
    pub VertexAttrib2fv: FnPtr,
    pub VertexAttrib2s: FnPtr,
    pub VertexAttrib2sv: FnPtr,
    pub VertexAttrib3d: FnPtr,
    pub VertexAttrib3dv: FnPtr,
    pub VertexAttrib3f: FnPtr,
    pub VertexAttrib3fv: FnPtr,
    pub VertexAttrib3s: FnPtr,
    pub VertexAttrib3sv: FnPtr,
    pub VertexAttrib4Nbv: FnPtr,
    pub VertexAttrib4Niv: FnPtr,
    pub VertexAttrib4Nsv: FnPtr,
    pub VertexAttrib4Nub: FnPtr,
    pub VertexAttrib4Nubv: FnPtr,
    pub VertexAttrib4Nuiv: FnPtr,
    pub VertexAttrib4Nusv: FnPtr,
    pub VertexAttrib4bv: FnPtr,
    pub VertexAttrib4d: FnPtr,
    pub VertexAttrib4dv: FnPtr,
    pub VertexAttrib4f: FnPtr,
    pub VertexAttrib4fv: FnPtr,
    pub VertexAttrib4iv: FnPtr,
    pub VertexAttrib4s: FnPtr,
    pub VertexAttrib4sv: FnPtr,
    pub VertexAttrib4ubv: FnPtr,
    pub VertexAttrib4uiv: FnPtr,
    pub VertexAttrib4usv: FnPtr,
    pub VertexAttribBinding: FnPtr,
    pub VertexAttribDivisor: FnPtr,
    pub VertexAttribFormat: FnPtr,
    pub VertexAttribI1i: FnPtr,
    pub VertexAttribI1iv: FnPtr,
    pub VertexAttribI1ui: FnPtr,
    pub VertexAttribI1uiv: FnPtr,
    pub VertexAttribI2i: FnPtr,
    pub VertexAttribI2iv: FnPtr,
    pub VertexAttribI2ui: FnPtr,
    pub VertexAttribI2uiv: FnPtr,
    pub VertexAttribI3i: FnPtr,
    pub VertexAttribI3iv: FnPtr,
    pub VertexAttribI3ui: FnPtr,
    pub VertexAttribI3uiv: FnPtr,
    pub VertexAttribI4bv: FnPtr,
    pub VertexAttribI4i: FnPtr,
    pub VertexAttribI4iv: FnPtr,
    pub VertexAttribI4sv: FnPtr,
    pub VertexAttribI4ubv: FnPtr,
    pub VertexAttribI4ui: FnPtr,
    pub VertexAttribI4uiv: FnPtr,
    pub VertexAttribI4usv: FnPtr,
    pub VertexAttribIFormat: FnPtr,
    pub VertexAttribIPointer: FnPtr,
    pub VertexAttribL1d: FnPtr,
    pub VertexAttribL1dv: FnPtr,
    pub VertexAttribL2d: FnPtr,
    pub VertexAttribL2dv: FnPtr,
    pub VertexAttribL3d: FnPtr,
    pub VertexAttribL3dv: FnPtr,
    pub VertexAttribL4d: FnPtr,
    pub VertexAttribL4dv: FnPtr,
    pub VertexAttribLFormat: FnPtr,
    pub VertexAttribLPointer: FnPtr,
    pub VertexAttribP1ui: FnPtr,
    pub VertexAttribP1uiv: FnPtr,
    pub VertexAttribP2ui: FnPtr,
    pub VertexAttribP2uiv: FnPtr,
    pub VertexAttribP3ui: FnPtr,
    pub VertexAttribP3uiv: FnPtr,
    pub VertexAttribP4ui: FnPtr,
    pub VertexAttribP4uiv: FnPtr,
    pub VertexAttribPointer: FnPtr,
    pub VertexBindingDivisor: FnPtr,
    pub VertexP2ui: FnPtr,
    pub VertexP2uiv: FnPtr,
    pub VertexP3ui: FnPtr,
    pub VertexP3uiv: FnPtr,
    pub VertexP4ui: FnPtr,
    pub VertexP4uiv: FnPtr,
    pub Viewport: FnPtr,
    pub ViewportArrayv: FnPtr,
    pub ViewportIndexedf: FnPtr,
    pub ViewportIndexedfv: FnPtr,
    pub WaitSync: FnPtr,
}

impl PointerStorage {
    pub fn new() -> PointerStorage {
        return PointerStorage {
            ActiveShaderProgram: FnPtr::new((0 as *const c_void, "glActiveShaderProgram")),
            ActiveTexture: FnPtr::new((0 as *const c_void, "glActiveTexture")),
            AttachShader: FnPtr::new((0 as *const c_void, "glAttachShader")),
            BeginConditionalRender: FnPtr::new((0 as *const c_void, "glBeginConditionalRender")),
            BeginQuery: FnPtr::new((0 as *const c_void, "glBeginQuery")),
            BeginQueryIndexed: FnPtr::new((0 as *const c_void, "glBeginQueryIndexed")),
            BeginTransformFeedback: FnPtr::new((0 as *const c_void, "glBeginTransformFeedback")),
            BindAttribLocation: FnPtr::new((0 as *const c_void, "glBindAttribLocation")),
            BindBuffer: FnPtr::new((0 as *const c_void, "glBindBuffer")),
            BindBufferBase: FnPtr::new((0 as *const c_void, "glBindBufferBase")),
            BindBufferRange: FnPtr::new((0 as *const c_void, "glBindBufferRange")),
            BindBuffersBase: FnPtr::new((0 as *const c_void, "glBindBuffersBase")),
            BindBuffersRange: FnPtr::new((0 as *const c_void, "glBindBuffersRange")),
            BindFragDataLocation: FnPtr::new((0 as *const c_void, "glBindFragDataLocation")),
            BindFragDataLocationIndexed: FnPtr::new((0 as *const c_void, "glBindFragDataLocationIndexed")),
            BindFramebuffer: FnPtr::new((0 as *const c_void, "glBindFramebuffer")),
            BindImageTexture: FnPtr::new((0 as *const c_void, "glBindImageTexture")),
            BindImageTextures: FnPtr::new((0 as *const c_void, "glBindImageTextures")),
            BindProgramPipeline: FnPtr::new((0 as *const c_void, "glBindProgramPipeline")),
            BindRenderbuffer: FnPtr::new((0 as *const c_void, "glBindRenderbuffer")),
            BindSampler: FnPtr::new((0 as *const c_void, "glBindSampler")),
            BindSamplers: FnPtr::new((0 as *const c_void, "glBindSamplers")),
            BindTexture: FnPtr::new((0 as *const c_void, "glBindTexture")),
            BindTextureUnit: FnPtr::new((0 as *const c_void, "glBindTextureUnit")),
            BindTextures: FnPtr::new((0 as *const c_void, "glBindTextures")),
            BindTransformFeedback: FnPtr::new((0 as *const c_void, "glBindTransformFeedback")),
            BindVertexArray: FnPtr::new((0 as *const c_void, "glBindVertexArray")),
            BindVertexBuffer: FnPtr::new((0 as *const c_void, "glBindVertexBuffer")),
            BindVertexBuffers: FnPtr::new((0 as *const c_void, "glBindVertexBuffers")),
            BlendColor: FnPtr::new((0 as *const c_void, "glBlendColor")),
            BlendEquation: FnPtr::new((0 as *const c_void, "glBlendEquation")),
            BlendEquationSeparate: FnPtr::new((0 as *const c_void, "glBlendEquationSeparate")),
            BlendEquationSeparatei: FnPtr::new((0 as *const c_void, "glBlendEquationSeparatei")),
            BlendEquationi: FnPtr::new((0 as *const c_void, "glBlendEquationi")),
            BlendFunc: FnPtr::new((0 as *const c_void, "glBlendFunc")),
            BlendFuncSeparate: FnPtr::new((0 as *const c_void, "glBlendFuncSeparate")),
            BlendFuncSeparatei: FnPtr::new((0 as *const c_void, "glBlendFuncSeparatei")),
            BlendFunci: FnPtr::new((0 as *const c_void, "glBlendFunci")),
            BlitFramebuffer: FnPtr::new((0 as *const c_void, "glBlitFramebuffer")),
            BlitNamedFramebuffer: FnPtr::new((0 as *const c_void, "glBlitNamedFramebuffer")),
            BufferData: FnPtr::new((0 as *const c_void, "glBufferData")),
            BufferStorage: FnPtr::new((0 as *const c_void, "glBufferStorage")),
            BufferSubData: FnPtr::new((0 as *const c_void, "glBufferSubData")),
            CheckFramebufferStatus: FnPtr::new((0 as *const c_void, "glCheckFramebufferStatus")),
            CheckNamedFramebufferStatus: FnPtr::new((0 as *const c_void, "glCheckNamedFramebufferStatus")),
            ClampColor: FnPtr::new((0 as *const c_void, "glClampColor")),
            Clear: FnPtr::new((0 as *const c_void, "glClear")),
            ClearBufferData: FnPtr::new((0 as *const c_void, "glClearBufferData")),
            ClearBufferSubData: FnPtr::new((0 as *const c_void, "glClearBufferSubData")),
            ClearBufferfi: FnPtr::new((0 as *const c_void, "glClearBufferfi")),
            ClearBufferfv: FnPtr::new((0 as *const c_void, "glClearBufferfv")),
            ClearBufferiv: FnPtr::new((0 as *const c_void, "glClearBufferiv")),
            ClearBufferuiv: FnPtr::new((0 as *const c_void, "glClearBufferuiv")),
            ClearColor: FnPtr::new((0 as *const c_void, "glClearColor")),
            ClearDepth: FnPtr::new((0 as *const c_void, "glClearDepth")),
            ClearDepthf: FnPtr::new((0 as *const c_void, "glClearDepthf")),
            ClearNamedBufferData: FnPtr::new((0 as *const c_void, "glClearNamedBufferData")),
            ClearNamedBufferSubData: FnPtr::new((0 as *const c_void, "glClearNamedBufferSubData")),
            ClearNamedFramebufferfi: FnPtr::new((0 as *const c_void, "glClearNamedFramebufferfi")),
            ClearNamedFramebufferfv: FnPtr::new((0 as *const c_void, "glClearNamedFramebufferfv")),
            ClearNamedFramebufferiv: FnPtr::new((0 as *const c_void, "glClearNamedFramebufferiv")),
            ClearNamedFramebufferuiv: FnPtr::new((0 as *const c_void, "glClearNamedFramebufferuiv")),
            ClearStencil: FnPtr::new((0 as *const c_void, "glClearStencil")),
            ClearTexImage: FnPtr::new((0 as *const c_void, "glClearTexImage")),
            ClearTexSubImage: FnPtr::new((0 as *const c_void, "glClearTexSubImage")),
            ClientWaitSync: FnPtr::new((0 as *const c_void, "glClientWaitSync")),
            ClipControl: FnPtr::new((0 as *const c_void, "glClipControl")),
            ColorMask: FnPtr::new((0 as *const c_void, "glColorMask")),
            ColorMaski: FnPtr::new((0 as *const c_void, "glColorMaski")),
            ColorP3ui: FnPtr::new((0 as *const c_void, "glColorP3ui")),
            ColorP3uiv: FnPtr::new((0 as *const c_void, "glColorP3uiv")),
            ColorP4ui: FnPtr::new((0 as *const c_void, "glColorP4ui")),
            ColorP4uiv: FnPtr::new((0 as *const c_void, "glColorP4uiv")),
            CompileShader: FnPtr::new((0 as *const c_void, "glCompileShader")),
            CompressedTexImage1D: FnPtr::new((0 as *const c_void, "glCompressedTexImage1D")),
            CompressedTexImage2D: FnPtr::new((0 as *const c_void, "glCompressedTexImage2D")),
            CompressedTexImage3D: FnPtr::new((0 as *const c_void, "glCompressedTexImage3D")),
            CompressedTexSubImage1D: FnPtr::new((0 as *const c_void, "glCompressedTexSubImage1D")),
            CompressedTexSubImage2D: FnPtr::new((0 as *const c_void, "glCompressedTexSubImage2D")),
            CompressedTexSubImage3D: FnPtr::new((0 as *const c_void, "glCompressedTexSubImage3D")),
            CompressedTextureSubImage1D: FnPtr::new((0 as *const c_void, "glCompressedTextureSubImage1D")),
            CompressedTextureSubImage2D: FnPtr::new((0 as *const c_void, "glCompressedTextureSubImage2D")),
            CompressedTextureSubImage3D: FnPtr::new((0 as *const c_void, "glCompressedTextureSubImage3D")),
            CopyBufferSubData: FnPtr::new((0 as *const c_void, "glCopyBufferSubData")),
            CopyImageSubData: FnPtr::new((0 as *const c_void, "glCopyImageSubData")),
            CopyNamedBufferSubData: FnPtr::new((0 as *const c_void, "glCopyNamedBufferSubData")),
            CopyTexImage1D: FnPtr::new((0 as *const c_void, "glCopyTexImage1D")),
            CopyTexImage2D: FnPtr::new((0 as *const c_void, "glCopyTexImage2D")),
            CopyTexSubImage1D: FnPtr::new((0 as *const c_void, "glCopyTexSubImage1D")),
            CopyTexSubImage2D: FnPtr::new((0 as *const c_void, "glCopyTexSubImage2D")),
            CopyTexSubImage3D: FnPtr::new((0 as *const c_void, "glCopyTexSubImage3D")),
            CopyTextureSubImage1D: FnPtr::new((0 as *const c_void, "glCopyTextureSubImage1D")),
            CopyTextureSubImage2D: FnPtr::new((0 as *const c_void, "glCopyTextureSubImage2D")),
            CopyTextureSubImage3D: FnPtr::new((0 as *const c_void, "glCopyTextureSubImage3D")),
            CreateBuffers: FnPtr::new((0 as *const c_void, "glCreateBuffers")),
            CreateFramebuffers: FnPtr::new((0 as *const c_void, "glCreateFramebuffers")),
            CreateProgram: FnPtr::new((0 as *const c_void, "glCreateProgram")),
            CreateProgramPipelines: FnPtr::new((0 as *const c_void, "glCreateProgramPipelines")),
            CreateQueries: FnPtr::new((0 as *const c_void, "glCreateQueries")),
            CreateRenderbuffers: FnPtr::new((0 as *const c_void, "glCreateRenderbuffers")),
            CreateSamplers: FnPtr::new((0 as *const c_void, "glCreateSamplers")),
            CreateShader: FnPtr::new((0 as *const c_void, "glCreateShader")),
            CreateShaderProgramv: FnPtr::new((0 as *const c_void, "glCreateShaderProgramv")),
            CreateTextures: FnPtr::new((0 as *const c_void, "glCreateTextures")),
            CreateTransformFeedbacks: FnPtr::new((0 as *const c_void, "glCreateTransformFeedbacks")),
            CreateVertexArrays: FnPtr::new((0 as *const c_void, "glCreateVertexArrays")),
            CullFace: FnPtr::new((0 as *const c_void, "glCullFace")),
            DebugMessageCallback: FnPtr::new((0 as *const c_void, "glDebugMessageCallback")),
            DebugMessageControl: FnPtr::new((0 as *const c_void, "glDebugMessageControl")),
            DebugMessageInsert: FnPtr::new((0 as *const c_void, "glDebugMessageInsert")),
            DeleteBuffers: FnPtr::new((0 as *const c_void, "glDeleteBuffers")),
            DeleteFramebuffers: FnPtr::new((0 as *const c_void, "glDeleteFramebuffers")),
            DeleteProgram: FnPtr::new((0 as *const c_void, "glDeleteProgram")),
            DeleteProgramPipelines: FnPtr::new((0 as *const c_void, "glDeleteProgramPipelines")),
            DeleteQueries: FnPtr::new((0 as *const c_void, "glDeleteQueries")),
            DeleteRenderbuffers: FnPtr::new((0 as *const c_void, "glDeleteRenderbuffers")),
            DeleteSamplers: FnPtr::new((0 as *const c_void, "glDeleteSamplers")),
            DeleteShader: FnPtr::new((0 as *const c_void, "glDeleteShader")),
            DeleteSync: FnPtr::new((0 as *const c_void, "glDeleteSync")),
            DeleteTextures: FnPtr::new((0 as *const c_void, "glDeleteTextures")),
            DeleteTransformFeedbacks: FnPtr::new((0 as *const c_void, "glDeleteTransformFeedbacks")),
            DeleteVertexArrays: FnPtr::new((0 as *const c_void, "glDeleteVertexArrays")),
            DepthFunc: FnPtr::new((0 as *const c_void, "glDepthFunc")),
            DepthMask: FnPtr::new((0 as *const c_void, "glDepthMask")),
            DepthRange: FnPtr::new((0 as *const c_void, "glDepthRange")),
            DepthRangeArrayv: FnPtr::new((0 as *const c_void, "glDepthRangeArrayv")),
            DepthRangeIndexed: FnPtr::new((0 as *const c_void, "glDepthRangeIndexed")),
            DepthRangef: FnPtr::new((0 as *const c_void, "glDepthRangef")),
            DetachShader: FnPtr::new((0 as *const c_void, "glDetachShader")),
            Disable: FnPtr::new((0 as *const c_void, "glDisable")),
            DisableVertexArrayAttrib: FnPtr::new((0 as *const c_void, "glDisableVertexArrayAttrib")),
            DisableVertexAttribArray: FnPtr::new((0 as *const c_void, "glDisableVertexAttribArray")),
            Disablei: FnPtr::new((0 as *const c_void, "glDisablei")),
            DispatchCompute: FnPtr::new((0 as *const c_void, "glDispatchCompute")),
            DispatchComputeIndirect: FnPtr::new((0 as *const c_void, "glDispatchComputeIndirect")),
            DrawArrays: FnPtr::new((0 as *const c_void, "glDrawArrays")),
            DrawArraysIndirect: FnPtr::new((0 as *const c_void, "glDrawArraysIndirect")),
            DrawArraysInstanced: FnPtr::new((0 as *const c_void, "glDrawArraysInstanced")),
            DrawArraysInstancedBaseInstance: FnPtr::new((0 as *const c_void, "glDrawArraysInstancedBaseInstance")),
            DrawBuffer: FnPtr::new((0 as *const c_void, "glDrawBuffer")),
            DrawBuffers: FnPtr::new((0 as *const c_void, "glDrawBuffers")),
            DrawElements: FnPtr::new((0 as *const c_void, "glDrawElements")),
            DrawElementsBaseVertex: FnPtr::new((0 as *const c_void, "glDrawElementsBaseVertex")),
            DrawElementsIndirect: FnPtr::new((0 as *const c_void, "glDrawElementsIndirect")),
            DrawElementsInstanced: FnPtr::new((0 as *const c_void, "glDrawElementsInstanced")),
            DrawElementsInstancedBaseInstance: FnPtr::new((0 as *const c_void, "glDrawElementsInstancedBaseInstance")),
            DrawElementsInstancedBaseVertex: FnPtr::new((0 as *const c_void, "glDrawElementsInstancedBaseVertex")),
            DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new((0 as *const c_void, "glDrawElementsInstancedBaseVertexBaseInstance")),
            DrawRangeElements: FnPtr::new((0 as *const c_void, "glDrawRangeElements")),
            DrawRangeElementsBaseVertex: FnPtr::new((0 as *const c_void, "glDrawRangeElementsBaseVertex")),
            DrawTransformFeedback: FnPtr::new((0 as *const c_void, "glDrawTransformFeedback")),
            DrawTransformFeedbackInstanced: FnPtr::new((0 as *const c_void, "glDrawTransformFeedbackInstanced")),
            DrawTransformFeedbackStream: FnPtr::new((0 as *const c_void, "glDrawTransformFeedbackStream")),
            DrawTransformFeedbackStreamInstanced: FnPtr::new((0 as *const c_void, "glDrawTransformFeedbackStreamInstanced")),
            Enable: FnPtr::new((0 as *const c_void, "glEnable")),
            EnableVertexArrayAttrib: FnPtr::new((0 as *const c_void, "glEnableVertexArrayAttrib")),
            EnableVertexAttribArray: FnPtr::new((0 as *const c_void, "glEnableVertexAttribArray")),
            Enablei: FnPtr::new((0 as *const c_void, "glEnablei")),
            EndConditionalRender: FnPtr::new((0 as *const c_void, "glEndConditionalRender")),
            EndQuery: FnPtr::new((0 as *const c_void, "glEndQuery")),
            EndQueryIndexed: FnPtr::new((0 as *const c_void, "glEndQueryIndexed")),
            EndTransformFeedback: FnPtr::new((0 as *const c_void, "glEndTransformFeedback")),
            FenceSync: FnPtr::new((0 as *const c_void, "glFenceSync")),
            Finish: FnPtr::new((0 as *const c_void, "glFinish")),
            Flush: FnPtr::new((0 as *const c_void, "glFlush")),
            FlushMappedBufferRange: FnPtr::new((0 as *const c_void, "glFlushMappedBufferRange")),
            FlushMappedNamedBufferRange: FnPtr::new((0 as *const c_void, "glFlushMappedNamedBufferRange")),
            FramebufferParameteri: FnPtr::new((0 as *const c_void, "glFramebufferParameteri")),
            FramebufferRenderbuffer: FnPtr::new((0 as *const c_void, "glFramebufferRenderbuffer")),
            FramebufferTexture: FnPtr::new((0 as *const c_void, "glFramebufferTexture")),
            FramebufferTexture1D: FnPtr::new((0 as *const c_void, "glFramebufferTexture1D")),
            FramebufferTexture2D: FnPtr::new((0 as *const c_void, "glFramebufferTexture2D")),
            FramebufferTexture3D: FnPtr::new((0 as *const c_void, "glFramebufferTexture3D")),
            FramebufferTextureLayer: FnPtr::new((0 as *const c_void, "glFramebufferTextureLayer")),
            FrontFace: FnPtr::new((0 as *const c_void, "glFrontFace")),
            GenBuffers: FnPtr::new((0 as *const c_void, "glGenBuffers")),
            GenFramebuffers: FnPtr::new((0 as *const c_void, "glGenFramebuffers")),
            GenProgramPipelines: FnPtr::new((0 as *const c_void, "glGenProgramPipelines")),
            GenQueries: FnPtr::new((0 as *const c_void, "glGenQueries")),
            GenRenderbuffers: FnPtr::new((0 as *const c_void, "glGenRenderbuffers")),
            GenSamplers: FnPtr::new((0 as *const c_void, "glGenSamplers")),
            GenTextures: FnPtr::new((0 as *const c_void, "glGenTextures")),
            GenTransformFeedbacks: FnPtr::new((0 as *const c_void, "glGenTransformFeedbacks")),
            GenVertexArrays: FnPtr::new((0 as *const c_void, "glGenVertexArrays")),
            GenerateMipmap: FnPtr::new((0 as *const c_void, "glGenerateMipmap")),
            GenerateTextureMipmap: FnPtr::new((0 as *const c_void, "glGenerateTextureMipmap")),
            GetActiveAtomicCounterBufferiv: FnPtr::new((0 as *const c_void, "glGetActiveAtomicCounterBufferiv")),
            GetActiveAttrib: FnPtr::new((0 as *const c_void, "glGetActiveAttrib")),
            GetActiveSubroutineName: FnPtr::new((0 as *const c_void, "glGetActiveSubroutineName")),
            GetActiveSubroutineUniformName: FnPtr::new((0 as *const c_void, "glGetActiveSubroutineUniformName")),
            GetActiveSubroutineUniformiv: FnPtr::new((0 as *const c_void, "glGetActiveSubroutineUniformiv")),
            GetActiveUniform: FnPtr::new((0 as *const c_void, "glGetActiveUniform")),
            GetActiveUniformBlockName: FnPtr::new((0 as *const c_void, "glGetActiveUniformBlockName")),
            GetActiveUniformBlockiv: FnPtr::new((0 as *const c_void, "glGetActiveUniformBlockiv")),
            GetActiveUniformName: FnPtr::new((0 as *const c_void, "glGetActiveUniformName")),
            GetActiveUniformsiv: FnPtr::new((0 as *const c_void, "glGetActiveUniformsiv")),
            GetAttachedShaders: FnPtr::new((0 as *const c_void, "glGetAttachedShaders")),
            GetAttribLocation: FnPtr::new((0 as *const c_void, "glGetAttribLocation")),
            GetBooleani_v: FnPtr::new((0 as *const c_void, "glGetBooleani_v")),
            GetBooleanv: FnPtr::new((0 as *const c_void, "glGetBooleanv")),
            GetBufferParameteri64v: FnPtr::new((0 as *const c_void, "glGetBufferParameteri64v")),
            GetBufferParameteriv: FnPtr::new((0 as *const c_void, "glGetBufferParameteriv")),
            GetBufferPointerv: FnPtr::new((0 as *const c_void, "glGetBufferPointerv")),
            GetBufferSubData: FnPtr::new((0 as *const c_void, "glGetBufferSubData")),
            GetCompressedTexImage: FnPtr::new((0 as *const c_void, "glGetCompressedTexImage")),
            GetCompressedTextureImage: FnPtr::new((0 as *const c_void, "glGetCompressedTextureImage")),
            GetCompressedTextureSubImage: FnPtr::new((0 as *const c_void, "glGetCompressedTextureSubImage")),
            GetDebugMessageLog: FnPtr::new((0 as *const c_void, "glGetDebugMessageLog")),
            GetDoublei_v: FnPtr::new((0 as *const c_void, "glGetDoublei_v")),
            GetDoublev: FnPtr::new((0 as *const c_void, "glGetDoublev")),
            GetError: FnPtr::new((0 as *const c_void, "glGetError")),
            GetFloati_v: FnPtr::new((0 as *const c_void, "glGetFloati_v")),
            GetFloatv: FnPtr::new((0 as *const c_void, "glGetFloatv")),
            GetFragDataIndex: FnPtr::new((0 as *const c_void, "glGetFragDataIndex")),
            GetFragDataLocation: FnPtr::new((0 as *const c_void, "glGetFragDataLocation")),
            GetFramebufferAttachmentParameteriv: FnPtr::new((0 as *const c_void, "glGetFramebufferAttachmentParameteriv")),
            GetFramebufferParameteriv: FnPtr::new((0 as *const c_void, "glGetFramebufferParameteriv")),
            GetGraphicsResetStatus: FnPtr::new((0 as *const c_void, "glGetGraphicsResetStatus")),
            GetInteger64i_v: FnPtr::new((0 as *const c_void, "glGetInteger64i_v")),
            GetInteger64v: FnPtr::new((0 as *const c_void, "glGetInteger64v")),
            GetIntegeri_v: FnPtr::new((0 as *const c_void, "glGetIntegeri_v")),
            GetIntegerv: FnPtr::new((0 as *const c_void, "glGetIntegerv")),
            GetInternalformati64v: FnPtr::new((0 as *const c_void, "glGetInternalformati64v")),
            GetInternalformativ: FnPtr::new((0 as *const c_void, "glGetInternalformativ")),
            GetMultisamplefv: FnPtr::new((0 as *const c_void, "glGetMultisamplefv")),
            GetNamedBufferParameteri64v: FnPtr::new((0 as *const c_void, "glGetNamedBufferParameteri64v")),
            GetNamedBufferParameteriv: FnPtr::new((0 as *const c_void, "glGetNamedBufferParameteriv")),
            GetNamedBufferPointerv: FnPtr::new((0 as *const c_void, "glGetNamedBufferPointerv")),
            GetNamedBufferSubData: FnPtr::new((0 as *const c_void, "glGetNamedBufferSubData")),
            GetNamedFramebufferAttachmentParameteriv: FnPtr::new((0 as *const c_void, "glGetNamedFramebufferAttachmentParameteriv")),
            GetNamedFramebufferParameteriv: FnPtr::new((0 as *const c_void, "glGetNamedFramebufferParameteriv")),
            GetNamedRenderbufferParameteriv: FnPtr::new((0 as *const c_void, "glGetNamedRenderbufferParameteriv")),
            GetObjectLabel: FnPtr::new((0 as *const c_void, "glGetObjectLabel")),
            GetObjectPtrLabel: FnPtr::new((0 as *const c_void, "glGetObjectPtrLabel")),
            GetPointerv: FnPtr::new((0 as *const c_void, "glGetPointerv")),
            GetProgramBinary: FnPtr::new((0 as *const c_void, "glGetProgramBinary")),
            GetProgramInfoLog: FnPtr::new((0 as *const c_void, "glGetProgramInfoLog")),
            GetProgramInterfaceiv: FnPtr::new((0 as *const c_void, "glGetProgramInterfaceiv")),
            GetProgramPipelineInfoLog: FnPtr::new((0 as *const c_void, "glGetProgramPipelineInfoLog")),
            GetProgramPipelineiv: FnPtr::new((0 as *const c_void, "glGetProgramPipelineiv")),
            GetProgramResourceIndex: FnPtr::new((0 as *const c_void, "glGetProgramResourceIndex")),
            GetProgramResourceLocation: FnPtr::new((0 as *const c_void, "glGetProgramResourceLocation")),
            GetProgramResourceLocationIndex: FnPtr::new((0 as *const c_void, "glGetProgramResourceLocationIndex")),
            GetProgramResourceName: FnPtr::new((0 as *const c_void, "glGetProgramResourceName")),
            GetProgramResourceiv: FnPtr::new((0 as *const c_void, "glGetProgramResourceiv")),
            GetProgramStageiv: FnPtr::new((0 as *const c_void, "glGetProgramStageiv")),
            GetProgramiv: FnPtr::new((0 as *const c_void, "glGetProgramiv")),
            GetQueryBufferObjecti64v: FnPtr::new((0 as *const c_void, "glGetQueryBufferObjecti64v")),
            GetQueryBufferObjectiv: FnPtr::new((0 as *const c_void, "glGetQueryBufferObjectiv")),
            GetQueryBufferObjectui64v: FnPtr::new((0 as *const c_void, "glGetQueryBufferObjectui64v")),
            GetQueryBufferObjectuiv: FnPtr::new((0 as *const c_void, "glGetQueryBufferObjectuiv")),
            GetQueryIndexediv: FnPtr::new((0 as *const c_void, "glGetQueryIndexediv")),
            GetQueryObjecti64v: FnPtr::new((0 as *const c_void, "glGetQueryObjecti64v")),
            GetQueryObjectiv: FnPtr::new((0 as *const c_void, "glGetQueryObjectiv")),
            GetQueryObjectui64v: FnPtr::new((0 as *const c_void, "glGetQueryObjectui64v")),
            GetQueryObjectuiv: FnPtr::new((0 as *const c_void, "glGetQueryObjectuiv")),
            GetQueryiv: FnPtr::new((0 as *const c_void, "glGetQueryiv")),
            GetRenderbufferParameteriv: FnPtr::new((0 as *const c_void, "glGetRenderbufferParameteriv")),
            GetSamplerParameterIiv: FnPtr::new((0 as *const c_void, "glGetSamplerParameterIiv")),
            GetSamplerParameterIuiv: FnPtr::new((0 as *const c_void, "glGetSamplerParameterIuiv")),
            GetSamplerParameterfv: FnPtr::new((0 as *const c_void, "glGetSamplerParameterfv")),
            GetSamplerParameteriv: FnPtr::new((0 as *const c_void, "glGetSamplerParameteriv")),
            GetShaderInfoLog: FnPtr::new((0 as *const c_void, "glGetShaderInfoLog")),
            GetShaderPrecisionFormat: FnPtr::new((0 as *const c_void, "glGetShaderPrecisionFormat")),
            GetShaderSource: FnPtr::new((0 as *const c_void, "glGetShaderSource")),
            GetShaderiv: FnPtr::new((0 as *const c_void, "glGetShaderiv")),
            GetString: FnPtr::new((0 as *const c_void, "glGetString")),
            GetStringi: FnPtr::new((0 as *const c_void, "glGetStringi")),
            GetSubroutineIndex: FnPtr::new((0 as *const c_void, "glGetSubroutineIndex")),
            GetSubroutineUniformLocation: FnPtr::new((0 as *const c_void, "glGetSubroutineUniformLocation")),
            GetSynciv: FnPtr::new((0 as *const c_void, "glGetSynciv")),
            GetTexImage: FnPtr::new((0 as *const c_void, "glGetTexImage")),
            GetTexLevelParameterfv: FnPtr::new((0 as *const c_void, "glGetTexLevelParameterfv")),
            GetTexLevelParameteriv: FnPtr::new((0 as *const c_void, "glGetTexLevelParameteriv")),
            GetTexParameterIiv: FnPtr::new((0 as *const c_void, "glGetTexParameterIiv")),
            GetTexParameterIuiv: FnPtr::new((0 as *const c_void, "glGetTexParameterIuiv")),
            GetTexParameterfv: FnPtr::new((0 as *const c_void, "glGetTexParameterfv")),
            GetTexParameteriv: FnPtr::new((0 as *const c_void, "glGetTexParameteriv")),
            GetTextureImage: FnPtr::new((0 as *const c_void, "glGetTextureImage")),
            GetTextureLevelParameterfv: FnPtr::new((0 as *const c_void, "glGetTextureLevelParameterfv")),
            GetTextureLevelParameteriv: FnPtr::new((0 as *const c_void, "glGetTextureLevelParameteriv")),
            GetTextureParameterIiv: FnPtr::new((0 as *const c_void, "glGetTextureParameterIiv")),
            GetTextureParameterIuiv: FnPtr::new((0 as *const c_void, "glGetTextureParameterIuiv")),
            GetTextureParameterfv: FnPtr::new((0 as *const c_void, "glGetTextureParameterfv")),
            GetTextureParameteriv: FnPtr::new((0 as *const c_void, "glGetTextureParameteriv")),
            GetTextureSubImage: FnPtr::new((0 as *const c_void, "glGetTextureSubImage")),
            GetTransformFeedbackVarying: FnPtr::new((0 as *const c_void, "glGetTransformFeedbackVarying")),
            GetTransformFeedbacki64_v: FnPtr::new((0 as *const c_void, "glGetTransformFeedbacki64_v")),
            GetTransformFeedbacki_v: FnPtr::new((0 as *const c_void, "glGetTransformFeedbacki_v")),
            GetTransformFeedbackiv: FnPtr::new((0 as *const c_void, "glGetTransformFeedbackiv")),
            GetUniformBlockIndex: FnPtr::new((0 as *const c_void, "glGetUniformBlockIndex")),
            GetUniformIndices: FnPtr::new((0 as *const c_void, "glGetUniformIndices")),
            GetUniformLocation: FnPtr::new((0 as *const c_void, "glGetUniformLocation")),
            GetUniformSubroutineuiv: FnPtr::new((0 as *const c_void, "glGetUniformSubroutineuiv")),
            GetUniformdv: FnPtr::new((0 as *const c_void, "glGetUniformdv")),
            GetUniformfv: FnPtr::new((0 as *const c_void, "glGetUniformfv")),
            GetUniformiv: FnPtr::new((0 as *const c_void, "glGetUniformiv")),
            GetUniformuiv: FnPtr::new((0 as *const c_void, "glGetUniformuiv")),
            GetVertexArrayIndexed64iv: FnPtr::new((0 as *const c_void, "glGetVertexArrayIndexed64iv")),
            GetVertexArrayIndexediv: FnPtr::new((0 as *const c_void, "glGetVertexArrayIndexediv")),
            GetVertexArrayiv: FnPtr::new((0 as *const c_void, "glGetVertexArrayiv")),
            GetVertexAttribIiv: FnPtr::new((0 as *const c_void, "glGetVertexAttribIiv")),
            GetVertexAttribIuiv: FnPtr::new((0 as *const c_void, "glGetVertexAttribIuiv")),
            GetVertexAttribLdv: FnPtr::new((0 as *const c_void, "glGetVertexAttribLdv")),
            GetVertexAttribPointerv: FnPtr::new((0 as *const c_void, "glGetVertexAttribPointerv")),
            GetVertexAttribdv: FnPtr::new((0 as *const c_void, "glGetVertexAttribdv")),
            GetVertexAttribfv: FnPtr::new((0 as *const c_void, "glGetVertexAttribfv")),
            GetVertexAttribiv: FnPtr::new((0 as *const c_void, "glGetVertexAttribiv")),
            GetnColorTable: FnPtr::new((0 as *const c_void, "glGetnColorTable")),
            GetnCompressedTexImage: FnPtr::new((0 as *const c_void, "glGetnCompressedTexImage")),
            GetnConvolutionFilter: FnPtr::new((0 as *const c_void, "glGetnConvolutionFilter")),
            GetnHistogram: FnPtr::new((0 as *const c_void, "glGetnHistogram")),
            GetnMapdv: FnPtr::new((0 as *const c_void, "glGetnMapdv")),
            GetnMapfv: FnPtr::new((0 as *const c_void, "glGetnMapfv")),
            GetnMapiv: FnPtr::new((0 as *const c_void, "glGetnMapiv")),
            GetnMinmax: FnPtr::new((0 as *const c_void, "glGetnMinmax")),
            GetnPixelMapfv: FnPtr::new((0 as *const c_void, "glGetnPixelMapfv")),
            GetnPixelMapuiv: FnPtr::new((0 as *const c_void, "glGetnPixelMapuiv")),
            GetnPixelMapusv: FnPtr::new((0 as *const c_void, "glGetnPixelMapusv")),
            GetnPolygonStipple: FnPtr::new((0 as *const c_void, "glGetnPolygonStipple")),
            GetnSeparableFilter: FnPtr::new((0 as *const c_void, "glGetnSeparableFilter")),
            GetnTexImage: FnPtr::new((0 as *const c_void, "glGetnTexImage")),
            GetnUniformdv: FnPtr::new((0 as *const c_void, "glGetnUniformdv")),
            GetnUniformfv: FnPtr::new((0 as *const c_void, "glGetnUniformfv")),
            GetnUniformiv: FnPtr::new((0 as *const c_void, "glGetnUniformiv")),
            GetnUniformuiv: FnPtr::new((0 as *const c_void, "glGetnUniformuiv")),
            Hint: FnPtr::new((0 as *const c_void, "glHint")),
            InvalidateBufferData: FnPtr::new((0 as *const c_void, "glInvalidateBufferData")),
            InvalidateBufferSubData: FnPtr::new((0 as *const c_void, "glInvalidateBufferSubData")),
            InvalidateFramebuffer: FnPtr::new((0 as *const c_void, "glInvalidateFramebuffer")),
            InvalidateNamedFramebufferData: FnPtr::new((0 as *const c_void, "glInvalidateNamedFramebufferData")),
            InvalidateNamedFramebufferSubData: FnPtr::new((0 as *const c_void, "glInvalidateNamedFramebufferSubData")),
            InvalidateSubFramebuffer: FnPtr::new((0 as *const c_void, "glInvalidateSubFramebuffer")),
            InvalidateTexImage: FnPtr::new((0 as *const c_void, "glInvalidateTexImage")),
            InvalidateTexSubImage: FnPtr::new((0 as *const c_void, "glInvalidateTexSubImage")),
            IsBuffer: FnPtr::new((0 as *const c_void, "glIsBuffer")),
            IsEnabled: FnPtr::new((0 as *const c_void, "glIsEnabled")),
            IsEnabledi: FnPtr::new((0 as *const c_void, "glIsEnabledi")),
            IsFramebuffer: FnPtr::new((0 as *const c_void, "glIsFramebuffer")),
            IsProgram: FnPtr::new((0 as *const c_void, "glIsProgram")),
            IsProgramPipeline: FnPtr::new((0 as *const c_void, "glIsProgramPipeline")),
            IsQuery: FnPtr::new((0 as *const c_void, "glIsQuery")),
            IsRenderbuffer: FnPtr::new((0 as *const c_void, "glIsRenderbuffer")),
            IsSampler: FnPtr::new((0 as *const c_void, "glIsSampler")),
            IsShader: FnPtr::new((0 as *const c_void, "glIsShader")),
            IsSync: FnPtr::new((0 as *const c_void, "glIsSync")),
            IsTexture: FnPtr::new((0 as *const c_void, "glIsTexture")),
            IsTransformFeedback: FnPtr::new((0 as *const c_void, "glIsTransformFeedback")),
            IsVertexArray: FnPtr::new((0 as *const c_void, "glIsVertexArray")),
            LineWidth: FnPtr::new((0 as *const c_void, "glLineWidth")),
            LinkProgram: FnPtr::new((0 as *const c_void, "glLinkProgram")),
            LogicOp: FnPtr::new((0 as *const c_void, "glLogicOp")),
            MapBuffer: FnPtr::new((0 as *const c_void, "glMapBuffer")),
            MapBufferRange: FnPtr::new((0 as *const c_void, "glMapBufferRange")),
            MapNamedBuffer: FnPtr::new((0 as *const c_void, "glMapNamedBuffer")),
            MapNamedBufferRange: FnPtr::new((0 as *const c_void, "glMapNamedBufferRange")),
            MemoryBarrier: FnPtr::new((0 as *const c_void, "glMemoryBarrier")),
            MemoryBarrierByRegion: FnPtr::new((0 as *const c_void, "glMemoryBarrierByRegion")),
            MinSampleShading: FnPtr::new((0 as *const c_void, "glMinSampleShading")),
            MultiDrawArrays: FnPtr::new((0 as *const c_void, "glMultiDrawArrays")),
            MultiDrawArraysIndirect: FnPtr::new((0 as *const c_void, "glMultiDrawArraysIndirect")),
            MultiDrawElements: FnPtr::new((0 as *const c_void, "glMultiDrawElements")),
            MultiDrawElementsBaseVertex: FnPtr::new((0 as *const c_void, "glMultiDrawElementsBaseVertex")),
            MultiDrawElementsIndirect: FnPtr::new((0 as *const c_void, "glMultiDrawElementsIndirect")),
            MultiTexCoordP1ui: FnPtr::new((0 as *const c_void, "glMultiTexCoordP1ui")),
            MultiTexCoordP1uiv: FnPtr::new((0 as *const c_void, "glMultiTexCoordP1uiv")),
            MultiTexCoordP2ui: FnPtr::new((0 as *const c_void, "glMultiTexCoordP2ui")),
            MultiTexCoordP2uiv: FnPtr::new((0 as *const c_void, "glMultiTexCoordP2uiv")),
            MultiTexCoordP3ui: FnPtr::new((0 as *const c_void, "glMultiTexCoordP3ui")),
            MultiTexCoordP3uiv: FnPtr::new((0 as *const c_void, "glMultiTexCoordP3uiv")),
            MultiTexCoordP4ui: FnPtr::new((0 as *const c_void, "glMultiTexCoordP4ui")),
            MultiTexCoordP4uiv: FnPtr::new((0 as *const c_void, "glMultiTexCoordP4uiv")),
            NamedBufferData: FnPtr::new((0 as *const c_void, "glNamedBufferData")),
            NamedBufferStorage: FnPtr::new((0 as *const c_void, "glNamedBufferStorage")),
            NamedBufferSubData: FnPtr::new((0 as *const c_void, "glNamedBufferSubData")),
            NamedFramebufferDrawBuffer: FnPtr::new((0 as *const c_void, "glNamedFramebufferDrawBuffer")),
            NamedFramebufferDrawBuffers: FnPtr::new((0 as *const c_void, "glNamedFramebufferDrawBuffers")),
            NamedFramebufferParameteri: FnPtr::new((0 as *const c_void, "glNamedFramebufferParameteri")),
            NamedFramebufferReadBuffer: FnPtr::new((0 as *const c_void, "glNamedFramebufferReadBuffer")),
            NamedFramebufferRenderbuffer: FnPtr::new((0 as *const c_void, "glNamedFramebufferRenderbuffer")),
            NamedFramebufferTexture: FnPtr::new((0 as *const c_void, "glNamedFramebufferTexture")),
            NamedFramebufferTextureLayer: FnPtr::new((0 as *const c_void, "glNamedFramebufferTextureLayer")),
            NamedRenderbufferStorage: FnPtr::new((0 as *const c_void, "glNamedRenderbufferStorage")),
            NamedRenderbufferStorageMultisample: FnPtr::new((0 as *const c_void, "glNamedRenderbufferStorageMultisample")),
            NormalP3ui: FnPtr::new((0 as *const c_void, "glNormalP3ui")),
            NormalP3uiv: FnPtr::new((0 as *const c_void, "glNormalP3uiv")),
            ObjectLabel: FnPtr::new((0 as *const c_void, "glObjectLabel")),
            ObjectPtrLabel: FnPtr::new((0 as *const c_void, "glObjectPtrLabel")),
            PatchParameterfv: FnPtr::new((0 as *const c_void, "glPatchParameterfv")),
            PatchParameteri: FnPtr::new((0 as *const c_void, "glPatchParameteri")),
            PauseTransformFeedback: FnPtr::new((0 as *const c_void, "glPauseTransformFeedback")),
            PixelStoref: FnPtr::new((0 as *const c_void, "glPixelStoref")),
            PixelStorei: FnPtr::new((0 as *const c_void, "glPixelStorei")),
            PointParameterf: FnPtr::new((0 as *const c_void, "glPointParameterf")),
            PointParameterfv: FnPtr::new((0 as *const c_void, "glPointParameterfv")),
            PointParameteri: FnPtr::new((0 as *const c_void, "glPointParameteri")),
            PointParameteriv: FnPtr::new((0 as *const c_void, "glPointParameteriv")),
            PointSize: FnPtr::new((0 as *const c_void, "glPointSize")),
            PolygonMode: FnPtr::new((0 as *const c_void, "glPolygonMode")),
            PolygonOffset: FnPtr::new((0 as *const c_void, "glPolygonOffset")),
            PopDebugGroup: FnPtr::new((0 as *const c_void, "glPopDebugGroup")),
            PrimitiveRestartIndex: FnPtr::new((0 as *const c_void, "glPrimitiveRestartIndex")),
            ProgramBinary: FnPtr::new((0 as *const c_void, "glProgramBinary")),
            ProgramParameteri: FnPtr::new((0 as *const c_void, "glProgramParameteri")),
            ProgramUniform1d: FnPtr::new((0 as *const c_void, "glProgramUniform1d")),
            ProgramUniform1dv: FnPtr::new((0 as *const c_void, "glProgramUniform1dv")),
            ProgramUniform1f: FnPtr::new((0 as *const c_void, "glProgramUniform1f")),
            ProgramUniform1fv: FnPtr::new((0 as *const c_void, "glProgramUniform1fv")),
            ProgramUniform1i: FnPtr::new((0 as *const c_void, "glProgramUniform1i")),
            ProgramUniform1iv: FnPtr::new((0 as *const c_void, "glProgramUniform1iv")),
            ProgramUniform1ui: FnPtr::new((0 as *const c_void, "glProgramUniform1ui")),
            ProgramUniform1uiv: FnPtr::new((0 as *const c_void, "glProgramUniform1uiv")),
            ProgramUniform2d: FnPtr::new((0 as *const c_void, "glProgramUniform2d")),
            ProgramUniform2dv: FnPtr::new((0 as *const c_void, "glProgramUniform2dv")),
            ProgramUniform2f: FnPtr::new((0 as *const c_void, "glProgramUniform2f")),
            ProgramUniform2fv: FnPtr::new((0 as *const c_void, "glProgramUniform2fv")),
            ProgramUniform2i: FnPtr::new((0 as *const c_void, "glProgramUniform2i")),
            ProgramUniform2iv: FnPtr::new((0 as *const c_void, "glProgramUniform2iv")),
            ProgramUniform2ui: FnPtr::new((0 as *const c_void, "glProgramUniform2ui")),
            ProgramUniform2uiv: FnPtr::new((0 as *const c_void, "glProgramUniform2uiv")),
            ProgramUniform3d: FnPtr::new((0 as *const c_void, "glProgramUniform3d")),
            ProgramUniform3dv: FnPtr::new((0 as *const c_void, "glProgramUniform3dv")),
            ProgramUniform3f: FnPtr::new((0 as *const c_void, "glProgramUniform3f")),
            ProgramUniform3fv: FnPtr::new((0 as *const c_void, "glProgramUniform3fv")),
            ProgramUniform3i: FnPtr::new((0 as *const c_void, "glProgramUniform3i")),
            ProgramUniform3iv: FnPtr::new((0 as *const c_void, "glProgramUniform3iv")),
            ProgramUniform3ui: FnPtr::new((0 as *const c_void, "glProgramUniform3ui")),
            ProgramUniform3uiv: FnPtr::new((0 as *const c_void, "glProgramUniform3uiv")),
            ProgramUniform4d: FnPtr::new((0 as *const c_void, "glProgramUniform4d")),
            ProgramUniform4dv: FnPtr::new((0 as *const c_void, "glProgramUniform4dv")),
            ProgramUniform4f: FnPtr::new((0 as *const c_void, "glProgramUniform4f")),
            ProgramUniform4fv: FnPtr::new((0 as *const c_void, "glProgramUniform4fv")),
            ProgramUniform4i: FnPtr::new((0 as *const c_void, "glProgramUniform4i")),
            ProgramUniform4iv: FnPtr::new((0 as *const c_void, "glProgramUniform4iv")),
            ProgramUniform4ui: FnPtr::new((0 as *const c_void, "glProgramUniform4ui")),
            ProgramUniform4uiv: FnPtr::new((0 as *const c_void, "glProgramUniform4uiv")),
            ProgramUniformMatrix2dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2dv")),
            ProgramUniformMatrix2fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2fv")),
            ProgramUniformMatrix2x3dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2x3dv")),
            ProgramUniformMatrix2x3fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2x3fv")),
            ProgramUniformMatrix2x4dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2x4dv")),
            ProgramUniformMatrix2x4fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix2x4fv")),
            ProgramUniformMatrix3dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3dv")),
            ProgramUniformMatrix3fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3fv")),
            ProgramUniformMatrix3x2dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3x2dv")),
            ProgramUniformMatrix3x2fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3x2fv")),
            ProgramUniformMatrix3x4dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3x4dv")),
            ProgramUniformMatrix3x4fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix3x4fv")),
            ProgramUniformMatrix4dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4dv")),
            ProgramUniformMatrix4fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4fv")),
            ProgramUniformMatrix4x2dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4x2dv")),
            ProgramUniformMatrix4x2fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4x2fv")),
            ProgramUniformMatrix4x3dv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4x3dv")),
            ProgramUniformMatrix4x3fv: FnPtr::new((0 as *const c_void, "glProgramUniformMatrix4x3fv")),
            ProvokingVertex: FnPtr::new((0 as *const c_void, "glProvokingVertex")),
            PushDebugGroup: FnPtr::new((0 as *const c_void, "glPushDebugGroup")),
            QueryCounter: FnPtr::new((0 as *const c_void, "glQueryCounter")),
            ReadBuffer: FnPtr::new((0 as *const c_void, "glReadBuffer")),
            ReadPixels: FnPtr::new((0 as *const c_void, "glReadPixels")),
            ReadnPixels: FnPtr::new((0 as *const c_void, "glReadnPixels")),
            ReleaseShaderCompiler: FnPtr::new((0 as *const c_void, "glReleaseShaderCompiler")),
            RenderbufferStorage: FnPtr::new((0 as *const c_void, "glRenderbufferStorage")),
            RenderbufferStorageMultisample: FnPtr::new((0 as *const c_void, "glRenderbufferStorageMultisample")),
            ResumeTransformFeedback: FnPtr::new((0 as *const c_void, "glResumeTransformFeedback")),
            SampleCoverage: FnPtr::new((0 as *const c_void, "glSampleCoverage")),
            SampleMaski: FnPtr::new((0 as *const c_void, "glSampleMaski")),
            SamplerParameterIiv: FnPtr::new((0 as *const c_void, "glSamplerParameterIiv")),
            SamplerParameterIuiv: FnPtr::new((0 as *const c_void, "glSamplerParameterIuiv")),
            SamplerParameterf: FnPtr::new((0 as *const c_void, "glSamplerParameterf")),
            SamplerParameterfv: FnPtr::new((0 as *const c_void, "glSamplerParameterfv")),
            SamplerParameteri: FnPtr::new((0 as *const c_void, "glSamplerParameteri")),
            SamplerParameteriv: FnPtr::new((0 as *const c_void, "glSamplerParameteriv")),
            Scissor: FnPtr::new((0 as *const c_void, "glScissor")),
            ScissorArrayv: FnPtr::new((0 as *const c_void, "glScissorArrayv")),
            ScissorIndexed: FnPtr::new((0 as *const c_void, "glScissorIndexed")),
            ScissorIndexedv: FnPtr::new((0 as *const c_void, "glScissorIndexedv")),
            SecondaryColorP3ui: FnPtr::new((0 as *const c_void, "glSecondaryColorP3ui")),
            SecondaryColorP3uiv: FnPtr::new((0 as *const c_void, "glSecondaryColorP3uiv")),
            ShaderBinary: FnPtr::new((0 as *const c_void, "glShaderBinary")),
            ShaderSource: FnPtr::new((0 as *const c_void, "glShaderSource")),
            ShaderStorageBlockBinding: FnPtr::new((0 as *const c_void, "glShaderStorageBlockBinding")),
            StencilFunc: FnPtr::new((0 as *const c_void, "glStencilFunc")),
            StencilFuncSeparate: FnPtr::new((0 as *const c_void, "glStencilFuncSeparate")),
            StencilMask: FnPtr::new((0 as *const c_void, "glStencilMask")),
            StencilMaskSeparate: FnPtr::new((0 as *const c_void, "glStencilMaskSeparate")),
            StencilOp: FnPtr::new((0 as *const c_void, "glStencilOp")),
            StencilOpSeparate: FnPtr::new((0 as *const c_void, "glStencilOpSeparate")),
            TexBuffer: FnPtr::new((0 as *const c_void, "glTexBuffer")),
            TexBufferRange: FnPtr::new((0 as *const c_void, "glTexBufferRange")),
            TexCoordP1ui: FnPtr::new((0 as *const c_void, "glTexCoordP1ui")),
            TexCoordP1uiv: FnPtr::new((0 as *const c_void, "glTexCoordP1uiv")),
            TexCoordP2ui: FnPtr::new((0 as *const c_void, "glTexCoordP2ui")),
            TexCoordP2uiv: FnPtr::new((0 as *const c_void, "glTexCoordP2uiv")),
            TexCoordP3ui: FnPtr::new((0 as *const c_void, "glTexCoordP3ui")),
            TexCoordP3uiv: FnPtr::new((0 as *const c_void, "glTexCoordP3uiv")),
            TexCoordP4ui: FnPtr::new((0 as *const c_void, "glTexCoordP4ui")),
            TexCoordP4uiv: FnPtr::new((0 as *const c_void, "glTexCoordP4uiv")),
            TexImage1D: FnPtr::new((0 as *const c_void, "glTexImage1D")),
            TexImage2D: FnPtr::new((0 as *const c_void, "glTexImage2D")),
            TexImage2DMultisample: FnPtr::new((0 as *const c_void, "glTexImage2DMultisample")),
            TexImage3D: FnPtr::new((0 as *const c_void, "glTexImage3D")),
            TexImage3DMultisample: FnPtr::new((0 as *const c_void, "glTexImage3DMultisample")),
            TexParameterIiv: FnPtr::new((0 as *const c_void, "glTexParameterIiv")),
            TexParameterIuiv: FnPtr::new((0 as *const c_void, "glTexParameterIuiv")),
            TexParameterf: FnPtr::new((0 as *const c_void, "glTexParameterf")),
            TexParameterfv: FnPtr::new((0 as *const c_void, "glTexParameterfv")),
            TexParameteri: FnPtr::new((0 as *const c_void, "glTexParameteri")),
            TexParameteriv: FnPtr::new((0 as *const c_void, "glTexParameteriv")),
            TexStorage1D: FnPtr::new((0 as *const c_void, "glTexStorage1D")),
            TexStorage2D: FnPtr::new((0 as *const c_void, "glTexStorage2D")),
            TexStorage2DMultisample: FnPtr::new((0 as *const c_void, "glTexStorage2DMultisample")),
            TexStorage3D: FnPtr::new((0 as *const c_void, "glTexStorage3D")),
            TexStorage3DMultisample: FnPtr::new((0 as *const c_void, "glTexStorage3DMultisample")),
            TexSubImage1D: FnPtr::new((0 as *const c_void, "glTexSubImage1D")),
            TexSubImage2D: FnPtr::new((0 as *const c_void, "glTexSubImage2D")),
            TexSubImage3D: FnPtr::new((0 as *const c_void, "glTexSubImage3D")),
            TextureBarrier: FnPtr::new((0 as *const c_void, "glTextureBarrier")),
            TextureBuffer: FnPtr::new((0 as *const c_void, "glTextureBuffer")),
            TextureBufferRange: FnPtr::new((0 as *const c_void, "glTextureBufferRange")),
            TextureParameterIiv: FnPtr::new((0 as *const c_void, "glTextureParameterIiv")),
            TextureParameterIuiv: FnPtr::new((0 as *const c_void, "glTextureParameterIuiv")),
            TextureParameterf: FnPtr::new((0 as *const c_void, "glTextureParameterf")),
            TextureParameterfv: FnPtr::new((0 as *const c_void, "glTextureParameterfv")),
            TextureParameteri: FnPtr::new((0 as *const c_void, "glTextureParameteri")),
            TextureParameteriv: FnPtr::new((0 as *const c_void, "glTextureParameteriv")),
            TextureStorage1D: FnPtr::new((0 as *const c_void, "glTextureStorage1D")),
            TextureStorage2D: FnPtr::new((0 as *const c_void, "glTextureStorage2D")),
            TextureStorage2DMultisample: FnPtr::new((0 as *const c_void, "glTextureStorage2DMultisample")),
            TextureStorage3D: FnPtr::new((0 as *const c_void, "glTextureStorage3D")),
            TextureStorage3DMultisample: FnPtr::new((0 as *const c_void, "glTextureStorage3DMultisample")),
            TextureSubImage1D: FnPtr::new((0 as *const c_void, "glTextureSubImage1D")),
            TextureSubImage2D: FnPtr::new((0 as *const c_void, "glTextureSubImage2D")),
            TextureSubImage3D: FnPtr::new((0 as *const c_void, "glTextureSubImage3D")),
            TextureView: FnPtr::new((0 as *const c_void, "glTextureView")),
            TransformFeedbackBufferBase: FnPtr::new((0 as *const c_void, "glTransformFeedbackBufferBase")),
            TransformFeedbackBufferRange: FnPtr::new((0 as *const c_void, "glTransformFeedbackBufferRange")),
            TransformFeedbackVaryings: FnPtr::new((0 as *const c_void, "glTransformFeedbackVaryings")),
            Uniform1d: FnPtr::new((0 as *const c_void, "glUniform1d")),
            Uniform1dv: FnPtr::new((0 as *const c_void, "glUniform1dv")),
            Uniform1f: FnPtr::new((0 as *const c_void, "glUniform1f")),
            Uniform1fv: FnPtr::new((0 as *const c_void, "glUniform1fv")),
            Uniform1i: FnPtr::new((0 as *const c_void, "glUniform1i")),
            Uniform1iv: FnPtr::new((0 as *const c_void, "glUniform1iv")),
            Uniform1ui: FnPtr::new((0 as *const c_void, "glUniform1ui")),
            Uniform1uiv: FnPtr::new((0 as *const c_void, "glUniform1uiv")),
            Uniform2d: FnPtr::new((0 as *const c_void, "glUniform2d")),
            Uniform2dv: FnPtr::new((0 as *const c_void, "glUniform2dv")),
            Uniform2f: FnPtr::new((0 as *const c_void, "glUniform2f")),
            Uniform2fv: FnPtr::new((0 as *const c_void, "glUniform2fv")),
            Uniform2i: FnPtr::new((0 as *const c_void, "glUniform2i")),
            Uniform2iv: FnPtr::new((0 as *const c_void, "glUniform2iv")),
            Uniform2ui: FnPtr::new((0 as *const c_void, "glUniform2ui")),
            Uniform2uiv: FnPtr::new((0 as *const c_void, "glUniform2uiv")),
            Uniform3d: FnPtr::new((0 as *const c_void, "glUniform3d")),
            Uniform3dv: FnPtr::new((0 as *const c_void, "glUniform3dv")),
            Uniform3f: FnPtr::new((0 as *const c_void, "glUniform3f")),
            Uniform3fv: FnPtr::new((0 as *const c_void, "glUniform3fv")),
            Uniform3i: FnPtr::new((0 as *const c_void, "glUniform3i")),
            Uniform3iv: FnPtr::new((0 as *const c_void, "glUniform3iv")),
            Uniform3ui: FnPtr::new((0 as *const c_void, "glUniform3ui")),
            Uniform3uiv: FnPtr::new((0 as *const c_void, "glUniform3uiv")),
            Uniform4d: FnPtr::new((0 as *const c_void, "glUniform4d")),
            Uniform4dv: FnPtr::new((0 as *const c_void, "glUniform4dv")),
            Uniform4f: FnPtr::new((0 as *const c_void, "glUniform4f")),
            Uniform4fv: FnPtr::new((0 as *const c_void, "glUniform4fv")),
            Uniform4i: FnPtr::new((0 as *const c_void, "glUniform4i")),
            Uniform4iv: FnPtr::new((0 as *const c_void, "glUniform4iv")),
            Uniform4ui: FnPtr::new((0 as *const c_void, "glUniform4ui")),
            Uniform4uiv: FnPtr::new((0 as *const c_void, "glUniform4uiv")),
            UniformBlockBinding: FnPtr::new((0 as *const c_void, "glUniformBlockBinding")),
            UniformMatrix2dv: FnPtr::new((0 as *const c_void, "glUniformMatrix2dv")),
            UniformMatrix2fv: FnPtr::new((0 as *const c_void, "glUniformMatrix2fv")),
            UniformMatrix2x3dv: FnPtr::new((0 as *const c_void, "glUniformMatrix2x3dv")),
            UniformMatrix2x3fv: FnPtr::new((0 as *const c_void, "glUniformMatrix2x3fv")),
            UniformMatrix2x4dv: FnPtr::new((0 as *const c_void, "glUniformMatrix2x4dv")),
            UniformMatrix2x4fv: FnPtr::new((0 as *const c_void, "glUniformMatrix2x4fv")),
            UniformMatrix3dv: FnPtr::new((0 as *const c_void, "glUniformMatrix3dv")),
            UniformMatrix3fv: FnPtr::new((0 as *const c_void, "glUniformMatrix3fv")),
            UniformMatrix3x2dv: FnPtr::new((0 as *const c_void, "glUniformMatrix3x2dv")),
            UniformMatrix3x2fv: FnPtr::new((0 as *const c_void, "glUniformMatrix3x2fv")),
            UniformMatrix3x4dv: FnPtr::new((0 as *const c_void, "glUniformMatrix3x4dv")),
            UniformMatrix3x4fv: FnPtr::new((0 as *const c_void, "glUniformMatrix3x4fv")),
            UniformMatrix4dv: FnPtr::new((0 as *const c_void, "glUniformMatrix4dv")),
            UniformMatrix4fv: FnPtr::new((0 as *const c_void, "glUniformMatrix4fv")),
            UniformMatrix4x2dv: FnPtr::new((0 as *const c_void, "glUniformMatrix4x2dv")),
            UniformMatrix4x2fv: FnPtr::new((0 as *const c_void, "glUniformMatrix4x2fv")),
            UniformMatrix4x3dv: FnPtr::new((0 as *const c_void, "glUniformMatrix4x3dv")),
            UniformMatrix4x3fv: FnPtr::new((0 as *const c_void, "glUniformMatrix4x3fv")),
            UniformSubroutinesuiv: FnPtr::new((0 as *const c_void, "glUniformSubroutinesuiv")),
            UnmapBuffer: FnPtr::new((0 as *const c_void, "glUnmapBuffer")),
            UnmapNamedBuffer: FnPtr::new((0 as *const c_void, "glUnmapNamedBuffer")),
            UseProgram: FnPtr::new((0 as *const c_void, "glUseProgram")),
            UseProgramStages: FnPtr::new((0 as *const c_void, "glUseProgramStages")),
            ValidateProgram: FnPtr::new((0 as *const c_void, "glValidateProgram")),
            ValidateProgramPipeline: FnPtr::new((0 as *const c_void, "glValidateProgramPipeline")),
            VertexArrayAttribBinding: FnPtr::new((0 as *const c_void, "glVertexArrayAttribBinding")),
            VertexArrayAttribFormat: FnPtr::new((0 as *const c_void, "glVertexArrayAttribFormat")),
            VertexArrayAttribIFormat: FnPtr::new((0 as *const c_void, "glVertexArrayAttribIFormat")),
            VertexArrayAttribLFormat: FnPtr::new((0 as *const c_void, "glVertexArrayAttribLFormat")),
            VertexArrayBindingDivisor: FnPtr::new((0 as *const c_void, "glVertexArrayBindingDivisor")),
            VertexArrayElementBuffer: FnPtr::new((0 as *const c_void, "glVertexArrayElementBuffer")),
            VertexArrayVertexBuffer: FnPtr::new((0 as *const c_void, "glVertexArrayVertexBuffer")),
            VertexArrayVertexBuffers: FnPtr::new((0 as *const c_void, "glVertexArrayVertexBuffers")),
            VertexAttrib1d: FnPtr::new((0 as *const c_void, "glVertexAttrib1d")),
            VertexAttrib1dv: FnPtr::new((0 as *const c_void, "glVertexAttrib1dv")),
            VertexAttrib1f: FnPtr::new((0 as *const c_void, "glVertexAttrib1f")),
            VertexAttrib1fv: FnPtr::new((0 as *const c_void, "glVertexAttrib1fv")),
            VertexAttrib1s: FnPtr::new((0 as *const c_void, "glVertexAttrib1s")),
            VertexAttrib1sv: FnPtr::new((0 as *const c_void, "glVertexAttrib1sv")),
            VertexAttrib2d: FnPtr::new((0 as *const c_void, "glVertexAttrib2d")),
            VertexAttrib2dv: FnPtr::new((0 as *const c_void, "glVertexAttrib2dv")),
            VertexAttrib2f: FnPtr::new((0 as *const c_void, "glVertexAttrib2f")),
            VertexAttrib2fv: FnPtr::new((0 as *const c_void, "glVertexAttrib2fv")),
            VertexAttrib2s: FnPtr::new((0 as *const c_void, "glVertexAttrib2s")),
            VertexAttrib2sv: FnPtr::new((0 as *const c_void, "glVertexAttrib2sv")),
            VertexAttrib3d: FnPtr::new((0 as *const c_void, "glVertexAttrib3d")),
            VertexAttrib3dv: FnPtr::new((0 as *const c_void, "glVertexAttrib3dv")),
            VertexAttrib3f: FnPtr::new((0 as *const c_void, "glVertexAttrib3f")),
            VertexAttrib3fv: FnPtr::new((0 as *const c_void, "glVertexAttrib3fv")),
            VertexAttrib3s: FnPtr::new((0 as *const c_void, "glVertexAttrib3s")),
            VertexAttrib3sv: FnPtr::new((0 as *const c_void, "glVertexAttrib3sv")),
            VertexAttrib4Nbv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nbv")),
            VertexAttrib4Niv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Niv")),
            VertexAttrib4Nsv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nsv")),
            VertexAttrib4Nub: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nub")),
            VertexAttrib4Nubv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nubv")),
            VertexAttrib4Nuiv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nuiv")),
            VertexAttrib4Nusv: FnPtr::new((0 as *const c_void, "glVertexAttrib4Nusv")),
            VertexAttrib4bv: FnPtr::new((0 as *const c_void, "glVertexAttrib4bv")),
            VertexAttrib4d: FnPtr::new((0 as *const c_void, "glVertexAttrib4d")),
            VertexAttrib4dv: FnPtr::new((0 as *const c_void, "glVertexAttrib4dv")),
            VertexAttrib4f: FnPtr::new((0 as *const c_void, "glVertexAttrib4f")),
            VertexAttrib4fv: FnPtr::new((0 as *const c_void, "glVertexAttrib4fv")),
            VertexAttrib4iv: FnPtr::new((0 as *const c_void, "glVertexAttrib4iv")),
            VertexAttrib4s: FnPtr::new((0 as *const c_void, "glVertexAttrib4s")),
            VertexAttrib4sv: FnPtr::new((0 as *const c_void, "glVertexAttrib4sv")),
            VertexAttrib4ubv: FnPtr::new((0 as *const c_void, "glVertexAttrib4ubv")),
            VertexAttrib4uiv: FnPtr::new((0 as *const c_void, "glVertexAttrib4uiv")),
            VertexAttrib4usv: FnPtr::new((0 as *const c_void, "glVertexAttrib4usv")),
            VertexAttribBinding: FnPtr::new((0 as *const c_void, "glVertexAttribBinding")),
            VertexAttribDivisor: FnPtr::new((0 as *const c_void, "glVertexAttribDivisor")),
            VertexAttribFormat: FnPtr::new((0 as *const c_void, "glVertexAttribFormat")),
            VertexAttribI1i: FnPtr::new((0 as *const c_void, "glVertexAttribI1i")),
            VertexAttribI1iv: FnPtr::new((0 as *const c_void, "glVertexAttribI1iv")),
            VertexAttribI1ui: FnPtr::new((0 as *const c_void, "glVertexAttribI1ui")),
            VertexAttribI1uiv: FnPtr::new((0 as *const c_void, "glVertexAttribI1uiv")),
            VertexAttribI2i: FnPtr::new((0 as *const c_void, "glVertexAttribI2i")),
            VertexAttribI2iv: FnPtr::new((0 as *const c_void, "glVertexAttribI2iv")),
            VertexAttribI2ui: FnPtr::new((0 as *const c_void, "glVertexAttribI2ui")),
            VertexAttribI2uiv: FnPtr::new((0 as *const c_void, "glVertexAttribI2uiv")),
            VertexAttribI3i: FnPtr::new((0 as *const c_void, "glVertexAttribI3i")),
            VertexAttribI3iv: FnPtr::new((0 as *const c_void, "glVertexAttribI3iv")),
            VertexAttribI3ui: FnPtr::new((0 as *const c_void, "glVertexAttribI3ui")),
            VertexAttribI3uiv: FnPtr::new((0 as *const c_void, "glVertexAttribI3uiv")),
            VertexAttribI4bv: FnPtr::new((0 as *const c_void, "glVertexAttribI4bv")),
            VertexAttribI4i: FnPtr::new((0 as *const c_void, "glVertexAttribI4i")),
            VertexAttribI4iv: FnPtr::new((0 as *const c_void, "glVertexAttribI4iv")),
            VertexAttribI4sv: FnPtr::new((0 as *const c_void, "glVertexAttribI4sv")),
            VertexAttribI4ubv: FnPtr::new((0 as *const c_void, "glVertexAttribI4ubv")),
            VertexAttribI4ui: FnPtr::new((0 as *const c_void, "glVertexAttribI4ui")),
            VertexAttribI4uiv: FnPtr::new((0 as *const c_void, "glVertexAttribI4uiv")),
            VertexAttribI4usv: FnPtr::new((0 as *const c_void, "glVertexAttribI4usv")),
            VertexAttribIFormat: FnPtr::new((0 as *const c_void, "glVertexAttribIFormat")),
            VertexAttribIPointer: FnPtr::new((0 as *const c_void, "glVertexAttribIPointer")),
            VertexAttribL1d: FnPtr::new((0 as *const c_void, "glVertexAttribL1d")),
            VertexAttribL1dv: FnPtr::new((0 as *const c_void, "glVertexAttribL1dv")),
            VertexAttribL2d: FnPtr::new((0 as *const c_void, "glVertexAttribL2d")),
            VertexAttribL2dv: FnPtr::new((0 as *const c_void, "glVertexAttribL2dv")),
            VertexAttribL3d: FnPtr::new((0 as *const c_void, "glVertexAttribL3d")),
            VertexAttribL3dv: FnPtr::new((0 as *const c_void, "glVertexAttribL3dv")),
            VertexAttribL4d: FnPtr::new((0 as *const c_void, "glVertexAttribL4d")),
            VertexAttribL4dv: FnPtr::new((0 as *const c_void, "glVertexAttribL4dv")),
            VertexAttribLFormat: FnPtr::new((0 as *const c_void, "glVertexAttribLFormat")),
            VertexAttribLPointer: FnPtr::new((0 as *const c_void, "glVertexAttribLPointer")),
            VertexAttribP1ui: FnPtr::new((0 as *const c_void, "glVertexAttribP1ui")),
            VertexAttribP1uiv: FnPtr::new((0 as *const c_void, "glVertexAttribP1uiv")),
            VertexAttribP2ui: FnPtr::new((0 as *const c_void, "glVertexAttribP2ui")),
            VertexAttribP2uiv: FnPtr::new((0 as *const c_void, "glVertexAttribP2uiv")),
            VertexAttribP3ui: FnPtr::new((0 as *const c_void, "glVertexAttribP3ui")),
            VertexAttribP3uiv: FnPtr::new((0 as *const c_void, "glVertexAttribP3uiv")),
            VertexAttribP4ui: FnPtr::new((0 as *const c_void, "glVertexAttribP4ui")),
            VertexAttribP4uiv: FnPtr::new((0 as *const c_void, "glVertexAttribP4uiv")),
            VertexAttribPointer: FnPtr::new((0 as *const c_void, "glVertexAttribPointer")),
            VertexBindingDivisor: FnPtr::new((0 as *const c_void, "glVertexBindingDivisor")),
            VertexP2ui: FnPtr::new((0 as *const c_void, "glVertexP2ui")),
            VertexP2uiv: FnPtr::new((0 as *const c_void, "glVertexP2uiv")),
            VertexP3ui: FnPtr::new((0 as *const c_void, "glVertexP3ui")),
            VertexP3uiv: FnPtr::new((0 as *const c_void, "glVertexP3uiv")),
            VertexP4ui: FnPtr::new((0 as *const c_void, "glVertexP4ui")),
            VertexP4uiv: FnPtr::new((0 as *const c_void, "glVertexP4uiv")),
            Viewport: FnPtr::new((0 as *const c_void, "glViewport")),
            ViewportArrayv: FnPtr::new((0 as *const c_void, "glViewportArrayv")),
            ViewportIndexedf: FnPtr::new((0 as *const c_void, "glViewportIndexedf")),
            ViewportIndexedfv: FnPtr::new((0 as *const c_void, "glViewportIndexedfv")),
            WaitSync: FnPtr::new((0 as *const c_void, "glWaitSync")) 
        }
    }
}

pub static mut STORAGE: *mut PointerStorage = 0 as *mut PointerStorage;

pub fn missing_fn_panic() -> ! {
    panic!("gl function was not loaded")
}

pub fn metaloadfn(
    loadfn: &mut dyn FnMut(&'static str) -> *const c_void,
    symbol: &'static str,
    fallbacks: &[&'static str]
) -> (*const c_void, &'static str) {
    let mut ptr = loadfn(symbol);
    if ptr.is_null() {
        for &sym in fallbacks {
            ptr = loadfn(sym);
            if !ptr.is_null() { break; }
        }
    }
    // if ptr.is_null() { println!("failed to load {symbol}"); }
    // else { println!("loaded {symbol}"); }
    return (ptr, symbol);
}

pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void {
    #[inline(never)]
    fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const c_void) {
        ActiveShaderProgram::load_with(&mut *loadfn);
        ActiveTexture::load_with(&mut *loadfn);
        AttachShader::load_with(&mut *loadfn);
        BeginConditionalRender::load_with(&mut *loadfn);
        BeginQuery::load_with(&mut *loadfn);
        BeginQueryIndexed::load_with(&mut *loadfn);
        BeginTransformFeedback::load_with(&mut *loadfn);
        BindAttribLocation::load_with(&mut *loadfn);
        BindBuffer::load_with(&mut *loadfn);
        BindBufferBase::load_with(&mut *loadfn);
        BindBufferRange::load_with(&mut *loadfn);
        BindBuffersBase::load_with(&mut *loadfn);
        BindBuffersRange::load_with(&mut *loadfn);
        BindFragDataLocation::load_with(&mut *loadfn);
        BindFragDataLocationIndexed::load_with(&mut *loadfn);
        BindFramebuffer::load_with(&mut *loadfn);
        BindImageTexture::load_with(&mut *loadfn);
        BindImageTextures::load_with(&mut *loadfn);
        BindProgramPipeline::load_with(&mut *loadfn);
        BindRenderbuffer::load_with(&mut *loadfn);
        BindSampler::load_with(&mut *loadfn);
        BindSamplers::load_with(&mut *loadfn);
        BindTexture::load_with(&mut *loadfn);
        BindTextureUnit::load_with(&mut *loadfn);
        BindTextures::load_with(&mut *loadfn);
        BindTransformFeedback::load_with(&mut *loadfn);
        BindVertexArray::load_with(&mut *loadfn);
        BindVertexBuffer::load_with(&mut *loadfn);
        BindVertexBuffers::load_with(&mut *loadfn);
        BlendColor::load_with(&mut *loadfn);
        BlendEquation::load_with(&mut *loadfn);
        BlendEquationSeparate::load_with(&mut *loadfn);
        BlendEquationSeparatei::load_with(&mut *loadfn);
        BlendEquationi::load_with(&mut *loadfn);
        BlendFunc::load_with(&mut *loadfn);
        BlendFuncSeparate::load_with(&mut *loadfn);
        BlendFuncSeparatei::load_with(&mut *loadfn);
        BlendFunci::load_with(&mut *loadfn);
        BlitFramebuffer::load_with(&mut *loadfn);
        BlitNamedFramebuffer::load_with(&mut *loadfn);
        BufferData::load_with(&mut *loadfn);
        BufferStorage::load_with(&mut *loadfn);
        BufferSubData::load_with(&mut *loadfn);
        CheckFramebufferStatus::load_with(&mut *loadfn);
        CheckNamedFramebufferStatus::load_with(&mut *loadfn);
        ClampColor::load_with(&mut *loadfn);
        Clear::load_with(&mut *loadfn);
        ClearBufferData::load_with(&mut *loadfn);
        ClearBufferSubData::load_with(&mut *loadfn);
        ClearBufferfi::load_with(&mut *loadfn);
        ClearBufferfv::load_with(&mut *loadfn);
        ClearBufferiv::load_with(&mut *loadfn);
        ClearBufferuiv::load_with(&mut *loadfn);
        ClearColor::load_with(&mut *loadfn);
        ClearDepth::load_with(&mut *loadfn);
        ClearDepthf::load_with(&mut *loadfn);
        ClearNamedBufferData::load_with(&mut *loadfn);
        ClearNamedBufferSubData::load_with(&mut *loadfn);
        ClearNamedFramebufferfi::load_with(&mut *loadfn);
        ClearNamedFramebufferfv::load_with(&mut *loadfn);
        ClearNamedFramebufferiv::load_with(&mut *loadfn);
        ClearNamedFramebufferuiv::load_with(&mut *loadfn);
        ClearStencil::load_with(&mut *loadfn);
        ClearTexImage::load_with(&mut *loadfn);
        ClearTexSubImage::load_with(&mut *loadfn);
        ClientWaitSync::load_with(&mut *loadfn);
        ClipControl::load_with(&mut *loadfn);
        ColorMask::load_with(&mut *loadfn);
        ColorMaski::load_with(&mut *loadfn);
        ColorP3ui::load_with(&mut *loadfn);
        ColorP3uiv::load_with(&mut *loadfn);
        ColorP4ui::load_with(&mut *loadfn);
        ColorP4uiv::load_with(&mut *loadfn);
        CompileShader::load_with(&mut *loadfn);
        CompressedTexImage1D::load_with(&mut *loadfn);
        CompressedTexImage2D::load_with(&mut *loadfn);
        CompressedTexImage3D::load_with(&mut *loadfn);
        CompressedTexSubImage1D::load_with(&mut *loadfn);
        CompressedTexSubImage2D::load_with(&mut *loadfn);
        CompressedTexSubImage3D::load_with(&mut *loadfn);
        CompressedTextureSubImage1D::load_with(&mut *loadfn);
        CompressedTextureSubImage2D::load_with(&mut *loadfn);
        CompressedTextureSubImage3D::load_with(&mut *loadfn);
        CopyBufferSubData::load_with(&mut *loadfn);
        CopyImageSubData::load_with(&mut *loadfn);
        CopyNamedBufferSubData::load_with(&mut *loadfn);
        CopyTexImage1D::load_with(&mut *loadfn);
        CopyTexImage2D::load_with(&mut *loadfn);
        CopyTexSubImage1D::load_with(&mut *loadfn);
        CopyTexSubImage2D::load_with(&mut *loadfn);
        CopyTexSubImage3D::load_with(&mut *loadfn);
        CopyTextureSubImage1D::load_with(&mut *loadfn);
        CopyTextureSubImage2D::load_with(&mut *loadfn);
        CopyTextureSubImage3D::load_with(&mut *loadfn);
        CreateBuffers::load_with(&mut *loadfn);
        CreateFramebuffers::load_with(&mut *loadfn);
        CreateProgram::load_with(&mut *loadfn);
        CreateProgramPipelines::load_with(&mut *loadfn);
        CreateQueries::load_with(&mut *loadfn);
        CreateRenderbuffers::load_with(&mut *loadfn);
        CreateSamplers::load_with(&mut *loadfn);
        CreateShader::load_with(&mut *loadfn);
        CreateShaderProgramv::load_with(&mut *loadfn);
        CreateTextures::load_with(&mut *loadfn);
        CreateTransformFeedbacks::load_with(&mut *loadfn);
        CreateVertexArrays::load_with(&mut *loadfn);
        CullFace::load_with(&mut *loadfn);
        DebugMessageCallback::load_with(&mut *loadfn);
        DebugMessageControl::load_with(&mut *loadfn);
        DebugMessageInsert::load_with(&mut *loadfn);
        DeleteBuffers::load_with(&mut *loadfn);
        DeleteFramebuffers::load_with(&mut *loadfn);
        DeleteProgram::load_with(&mut *loadfn);
        DeleteProgramPipelines::load_with(&mut *loadfn);
        DeleteQueries::load_with(&mut *loadfn);
        DeleteRenderbuffers::load_with(&mut *loadfn);
        DeleteSamplers::load_with(&mut *loadfn);
        DeleteShader::load_with(&mut *loadfn);
        DeleteSync::load_with(&mut *loadfn);
        DeleteTextures::load_with(&mut *loadfn);
        DeleteTransformFeedbacks::load_with(&mut *loadfn);
        DeleteVertexArrays::load_with(&mut *loadfn);
        DepthFunc::load_with(&mut *loadfn);
        DepthMask::load_with(&mut *loadfn);
        DepthRange::load_with(&mut *loadfn);
        DepthRangeArrayv::load_with(&mut *loadfn);
        DepthRangeIndexed::load_with(&mut *loadfn);
        DepthRangef::load_with(&mut *loadfn);
        DetachShader::load_with(&mut *loadfn);
        Disable::load_with(&mut *loadfn);
        DisableVertexArrayAttrib::load_with(&mut *loadfn);
        DisableVertexAttribArray::load_with(&mut *loadfn);
        Disablei::load_with(&mut *loadfn);
        DispatchCompute::load_with(&mut *loadfn);
        DispatchComputeIndirect::load_with(&mut *loadfn);
        DrawArrays::load_with(&mut *loadfn);
        DrawArraysIndirect::load_with(&mut *loadfn);
        DrawArraysInstanced::load_with(&mut *loadfn);
        DrawArraysInstancedBaseInstance::load_with(&mut *loadfn);
        DrawBuffer::load_with(&mut *loadfn);
        DrawBuffers::load_with(&mut *loadfn);
        DrawElements::load_with(&mut *loadfn);
        DrawElementsBaseVertex::load_with(&mut *loadfn);
        DrawElementsIndirect::load_with(&mut *loadfn);
        DrawElementsInstanced::load_with(&mut *loadfn);
        DrawElementsInstancedBaseInstance::load_with(&mut *loadfn);
        DrawElementsInstancedBaseVertex::load_with(&mut *loadfn);
        DrawElementsInstancedBaseVertexBaseInstance::load_with(&mut *loadfn);
        DrawRangeElements::load_with(&mut *loadfn);
        DrawRangeElementsBaseVertex::load_with(&mut *loadfn);
        DrawTransformFeedback::load_with(&mut *loadfn);
        DrawTransformFeedbackInstanced::load_with(&mut *loadfn);
        DrawTransformFeedbackStream::load_with(&mut *loadfn);
        DrawTransformFeedbackStreamInstanced::load_with(&mut *loadfn);
        Enable::load_with(&mut *loadfn);
        EnableVertexArrayAttrib::load_with(&mut *loadfn);
        EnableVertexAttribArray::load_with(&mut *loadfn);
        Enablei::load_with(&mut *loadfn);
        EndConditionalRender::load_with(&mut *loadfn);
        EndQuery::load_with(&mut *loadfn);
        EndQueryIndexed::load_with(&mut *loadfn);
        EndTransformFeedback::load_with(&mut *loadfn);
        FenceSync::load_with(&mut *loadfn);
        Finish::load_with(&mut *loadfn);
        Flush::load_with(&mut *loadfn);
        FlushMappedBufferRange::load_with(&mut *loadfn);
        FlushMappedNamedBufferRange::load_with(&mut *loadfn);
        FramebufferParameteri::load_with(&mut *loadfn);
        FramebufferRenderbuffer::load_with(&mut *loadfn);
        FramebufferTexture::load_with(&mut *loadfn);
        FramebufferTexture1D::load_with(&mut *loadfn);
        FramebufferTexture2D::load_with(&mut *loadfn);
        FramebufferTexture3D::load_with(&mut *loadfn);
        FramebufferTextureLayer::load_with(&mut *loadfn);
        FrontFace::load_with(&mut *loadfn);
        GenBuffers::load_with(&mut *loadfn);
        GenFramebuffers::load_with(&mut *loadfn);
        GenProgramPipelines::load_with(&mut *loadfn);
        GenQueries::load_with(&mut *loadfn);
        GenRenderbuffers::load_with(&mut *loadfn);
        GenSamplers::load_with(&mut *loadfn);
        GenTextures::load_with(&mut *loadfn);
        GenTransformFeedbacks::load_with(&mut *loadfn);
        GenVertexArrays::load_with(&mut *loadfn);
        GenerateMipmap::load_with(&mut *loadfn);
        GenerateTextureMipmap::load_with(&mut *loadfn);
        GetActiveAtomicCounterBufferiv::load_with(&mut *loadfn);
        GetActiveAttrib::load_with(&mut *loadfn);
        GetActiveSubroutineName::load_with(&mut *loadfn);
        GetActiveSubroutineUniformName::load_with(&mut *loadfn);
        GetActiveSubroutineUniformiv::load_with(&mut *loadfn);
        GetActiveUniform::load_with(&mut *loadfn);
        GetActiveUniformBlockName::load_with(&mut *loadfn);
        GetActiveUniformBlockiv::load_with(&mut *loadfn);
        GetActiveUniformName::load_with(&mut *loadfn);
        GetActiveUniformsiv::load_with(&mut *loadfn);
        GetAttachedShaders::load_with(&mut *loadfn);
        GetAttribLocation::load_with(&mut *loadfn);
        GetBooleani_v::load_with(&mut *loadfn);
        GetBooleanv::load_with(&mut *loadfn);
        GetBufferParameteri64v::load_with(&mut *loadfn);
        GetBufferParameteriv::load_with(&mut *loadfn);
        GetBufferPointerv::load_with(&mut *loadfn);
        GetBufferSubData::load_with(&mut *loadfn);
        GetCompressedTexImage::load_with(&mut *loadfn);
        GetCompressedTextureImage::load_with(&mut *loadfn);
        GetCompressedTextureSubImage::load_with(&mut *loadfn);
        GetDebugMessageLog::load_with(&mut *loadfn);
        GetDoublei_v::load_with(&mut *loadfn);
        GetDoublev::load_with(&mut *loadfn);
        GetError::load_with(&mut *loadfn);
        GetFloati_v::load_with(&mut *loadfn);
        GetFloatv::load_with(&mut *loadfn);
        GetFragDataIndex::load_with(&mut *loadfn);
        GetFragDataLocation::load_with(&mut *loadfn);
        GetFramebufferAttachmentParameteriv::load_with(&mut *loadfn);
        GetFramebufferParameteriv::load_with(&mut *loadfn);
        GetGraphicsResetStatus::load_with(&mut *loadfn);
        GetInteger64i_v::load_with(&mut *loadfn);
        GetInteger64v::load_with(&mut *loadfn);
        GetIntegeri_v::load_with(&mut *loadfn);
        GetIntegerv::load_with(&mut *loadfn);
        GetInternalformati64v::load_with(&mut *loadfn);
        GetInternalformativ::load_with(&mut *loadfn);
        GetMultisamplefv::load_with(&mut *loadfn);
        GetNamedBufferParameteri64v::load_with(&mut *loadfn);
        GetNamedBufferParameteriv::load_with(&mut *loadfn);
        GetNamedBufferPointerv::load_with(&mut *loadfn);
        GetNamedBufferSubData::load_with(&mut *loadfn);
        GetNamedFramebufferAttachmentParameteriv::load_with(&mut *loadfn);
        GetNamedFramebufferParameteriv::load_with(&mut *loadfn);
        GetNamedRenderbufferParameteriv::load_with(&mut *loadfn);
        GetObjectLabel::load_with(&mut *loadfn);
        GetObjectPtrLabel::load_with(&mut *loadfn);
        GetPointerv::load_with(&mut *loadfn);
        GetProgramBinary::load_with(&mut *loadfn);
        GetProgramInfoLog::load_with(&mut *loadfn);
        GetProgramInterfaceiv::load_with(&mut *loadfn);
        GetProgramPipelineInfoLog::load_with(&mut *loadfn);
        GetProgramPipelineiv::load_with(&mut *loadfn);
        GetProgramResourceIndex::load_with(&mut *loadfn);
        GetProgramResourceLocation::load_with(&mut *loadfn);
        GetProgramResourceLocationIndex::load_with(&mut *loadfn);
        GetProgramResourceName::load_with(&mut *loadfn);
        GetProgramResourceiv::load_with(&mut *loadfn);
        GetProgramStageiv::load_with(&mut *loadfn);
        GetProgramiv::load_with(&mut *loadfn);
        GetQueryBufferObjecti64v::load_with(&mut *loadfn);
        GetQueryBufferObjectiv::load_with(&mut *loadfn);
        GetQueryBufferObjectui64v::load_with(&mut *loadfn);
        GetQueryBufferObjectuiv::load_with(&mut *loadfn);
        GetQueryIndexediv::load_with(&mut *loadfn);
        GetQueryObjecti64v::load_with(&mut *loadfn);
        GetQueryObjectiv::load_with(&mut *loadfn);
        GetQueryObjectui64v::load_with(&mut *loadfn);
        GetQueryObjectuiv::load_with(&mut *loadfn);
        GetQueryiv::load_with(&mut *loadfn);
        GetRenderbufferParameteriv::load_with(&mut *loadfn);
        GetSamplerParameterIiv::load_with(&mut *loadfn);
        GetSamplerParameterIuiv::load_with(&mut *loadfn);
        GetSamplerParameterfv::load_with(&mut *loadfn);
        GetSamplerParameteriv::load_with(&mut *loadfn);
        GetShaderInfoLog::load_with(&mut *loadfn);
        GetShaderPrecisionFormat::load_with(&mut *loadfn);
        GetShaderSource::load_with(&mut *loadfn);
        GetShaderiv::load_with(&mut *loadfn);
        GetString::load_with(&mut *loadfn);
        GetStringi::load_with(&mut *loadfn);
        GetSubroutineIndex::load_with(&mut *loadfn);
        GetSubroutineUniformLocation::load_with(&mut *loadfn);
        GetSynciv::load_with(&mut *loadfn);
        GetTexImage::load_with(&mut *loadfn);
        GetTexLevelParameterfv::load_with(&mut *loadfn);
        GetTexLevelParameteriv::load_with(&mut *loadfn);
        GetTexParameterIiv::load_with(&mut *loadfn);
        GetTexParameterIuiv::load_with(&mut *loadfn);
        GetTexParameterfv::load_with(&mut *loadfn);
        GetTexParameteriv::load_with(&mut *loadfn);
        GetTextureImage::load_with(&mut *loadfn);
        GetTextureLevelParameterfv::load_with(&mut *loadfn);
        GetTextureLevelParameteriv::load_with(&mut *loadfn);
        GetTextureParameterIiv::load_with(&mut *loadfn);
        GetTextureParameterIuiv::load_with(&mut *loadfn);
        GetTextureParameterfv::load_with(&mut *loadfn);
        GetTextureParameteriv::load_with(&mut *loadfn);
        GetTextureSubImage::load_with(&mut *loadfn);
        GetTransformFeedbackVarying::load_with(&mut *loadfn);
        GetTransformFeedbacki64_v::load_with(&mut *loadfn);
        GetTransformFeedbacki_v::load_with(&mut *loadfn);
        GetTransformFeedbackiv::load_with(&mut *loadfn);
        GetUniformBlockIndex::load_with(&mut *loadfn);
        GetUniformIndices::load_with(&mut *loadfn);
        GetUniformLocation::load_with(&mut *loadfn);
        GetUniformSubroutineuiv::load_with(&mut *loadfn);
        GetUniformdv::load_with(&mut *loadfn);
        GetUniformfv::load_with(&mut *loadfn);
        GetUniformiv::load_with(&mut *loadfn);
        GetUniformuiv::load_with(&mut *loadfn);
        GetVertexArrayIndexed64iv::load_with(&mut *loadfn);
        GetVertexArrayIndexediv::load_with(&mut *loadfn);
        GetVertexArrayiv::load_with(&mut *loadfn);
        GetVertexAttribIiv::load_with(&mut *loadfn);
        GetVertexAttribIuiv::load_with(&mut *loadfn);
        GetVertexAttribLdv::load_with(&mut *loadfn);
        GetVertexAttribPointerv::load_with(&mut *loadfn);
        GetVertexAttribdv::load_with(&mut *loadfn);
        GetVertexAttribfv::load_with(&mut *loadfn);
        GetVertexAttribiv::load_with(&mut *loadfn);
        GetnColorTable::load_with(&mut *loadfn);
        GetnCompressedTexImage::load_with(&mut *loadfn);
        GetnConvolutionFilter::load_with(&mut *loadfn);
        GetnHistogram::load_with(&mut *loadfn);
        GetnMapdv::load_with(&mut *loadfn);
        GetnMapfv::load_with(&mut *loadfn);
        GetnMapiv::load_with(&mut *loadfn);
        GetnMinmax::load_with(&mut *loadfn);
        GetnPixelMapfv::load_with(&mut *loadfn);
        GetnPixelMapuiv::load_with(&mut *loadfn);
        GetnPixelMapusv::load_with(&mut *loadfn);
        GetnPolygonStipple::load_with(&mut *loadfn);
        GetnSeparableFilter::load_with(&mut *loadfn);
        GetnTexImage::load_with(&mut *loadfn);
        GetnUniformdv::load_with(&mut *loadfn);
        GetnUniformfv::load_with(&mut *loadfn);
        GetnUniformiv::load_with(&mut *loadfn);
        GetnUniformuiv::load_with(&mut *loadfn);
        Hint::load_with(&mut *loadfn);
        InvalidateBufferData::load_with(&mut *loadfn);
        InvalidateBufferSubData::load_with(&mut *loadfn);
        InvalidateFramebuffer::load_with(&mut *loadfn);
        InvalidateNamedFramebufferData::load_with(&mut *loadfn);
        InvalidateNamedFramebufferSubData::load_with(&mut *loadfn);
        InvalidateSubFramebuffer::load_with(&mut *loadfn);
        InvalidateTexImage::load_with(&mut *loadfn);
        InvalidateTexSubImage::load_with(&mut *loadfn);
        IsBuffer::load_with(&mut *loadfn);
        IsEnabled::load_with(&mut *loadfn);
        IsEnabledi::load_with(&mut *loadfn);
        IsFramebuffer::load_with(&mut *loadfn);
        IsProgram::load_with(&mut *loadfn);
        IsProgramPipeline::load_with(&mut *loadfn);
        IsQuery::load_with(&mut *loadfn);
        IsRenderbuffer::load_with(&mut *loadfn);
        IsSampler::load_with(&mut *loadfn);
        IsShader::load_with(&mut *loadfn);
        IsSync::load_with(&mut *loadfn);
        IsTexture::load_with(&mut *loadfn);
        IsTransformFeedback::load_with(&mut *loadfn);
        IsVertexArray::load_with(&mut *loadfn);
        LineWidth::load_with(&mut *loadfn);
        LinkProgram::load_with(&mut *loadfn);
        LogicOp::load_with(&mut *loadfn);
        MapBuffer::load_with(&mut *loadfn);
        MapBufferRange::load_with(&mut *loadfn);
        MapNamedBuffer::load_with(&mut *loadfn);
        MapNamedBufferRange::load_with(&mut *loadfn);
        MemoryBarrier::load_with(&mut *loadfn);
        MemoryBarrierByRegion::load_with(&mut *loadfn);
        MinSampleShading::load_with(&mut *loadfn);
        MultiDrawArrays::load_with(&mut *loadfn);
        MultiDrawArraysIndirect::load_with(&mut *loadfn);
        MultiDrawElements::load_with(&mut *loadfn);
        MultiDrawElementsBaseVertex::load_with(&mut *loadfn);
        MultiDrawElementsIndirect::load_with(&mut *loadfn);
        MultiTexCoordP1ui::load_with(&mut *loadfn);
        MultiTexCoordP1uiv::load_with(&mut *loadfn);
        MultiTexCoordP2ui::load_with(&mut *loadfn);
        MultiTexCoordP2uiv::load_with(&mut *loadfn);
        MultiTexCoordP3ui::load_with(&mut *loadfn);
        MultiTexCoordP3uiv::load_with(&mut *loadfn);
        MultiTexCoordP4ui::load_with(&mut *loadfn);
        MultiTexCoordP4uiv::load_with(&mut *loadfn);
        NamedBufferData::load_with(&mut *loadfn);
        NamedBufferStorage::load_with(&mut *loadfn);
        NamedBufferSubData::load_with(&mut *loadfn);
        NamedFramebufferDrawBuffer::load_with(&mut *loadfn);
        NamedFramebufferDrawBuffers::load_with(&mut *loadfn);
        NamedFramebufferParameteri::load_with(&mut *loadfn);
        NamedFramebufferReadBuffer::load_with(&mut *loadfn);
        NamedFramebufferRenderbuffer::load_with(&mut *loadfn);
        NamedFramebufferTexture::load_with(&mut *loadfn);
        NamedFramebufferTextureLayer::load_with(&mut *loadfn);
        NamedRenderbufferStorage::load_with(&mut *loadfn);
        NamedRenderbufferStorageMultisample::load_with(&mut *loadfn);
        NormalP3ui::load_with(&mut *loadfn);
        NormalP3uiv::load_with(&mut *loadfn);
        ObjectLabel::load_with(&mut *loadfn);
        ObjectPtrLabel::load_with(&mut *loadfn);
        PatchParameterfv::load_with(&mut *loadfn);
        PatchParameteri::load_with(&mut *loadfn);
        PauseTransformFeedback::load_with(&mut *loadfn);
        PixelStoref::load_with(&mut *loadfn);
        PixelStorei::load_with(&mut *loadfn);
        PointParameterf::load_with(&mut *loadfn);
        PointParameterfv::load_with(&mut *loadfn);
        PointParameteri::load_with(&mut *loadfn);
        PointParameteriv::load_with(&mut *loadfn);
        PointSize::load_with(&mut *loadfn);
        PolygonMode::load_with(&mut *loadfn);
        PolygonOffset::load_with(&mut *loadfn);
        PopDebugGroup::load_with(&mut *loadfn);
        PrimitiveRestartIndex::load_with(&mut *loadfn);
        ProgramBinary::load_with(&mut *loadfn);
        ProgramParameteri::load_with(&mut *loadfn);
        ProgramUniform1d::load_with(&mut *loadfn);
        ProgramUniform1dv::load_with(&mut *loadfn);
        ProgramUniform1f::load_with(&mut *loadfn);
        ProgramUniform1fv::load_with(&mut *loadfn);
        ProgramUniform1i::load_with(&mut *loadfn);
        ProgramUniform1iv::load_with(&mut *loadfn);
        ProgramUniform1ui::load_with(&mut *loadfn);
        ProgramUniform1uiv::load_with(&mut *loadfn);
        ProgramUniform2d::load_with(&mut *loadfn);
        ProgramUniform2dv::load_with(&mut *loadfn);
        ProgramUniform2f::load_with(&mut *loadfn);
        ProgramUniform2fv::load_with(&mut *loadfn);
        ProgramUniform2i::load_with(&mut *loadfn);
        ProgramUniform2iv::load_with(&mut *loadfn);
        ProgramUniform2ui::load_with(&mut *loadfn);
        ProgramUniform2uiv::load_with(&mut *loadfn);
        ProgramUniform3d::load_with(&mut *loadfn);
        ProgramUniform3dv::load_with(&mut *loadfn);
        ProgramUniform3f::load_with(&mut *loadfn);
        ProgramUniform3fv::load_with(&mut *loadfn);
        ProgramUniform3i::load_with(&mut *loadfn);
        ProgramUniform3iv::load_with(&mut *loadfn);
        ProgramUniform3ui::load_with(&mut *loadfn);
        ProgramUniform3uiv::load_with(&mut *loadfn);
        ProgramUniform4d::load_with(&mut *loadfn);
        ProgramUniform4dv::load_with(&mut *loadfn);
        ProgramUniform4f::load_with(&mut *loadfn);
        ProgramUniform4fv::load_with(&mut *loadfn);
        ProgramUniform4i::load_with(&mut *loadfn);
        ProgramUniform4iv::load_with(&mut *loadfn);
        ProgramUniform4ui::load_with(&mut *loadfn);
        ProgramUniform4uiv::load_with(&mut *loadfn);
        ProgramUniformMatrix2dv::load_with(&mut *loadfn);
        ProgramUniformMatrix2fv::load_with(&mut *loadfn);
        ProgramUniformMatrix2x3dv::load_with(&mut *loadfn);
        ProgramUniformMatrix2x3fv::load_with(&mut *loadfn);
        ProgramUniformMatrix2x4dv::load_with(&mut *loadfn);
        ProgramUniformMatrix2x4fv::load_with(&mut *loadfn);
        ProgramUniformMatrix3dv::load_with(&mut *loadfn);
        ProgramUniformMatrix3fv::load_with(&mut *loadfn);
        ProgramUniformMatrix3x2dv::load_with(&mut *loadfn);
        ProgramUniformMatrix3x2fv::load_with(&mut *loadfn);
        ProgramUniformMatrix3x4dv::load_with(&mut *loadfn);
        ProgramUniformMatrix3x4fv::load_with(&mut *loadfn);
        ProgramUniformMatrix4dv::load_with(&mut *loadfn);
        ProgramUniformMatrix4fv::load_with(&mut *loadfn);
        ProgramUniformMatrix4x2dv::load_with(&mut *loadfn);
        ProgramUniformMatrix4x2fv::load_with(&mut *loadfn);
        ProgramUniformMatrix4x3dv::load_with(&mut *loadfn);
        ProgramUniformMatrix4x3fv::load_with(&mut *loadfn);
        ProvokingVertex::load_with(&mut *loadfn);
        PushDebugGroup::load_with(&mut *loadfn);
        QueryCounter::load_with(&mut *loadfn);
        ReadBuffer::load_with(&mut *loadfn);
        ReadPixels::load_with(&mut *loadfn);
        ReadnPixels::load_with(&mut *loadfn);
        ReleaseShaderCompiler::load_with(&mut *loadfn);
        RenderbufferStorage::load_with(&mut *loadfn);
        RenderbufferStorageMultisample::load_with(&mut *loadfn);
        ResumeTransformFeedback::load_with(&mut *loadfn);
        SampleCoverage::load_with(&mut *loadfn);
        SampleMaski::load_with(&mut *loadfn);
        SamplerParameterIiv::load_with(&mut *loadfn);
        SamplerParameterIuiv::load_with(&mut *loadfn);
        SamplerParameterf::load_with(&mut *loadfn);
        SamplerParameterfv::load_with(&mut *loadfn);
        SamplerParameteri::load_with(&mut *loadfn);
        SamplerParameteriv::load_with(&mut *loadfn);
        Scissor::load_with(&mut *loadfn);
        ScissorArrayv::load_with(&mut *loadfn);
        ScissorIndexed::load_with(&mut *loadfn);
        ScissorIndexedv::load_with(&mut *loadfn);
        SecondaryColorP3ui::load_with(&mut *loadfn);
        SecondaryColorP3uiv::load_with(&mut *loadfn);
        ShaderBinary::load_with(&mut *loadfn);
        ShaderSource::load_with(&mut *loadfn);
        ShaderStorageBlockBinding::load_with(&mut *loadfn);
        StencilFunc::load_with(&mut *loadfn);
        StencilFuncSeparate::load_with(&mut *loadfn);
        StencilMask::load_with(&mut *loadfn);
        StencilMaskSeparate::load_with(&mut *loadfn);
        StencilOp::load_with(&mut *loadfn);
        StencilOpSeparate::load_with(&mut *loadfn);
        TexBuffer::load_with(&mut *loadfn);
        TexBufferRange::load_with(&mut *loadfn);
        TexCoordP1ui::load_with(&mut *loadfn);
        TexCoordP1uiv::load_with(&mut *loadfn);
        TexCoordP2ui::load_with(&mut *loadfn);
        TexCoordP2uiv::load_with(&mut *loadfn);
        TexCoordP3ui::load_with(&mut *loadfn);
        TexCoordP3uiv::load_with(&mut *loadfn);
        TexCoordP4ui::load_with(&mut *loadfn);
        TexCoordP4uiv::load_with(&mut *loadfn);
        TexImage1D::load_with(&mut *loadfn);
        TexImage2D::load_with(&mut *loadfn);
        TexImage2DMultisample::load_with(&mut *loadfn);
        TexImage3D::load_with(&mut *loadfn);
        TexImage3DMultisample::load_with(&mut *loadfn);
        TexParameterIiv::load_with(&mut *loadfn);
        TexParameterIuiv::load_with(&mut *loadfn);
        TexParameterf::load_with(&mut *loadfn);
        TexParameterfv::load_with(&mut *loadfn);
        TexParameteri::load_with(&mut *loadfn);
        TexParameteriv::load_with(&mut *loadfn);
        TexStorage1D::load_with(&mut *loadfn);
        TexStorage2D::load_with(&mut *loadfn);
        TexStorage2DMultisample::load_with(&mut *loadfn);
        TexStorage3D::load_with(&mut *loadfn);
        TexStorage3DMultisample::load_with(&mut *loadfn);
        TexSubImage1D::load_with(&mut *loadfn);
        TexSubImage2D::load_with(&mut *loadfn);
        TexSubImage3D::load_with(&mut *loadfn);
        TextureBarrier::load_with(&mut *loadfn);
        TextureBuffer::load_with(&mut *loadfn);
        TextureBufferRange::load_with(&mut *loadfn);
        TextureParameterIiv::load_with(&mut *loadfn);
        TextureParameterIuiv::load_with(&mut *loadfn);
        TextureParameterf::load_with(&mut *loadfn);
        TextureParameterfv::load_with(&mut *loadfn);
        TextureParameteri::load_with(&mut *loadfn);
        TextureParameteriv::load_with(&mut *loadfn);
        TextureStorage1D::load_with(&mut *loadfn);
        TextureStorage2D::load_with(&mut *loadfn);
        TextureStorage2DMultisample::load_with(&mut *loadfn);
        TextureStorage3D::load_with(&mut *loadfn);
        TextureStorage3DMultisample::load_with(&mut *loadfn);
        TextureSubImage1D::load_with(&mut *loadfn);
        TextureSubImage2D::load_with(&mut *loadfn);
        TextureSubImage3D::load_with(&mut *loadfn);
        TextureView::load_with(&mut *loadfn);
        TransformFeedbackBufferBase::load_with(&mut *loadfn);
        TransformFeedbackBufferRange::load_with(&mut *loadfn);
        TransformFeedbackVaryings::load_with(&mut *loadfn);
        Uniform1d::load_with(&mut *loadfn);
        Uniform1dv::load_with(&mut *loadfn);
        Uniform1f::load_with(&mut *loadfn);
        Uniform1fv::load_with(&mut *loadfn);
        Uniform1i::load_with(&mut *loadfn);
        Uniform1iv::load_with(&mut *loadfn);
        Uniform1ui::load_with(&mut *loadfn);
        Uniform1uiv::load_with(&mut *loadfn);
        Uniform2d::load_with(&mut *loadfn);
        Uniform2dv::load_with(&mut *loadfn);
        Uniform2f::load_with(&mut *loadfn);
        Uniform2fv::load_with(&mut *loadfn);
        Uniform2i::load_with(&mut *loadfn);
        Uniform2iv::load_with(&mut *loadfn);
        Uniform2ui::load_with(&mut *loadfn);
        Uniform2uiv::load_with(&mut *loadfn);
        Uniform3d::load_with(&mut *loadfn);
        Uniform3dv::load_with(&mut *loadfn);
        Uniform3f::load_with(&mut *loadfn);
        Uniform3fv::load_with(&mut *loadfn);
        Uniform3i::load_with(&mut *loadfn);
        Uniform3iv::load_with(&mut *loadfn);
        Uniform3ui::load_with(&mut *loadfn);
        Uniform3uiv::load_with(&mut *loadfn);
        Uniform4d::load_with(&mut *loadfn);
        Uniform4dv::load_with(&mut *loadfn);
        Uniform4f::load_with(&mut *loadfn);
        Uniform4fv::load_with(&mut *loadfn);
        Uniform4i::load_with(&mut *loadfn);
        Uniform4iv::load_with(&mut *loadfn);
        Uniform4ui::load_with(&mut *loadfn);
        Uniform4uiv::load_with(&mut *loadfn);
        UniformBlockBinding::load_with(&mut *loadfn);
        UniformMatrix2dv::load_with(&mut *loadfn);
        UniformMatrix2fv::load_with(&mut *loadfn);
        UniformMatrix2x3dv::load_with(&mut *loadfn);
        UniformMatrix2x3fv::load_with(&mut *loadfn);
        UniformMatrix2x4dv::load_with(&mut *loadfn);
        UniformMatrix2x4fv::load_with(&mut *loadfn);
        UniformMatrix3dv::load_with(&mut *loadfn);
        UniformMatrix3fv::load_with(&mut *loadfn);
        UniformMatrix3x2dv::load_with(&mut *loadfn);
        UniformMatrix3x2fv::load_with(&mut *loadfn);
        UniformMatrix3x4dv::load_with(&mut *loadfn);
        UniformMatrix3x4fv::load_with(&mut *loadfn);
        UniformMatrix4dv::load_with(&mut *loadfn);
        UniformMatrix4fv::load_with(&mut *loadfn);
        UniformMatrix4x2dv::load_with(&mut *loadfn);
        UniformMatrix4x2fv::load_with(&mut *loadfn);
        UniformMatrix4x3dv::load_with(&mut *loadfn);
        UniformMatrix4x3fv::load_with(&mut *loadfn);
        UniformSubroutinesuiv::load_with(&mut *loadfn);
        UnmapBuffer::load_with(&mut *loadfn);
        UnmapNamedBuffer::load_with(&mut *loadfn);
        UseProgram::load_with(&mut *loadfn);
        UseProgramStages::load_with(&mut *loadfn);
        ValidateProgram::load_with(&mut *loadfn);
        ValidateProgramPipeline::load_with(&mut *loadfn);
        VertexArrayAttribBinding::load_with(&mut *loadfn);
        VertexArrayAttribFormat::load_with(&mut *loadfn);
        VertexArrayAttribIFormat::load_with(&mut *loadfn);
        VertexArrayAttribLFormat::load_with(&mut *loadfn);
        VertexArrayBindingDivisor::load_with(&mut *loadfn);
        VertexArrayElementBuffer::load_with(&mut *loadfn);
        VertexArrayVertexBuffer::load_with(&mut *loadfn);
        VertexArrayVertexBuffers::load_with(&mut *loadfn);
        VertexAttrib1d::load_with(&mut *loadfn);
        VertexAttrib1dv::load_with(&mut *loadfn);
        VertexAttrib1f::load_with(&mut *loadfn);
        VertexAttrib1fv::load_with(&mut *loadfn);
        VertexAttrib1s::load_with(&mut *loadfn);
        VertexAttrib1sv::load_with(&mut *loadfn);
        VertexAttrib2d::load_with(&mut *loadfn);
        VertexAttrib2dv::load_with(&mut *loadfn);
        VertexAttrib2f::load_with(&mut *loadfn);
        VertexAttrib2fv::load_with(&mut *loadfn);
        VertexAttrib2s::load_with(&mut *loadfn);
        VertexAttrib2sv::load_with(&mut *loadfn);
        VertexAttrib3d::load_with(&mut *loadfn);
        VertexAttrib3dv::load_with(&mut *loadfn);
        VertexAttrib3f::load_with(&mut *loadfn);
        VertexAttrib3fv::load_with(&mut *loadfn);
        VertexAttrib3s::load_with(&mut *loadfn);
        VertexAttrib3sv::load_with(&mut *loadfn);
        VertexAttrib4Nbv::load_with(&mut *loadfn);
        VertexAttrib4Niv::load_with(&mut *loadfn);
        VertexAttrib4Nsv::load_with(&mut *loadfn);
        VertexAttrib4Nub::load_with(&mut *loadfn);
        VertexAttrib4Nubv::load_with(&mut *loadfn);
        VertexAttrib4Nuiv::load_with(&mut *loadfn);
        VertexAttrib4Nusv::load_with(&mut *loadfn);
        VertexAttrib4bv::load_with(&mut *loadfn);
        VertexAttrib4d::load_with(&mut *loadfn);
        VertexAttrib4dv::load_with(&mut *loadfn);
        VertexAttrib4f::load_with(&mut *loadfn);
        VertexAttrib4fv::load_with(&mut *loadfn);
        VertexAttrib4iv::load_with(&mut *loadfn);
        VertexAttrib4s::load_with(&mut *loadfn);
        VertexAttrib4sv::load_with(&mut *loadfn);
        VertexAttrib4ubv::load_with(&mut *loadfn);
        VertexAttrib4uiv::load_with(&mut *loadfn);
        VertexAttrib4usv::load_with(&mut *loadfn);
        VertexAttribBinding::load_with(&mut *loadfn);
        VertexAttribDivisor::load_with(&mut *loadfn);
        VertexAttribFormat::load_with(&mut *loadfn);
        VertexAttribI1i::load_with(&mut *loadfn);
        VertexAttribI1iv::load_with(&mut *loadfn);
        VertexAttribI1ui::load_with(&mut *loadfn);
        VertexAttribI1uiv::load_with(&mut *loadfn);
        VertexAttribI2i::load_with(&mut *loadfn);
        VertexAttribI2iv::load_with(&mut *loadfn);
        VertexAttribI2ui::load_with(&mut *loadfn);
        VertexAttribI2uiv::load_with(&mut *loadfn);
        VertexAttribI3i::load_with(&mut *loadfn);
        VertexAttribI3iv::load_with(&mut *loadfn);
        VertexAttribI3ui::load_with(&mut *loadfn);
        VertexAttribI3uiv::load_with(&mut *loadfn);
        VertexAttribI4bv::load_with(&mut *loadfn);
        VertexAttribI4i::load_with(&mut *loadfn);
        VertexAttribI4iv::load_with(&mut *loadfn);
        VertexAttribI4sv::load_with(&mut *loadfn);
        VertexAttribI4ubv::load_with(&mut *loadfn);
        VertexAttribI4ui::load_with(&mut *loadfn);
        VertexAttribI4uiv::load_with(&mut *loadfn);
        VertexAttribI4usv::load_with(&mut *loadfn);
        VertexAttribIFormat::load_with(&mut *loadfn);
        VertexAttribIPointer::load_with(&mut *loadfn);
        VertexAttribL1d::load_with(&mut *loadfn);
        VertexAttribL1dv::load_with(&mut *loadfn);
        VertexAttribL2d::load_with(&mut *loadfn);
        VertexAttribL2dv::load_with(&mut *loadfn);
        VertexAttribL3d::load_with(&mut *loadfn);
        VertexAttribL3dv::load_with(&mut *loadfn);
        VertexAttribL4d::load_with(&mut *loadfn);
        VertexAttribL4dv::load_with(&mut *loadfn);
        VertexAttribLFormat::load_with(&mut *loadfn);
        VertexAttribLPointer::load_with(&mut *loadfn);
        VertexAttribP1ui::load_with(&mut *loadfn);
        VertexAttribP1uiv::load_with(&mut *loadfn);
        VertexAttribP2ui::load_with(&mut *loadfn);
        VertexAttribP2uiv::load_with(&mut *loadfn);
        VertexAttribP3ui::load_with(&mut *loadfn);
        VertexAttribP3uiv::load_with(&mut *loadfn);
        VertexAttribP4ui::load_with(&mut *loadfn);
        VertexAttribP4uiv::load_with(&mut *loadfn);
        VertexAttribPointer::load_with(&mut *loadfn);
        VertexBindingDivisor::load_with(&mut *loadfn);
        VertexP2ui::load_with(&mut *loadfn);
        VertexP2uiv::load_with(&mut *loadfn);
        VertexP3ui::load_with(&mut *loadfn);
        VertexP3uiv::load_with(&mut *loadfn);
        VertexP4ui::load_with(&mut *loadfn);
        VertexP4uiv::load_with(&mut *loadfn);
        Viewport::load_with(&mut *loadfn);
        ViewportArrayv::load_with(&mut *loadfn);
        ViewportIndexedf::load_with(&mut *loadfn);
        ViewportIndexedfv::load_with(&mut *loadfn);
        WaitSync::load_with(&mut *loadfn);
    }

    if EXTERNAL_POINTERS {
        inner(&mut loadfn)
    } else {
        gl::load_with(loadfn);
    }
}

#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_PROGRAM: GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_RESOURCES: GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_VARIABLES: GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALREADY_SIGNALED: GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AND: GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_SIZE: GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_STRIDE: GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BGR: GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA: GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA_INTEGER: GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)] pub const BGR_INTEGER: GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLOCK_INDEX: GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_INTEGER: GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER: GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS: GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_BINDING: GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_DATA_SIZE: GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED: GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_VARIABLE: GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CAVEAT_SUPPORT: GLenum = 0x82B8;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_READ_COLOR: GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_BUFFER: GLenum = 0x82B4;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_TEXTURE: GLenum = 0x9365;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_STORAGE_BIT: GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DEPTH_MODE: GLenum = 0x935D;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE0: GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE1: GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE2: GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE3: GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE4: GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE5: GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE6: GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE7: GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_ORIGIN: GLenum = 0x935C;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_COMPONENTS: GLenum = 0x8283;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ENCODING: GLenum = 0x8296;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_RENDERABLE: GLenum = 0x8286;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMMAND_BARRIER_BIT: GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_R11_EAC: GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED: GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG: GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG11_EAC: GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB: GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA: GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB: GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER: GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER_BIT: GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE: GLenum = 0x92ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_TEXTURE: GLenum = 0x82A0;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS: GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_LOST: GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER: GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_QUERY: GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT: GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API: GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR: GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER: GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER: GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLAMP: GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENTS: GLenum = 0x8284;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RENDERABLE: GLenum = 0x8287;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL: GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)] pub const DISPLAY_LIST: GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2: GLenum = 0x8F46;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x3: GLenum = 0x8F49;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x4: GLenum = 0x8F4A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3: GLenum = 0x8F47;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x2: GLenum = 0x8F4B;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x4: GLenum = 0x8F4C;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4: GLenum = 0x8F48;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x2: GLenum = 0x8F4D;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x3: GLenum = 0x8F4E;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC2: GLenum = 0x8FFC;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC3: GLenum = 0x8FFD;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC4: GLenum = 0x8FFE;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER0: GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER1: GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER10: GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER11: GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER12: GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER13: GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER14: GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER15: GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER2: GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER3: GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER4: GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER5: GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER6: GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER7: GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER8: GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER9: GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_COPY: GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_READ: GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_STORAGE_BIT: GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FILTER: GLenum = 0x829A;
#[allow(dead_code, non_upper_case_globals)] pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED_ONLY: GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x3: GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x4: GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x2: GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x4: GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x2: GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_EVEN: GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_ODD: GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_TEXTURE: GLenum = 0x829F;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BLEND: GLenum = 0x828B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const FULL_SUPPORT: GLenum = 0x82B7;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_TEXTURE: GLenum = 0x829E;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_INTEGER: GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)] pub const GUILTY_CONTEXT_RESET: GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)] pub const HALF_FLOAT: GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_FLOAT: GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_INT: GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D: GLenum = 0x904C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D_ARRAY: GLenum = 0x9052;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D: GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_ARRAY: GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_RECT: GLenum = 0x904F;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_3D: GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_FORMAT: GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_NAME: GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BUFFER: GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE: GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)] pub const INT: GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D: GLenum = 0x9057;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D: GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_RECT: GLenum = 0x905A;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_3D: GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_BUFFER: GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE: GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const ISOLINES: GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)] pub const IS_PER_PATCH: GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)] pub const IS_ROW_MAJOR: GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)] pub const LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINES_ADJACENCY: GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION: GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_COMPONENT: GLenum = 0x934A;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_INDEX: GLenum = 0x930F;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)] pub const LOWER_LEFT: GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_FLOAT: GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_INT: GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)] pub const MAJOR_VERSION: GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)] pub const MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_COHERENT_BIT: GLenum = 0x0080;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_PERSISTENT_BIT: GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_READ_BIT: GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_WRITE_BIT: GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_STRIDE: GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX: GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CULL_DISTANCES: GLenum = 0x82F9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH: GLenum = 0x8280;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_HEIGHT: GLenum = 0x827F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_SAMPLES: GLenum = 0x906D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_UNITS: GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH: GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LAYERS: GLenum = 0x8281;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_LENGTH: GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PATCH_VERTICES: GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLES: GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINES: GLenum = 0x8DE7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_STREAMS: GLenum = 0x8E71;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORTS: GLenum = 0x825B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_WIDTH: GLenum = 0x827E;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_INT: GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)] pub const MIN: GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)] pub const MINOR_VERSION: GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)] pub const MIPMAP: GLenum = 0x8293;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_LENGTH: GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION: GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_EXTENSIONS: GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_TYPE: GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)] pub const OFFSET: GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OR: GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_LSB_FIRST: GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)] pub const PATCHES: GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_VERTICES: GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM: GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_INPUT: GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_OUTPUT: GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE: GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_SEPARABLE: GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)] pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY: GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER: GLenum = 0x9192;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BARRIER_BIT: GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BINDING: GLenum = 0x9193;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT: GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT: GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_TARGET: GLenum = 0x82EA;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT: GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT_INVERTED: GLenum = 0x8E17;
#[allow(dead_code, non_upper_case_globals)] pub const R11F_G11F_B10F: GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)] pub const R16: GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)] pub const R16F: GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)] pub const R16I: GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)] pub const R16UI: GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)] pub const R16_SNORM: GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)] pub const R32F: GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)] pub const R32I: GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)] pub const R32UI: GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const R8: GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)] pub const R8I: GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)] pub const R8UI: GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)] pub const R8_SNORM: GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)] pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)] pub const READ_ONLY: GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS: GLenum = 0x828C;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_FORMAT: GLenum = 0x828D;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_TYPE: GLenum = 0x828E;
#[allow(dead_code, non_upper_case_globals)] pub const READ_WRITE: GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)] pub const RED: GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_INTEGER: GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)] pub const RG: GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)] pub const RG16: GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)] pub const RG16F: GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)] pub const RG16I: GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)] pub const RG16UI: GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)] pub const RG16_SNORM: GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)] pub const RG32F: GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)] pub const RG32I: GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)] pub const RG32UI: GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)] pub const RG8: GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)] pub const RG8I: GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)] pub const RG8UI: GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)] pub const RG8_SNORM: GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2UI: GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16F: GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16I: GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16UI: GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16_SNORM: GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32F: GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32I: GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32UI: GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565: GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8I: GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8UI: GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_SNORM: GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)] pub const RGB9_E5: GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16F: GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16I: GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16UI: GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16_SNORM: GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32F: GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32I: GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32UI: GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8I: GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8UI: GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_SNORM: GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_INTEGER: GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_INTEGER: GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)] pub const RG_INTEGER: GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER: GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D: GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_3D: GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BINDING: GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES_PASSED: GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK: GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_POSITION: GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_SHADING: GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)] pub const SET: GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER: GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_COMPILER: GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_LOAD: GLenum = 0x82A4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_STORE: GLenum = 0x82A5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_ALPHA: GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_COLOR: GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB: GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8: GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8_ALPHA8: GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_ALPHA: GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_READ: GLenum = 0x8297;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_WRITE: GLenum = 0x8298;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_COPY: GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_READ: GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_COMPONENTS: GLenum = 0x8285;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX1: GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX16: GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX4: GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_RENDERABLE: GLenum = 0x8288;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_COPY: GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_READ: GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLAGS: GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER: GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_TEXTURE: GLenum = 0x829C;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER: GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_MODE: GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_POINT_MODE: GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_SPACING: GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D: GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER: GLenum = 0x82A2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SAMPLES: GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHADOW: GLenum = 0x82A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_TARGET: GLenum = 0x1006;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW: GLenum = 0x82B5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const TIMESTAMP: GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)] pub const TIME_ELAPSED: GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const TYPE: GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNDEFINED_VERTEX: GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM: GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK: GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER: GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_SIZE: GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_TYPE: GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)] pub const UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)] pub const UPPER_LEFT: GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE: GLenum = 0x92E8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_TEXTURE: GLenum = 0x829B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_128_BITS: GLenum = 0x82C4;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_16_BITS: GLenum = 0x82CA;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_24_BITS: GLenum = 0x82C9;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_32_BITS: GLenum = 0x82C8;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_48_BITS: GLenum = 0x82C7;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_64_BITS: GLenum = 0x82C6;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_8_BITS: GLenum = 0x82CB;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_96_BITS: GLenum = 0x82C5;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
#[allow(dead_code, non_upper_case_globals)] pub const WAIT_FAILED: GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY: GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO_TO_ONE: GLenum = 0x935F;

// (?<=storage)(::.+)(\.f\))(\(.*\))
// $1$2 } else { gl$1$3 }

// (mem::transmute)
// if EXTERNAL_POINTERS { $1

// storage::
// (*STORAGE).

pub const EXTERNAL_POINTERS: bool = true;