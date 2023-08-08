use gl::types::*;
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

pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

impl FnPtr {
    /// Creates a `FnPtr` from a load attempt.
    pub fn new(ptr: *const c_void) -> FnPtr {
        if ptr.is_null() {
            FnPtr { f: missing_fn_panic as *const c_void, is_loaded: false }
        } else {
            FnPtr { f: ptr, is_loaded: true }
        }
    }
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
            ActiveShaderProgram: FnPtr::new(0 as *const c_void),
            ActiveTexture: FnPtr::new(0 as *const c_void),
            AttachShader: FnPtr::new(0 as *const c_void),
            BeginConditionalRender: FnPtr::new(0 as *const c_void),
            BeginQuery: FnPtr::new(0 as *const c_void),
            BeginQueryIndexed: FnPtr::new(0 as *const c_void),
            BeginTransformFeedback: FnPtr::new(0 as *const c_void),
            BindAttribLocation: FnPtr::new(0 as *const c_void),
            BindBuffer: FnPtr::new(0 as *const c_void),
            BindBufferBase: FnPtr::new(0 as *const c_void),
            BindBufferRange: FnPtr::new(0 as *const c_void),
            BindBuffersBase: FnPtr::new(0 as *const c_void),
            BindBuffersRange: FnPtr::new(0 as *const c_void),
            BindFragDataLocation: FnPtr::new(0 as *const c_void),
            BindFragDataLocationIndexed: FnPtr::new(0 as *const c_void),
            BindFramebuffer: FnPtr::new(0 as *const c_void),
            BindImageTexture: FnPtr::new(0 as *const c_void),
            BindImageTextures: FnPtr::new(0 as *const c_void),
            BindProgramPipeline: FnPtr::new(0 as *const c_void),
            BindRenderbuffer: FnPtr::new(0 as *const c_void),
            BindSampler: FnPtr::new(0 as *const c_void),
            BindSamplers: FnPtr::new(0 as *const c_void),
            BindTexture: FnPtr::new(0 as *const c_void),
            BindTextureUnit: FnPtr::new(0 as *const c_void),
            BindTextures: FnPtr::new(0 as *const c_void),
            BindTransformFeedback: FnPtr::new(0 as *const c_void),
            BindVertexArray: FnPtr::new(0 as *const c_void),
            BindVertexBuffer: FnPtr::new(0 as *const c_void),
            BindVertexBuffers: FnPtr::new(0 as *const c_void),
            BlendColor: FnPtr::new(0 as *const c_void),
            BlendEquation: FnPtr::new(0 as *const c_void),
            BlendEquationSeparate: FnPtr::new(0 as *const c_void),
            BlendEquationSeparatei: FnPtr::new(0 as *const c_void),
            BlendEquationi: FnPtr::new(0 as *const c_void),
            BlendFunc: FnPtr::new(0 as *const c_void),
            BlendFuncSeparate: FnPtr::new(0 as *const c_void),
            BlendFuncSeparatei: FnPtr::new(0 as *const c_void),
            BlendFunci: FnPtr::new(0 as *const c_void),
            BlitFramebuffer: FnPtr::new(0 as *const c_void),
            BlitNamedFramebuffer: FnPtr::new(0 as *const c_void),
            BufferData: FnPtr::new(0 as *const c_void),
            BufferStorage: FnPtr::new(0 as *const c_void),
            BufferSubData: FnPtr::new(0 as *const c_void),
            CheckFramebufferStatus: FnPtr::new(0 as *const c_void),
            CheckNamedFramebufferStatus: FnPtr::new(0 as *const c_void),
            ClampColor: FnPtr::new(0 as *const c_void),
            Clear: FnPtr::new(0 as *const c_void),
            ClearBufferData: FnPtr::new(0 as *const c_void),
            ClearBufferSubData: FnPtr::new(0 as *const c_void),
            ClearBufferfi: FnPtr::new(0 as *const c_void),
            ClearBufferfv: FnPtr::new(0 as *const c_void),
            ClearBufferiv: FnPtr::new(0 as *const c_void),
            ClearBufferuiv: FnPtr::new(0 as *const c_void),
            ClearColor: FnPtr::new(0 as *const c_void),
            ClearDepth: FnPtr::new(0 as *const c_void),
            ClearDepthf: FnPtr::new(0 as *const c_void),
            ClearNamedBufferData: FnPtr::new(0 as *const c_void),
            ClearNamedBufferSubData: FnPtr::new(0 as *const c_void),
            ClearNamedFramebufferfi: FnPtr::new(0 as *const c_void),
            ClearNamedFramebufferfv: FnPtr::new(0 as *const c_void),
            ClearNamedFramebufferiv: FnPtr::new(0 as *const c_void),
            ClearNamedFramebufferuiv: FnPtr::new(0 as *const c_void),
            ClearStencil: FnPtr::new(0 as *const c_void),
            ClearTexImage: FnPtr::new(0 as *const c_void),
            ClearTexSubImage: FnPtr::new(0 as *const c_void),
            ClientWaitSync: FnPtr::new(0 as *const c_void),
            ClipControl: FnPtr::new(0 as *const c_void),
            ColorMask: FnPtr::new(0 as *const c_void),
            ColorMaski: FnPtr::new(0 as *const c_void),
            ColorP3ui: FnPtr::new(0 as *const c_void),
            ColorP3uiv: FnPtr::new(0 as *const c_void),
            ColorP4ui: FnPtr::new(0 as *const c_void),
            ColorP4uiv: FnPtr::new(0 as *const c_void),
            CompileShader: FnPtr::new(0 as *const c_void),
            CompressedTexImage1D: FnPtr::new(0 as *const c_void),
            CompressedTexImage2D: FnPtr::new(0 as *const c_void),
            CompressedTexImage3D: FnPtr::new(0 as *const c_void),
            CompressedTexSubImage1D: FnPtr::new(0 as *const c_void),
            CompressedTexSubImage2D: FnPtr::new(0 as *const c_void),
            CompressedTexSubImage3D: FnPtr::new(0 as *const c_void),
            CompressedTextureSubImage1D: FnPtr::new(0 as *const c_void),
            CompressedTextureSubImage2D: FnPtr::new(0 as *const c_void),
            CompressedTextureSubImage3D: FnPtr::new(0 as *const c_void),
            CopyBufferSubData: FnPtr::new(0 as *const c_void),
            CopyImageSubData: FnPtr::new(0 as *const c_void),
            CopyNamedBufferSubData: FnPtr::new(0 as *const c_void),
            CopyTexImage1D: FnPtr::new(0 as *const c_void),
            CopyTexImage2D: FnPtr::new(0 as *const c_void),
            CopyTexSubImage1D: FnPtr::new(0 as *const c_void),
            CopyTexSubImage2D: FnPtr::new(0 as *const c_void),
            CopyTexSubImage3D: FnPtr::new(0 as *const c_void),
            CopyTextureSubImage1D: FnPtr::new(0 as *const c_void),
            CopyTextureSubImage2D: FnPtr::new(0 as *const c_void),
            CopyTextureSubImage3D: FnPtr::new(0 as *const c_void),
            CreateBuffers: FnPtr::new(0 as *const c_void),
            CreateFramebuffers: FnPtr::new(0 as *const c_void),
            CreateProgram: FnPtr::new(0 as *const c_void),
            CreateProgramPipelines: FnPtr::new(0 as *const c_void),
            CreateQueries: FnPtr::new(0 as *const c_void),
            CreateRenderbuffers: FnPtr::new(0 as *const c_void),
            CreateSamplers: FnPtr::new(0 as *const c_void),
            CreateShader: FnPtr::new(0 as *const c_void),
            CreateShaderProgramv: FnPtr::new(0 as *const c_void),
            CreateTextures: FnPtr::new(0 as *const c_void),
            CreateTransformFeedbacks: FnPtr::new(0 as *const c_void),
            CreateVertexArrays: FnPtr::new(0 as *const c_void),
            CullFace: FnPtr::new(0 as *const c_void),
            DebugMessageCallback: FnPtr::new(0 as *const c_void),
            DebugMessageControl: FnPtr::new(0 as *const c_void),
            DebugMessageInsert: FnPtr::new(0 as *const c_void),
            DeleteBuffers: FnPtr::new(0 as *const c_void),
            DeleteFramebuffers: FnPtr::new(0 as *const c_void),
            DeleteProgram: FnPtr::new(0 as *const c_void),
            DeleteProgramPipelines: FnPtr::new(0 as *const c_void),
            DeleteQueries: FnPtr::new(0 as *const c_void),
            DeleteRenderbuffers: FnPtr::new(0 as *const c_void),
            DeleteSamplers: FnPtr::new(0 as *const c_void),
            DeleteShader: FnPtr::new(0 as *const c_void),
            DeleteSync: FnPtr::new(0 as *const c_void),
            DeleteTextures: FnPtr::new(0 as *const c_void),
            DeleteTransformFeedbacks: FnPtr::new(0 as *const c_void),
            DeleteVertexArrays: FnPtr::new(0 as *const c_void),
            DepthFunc: FnPtr::new(0 as *const c_void),
            DepthMask: FnPtr::new(0 as *const c_void),
            DepthRange: FnPtr::new(0 as *const c_void),
            DepthRangeArrayv: FnPtr::new(0 as *const c_void),
            DepthRangeIndexed: FnPtr::new(0 as *const c_void),
            DepthRangef: FnPtr::new(0 as *const c_void),
            DetachShader: FnPtr::new(0 as *const c_void),
            Disable: FnPtr::new(0 as *const c_void),
            DisableVertexArrayAttrib: FnPtr::new(0 as *const c_void),
            DisableVertexAttribArray: FnPtr::new(0 as *const c_void),
            Disablei: FnPtr::new(0 as *const c_void),
            DispatchCompute: FnPtr::new(0 as *const c_void),
            DispatchComputeIndirect: FnPtr::new(0 as *const c_void),
            DrawArrays: FnPtr::new(0 as *const c_void),
            DrawArraysIndirect: FnPtr::new(0 as *const c_void),
            DrawArraysInstanced: FnPtr::new(0 as *const c_void),
            DrawArraysInstancedBaseInstance: FnPtr::new(0 as *const c_void),
            DrawBuffer: FnPtr::new(0 as *const c_void),
            DrawBuffers: FnPtr::new(0 as *const c_void),
            DrawElements: FnPtr::new(0 as *const c_void),
            DrawElementsBaseVertex: FnPtr::new(0 as *const c_void),
            DrawElementsIndirect: FnPtr::new(0 as *const c_void),
            DrawElementsInstanced: FnPtr::new(0 as *const c_void),
            DrawElementsInstancedBaseInstance: FnPtr::new(0 as *const c_void),
            DrawElementsInstancedBaseVertex: FnPtr::new(0 as *const c_void),
            DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new(0 as *const c_void),
            DrawRangeElements: FnPtr::new(0 as *const c_void),
            DrawRangeElementsBaseVertex: FnPtr::new(0 as *const c_void),
            DrawTransformFeedback: FnPtr::new(0 as *const c_void),
            DrawTransformFeedbackInstanced: FnPtr::new(0 as *const c_void),
            DrawTransformFeedbackStream: FnPtr::new(0 as *const c_void),
            DrawTransformFeedbackStreamInstanced: FnPtr::new(0 as *const c_void),
            Enable: FnPtr::new(0 as *const c_void),
            EnableVertexArrayAttrib: FnPtr::new(0 as *const c_void),
            EnableVertexAttribArray: FnPtr::new(0 as *const c_void),
            Enablei: FnPtr::new(0 as *const c_void),
            EndConditionalRender: FnPtr::new(0 as *const c_void),
            EndQuery: FnPtr::new(0 as *const c_void),
            EndQueryIndexed: FnPtr::new(0 as *const c_void),
            EndTransformFeedback: FnPtr::new(0 as *const c_void),
            FenceSync: FnPtr::new(0 as *const c_void),
            Finish: FnPtr::new(0 as *const c_void),
            Flush: FnPtr::new(0 as *const c_void),
            FlushMappedBufferRange: FnPtr::new(0 as *const c_void),
            FlushMappedNamedBufferRange: FnPtr::new(0 as *const c_void),
            FramebufferParameteri: FnPtr::new(0 as *const c_void),
            FramebufferRenderbuffer: FnPtr::new(0 as *const c_void),
            FramebufferTexture: FnPtr::new(0 as *const c_void),
            FramebufferTexture1D: FnPtr::new(0 as *const c_void),
            FramebufferTexture2D: FnPtr::new(0 as *const c_void),
            FramebufferTexture3D: FnPtr::new(0 as *const c_void),
            FramebufferTextureLayer: FnPtr::new(0 as *const c_void),
            FrontFace: FnPtr::new(0 as *const c_void),
            GenBuffers: FnPtr::new(0 as *const c_void),
            GenFramebuffers: FnPtr::new(0 as *const c_void),
            GenProgramPipelines: FnPtr::new(0 as *const c_void),
            GenQueries: FnPtr::new(0 as *const c_void),
            GenRenderbuffers: FnPtr::new(0 as *const c_void),
            GenSamplers: FnPtr::new(0 as *const c_void),
            GenTextures: FnPtr::new(0 as *const c_void),
            GenTransformFeedbacks: FnPtr::new(0 as *const c_void),
            GenVertexArrays: FnPtr::new(0 as *const c_void),
            GenerateMipmap: FnPtr::new(0 as *const c_void),
            GenerateTextureMipmap: FnPtr::new(0 as *const c_void),
            GetActiveAtomicCounterBufferiv: FnPtr::new(0 as *const c_void),
            GetActiveAttrib: FnPtr::new(0 as *const c_void),
            GetActiveSubroutineName: FnPtr::new(0 as *const c_void),
            GetActiveSubroutineUniformName: FnPtr::new(0 as *const c_void),
            GetActiveSubroutineUniformiv: FnPtr::new(0 as *const c_void),
            GetActiveUniform: FnPtr::new(0 as *const c_void),
            GetActiveUniformBlockName: FnPtr::new(0 as *const c_void),
            GetActiveUniformBlockiv: FnPtr::new(0 as *const c_void),
            GetActiveUniformName: FnPtr::new(0 as *const c_void),
            GetActiveUniformsiv: FnPtr::new(0 as *const c_void),
            GetAttachedShaders: FnPtr::new(0 as *const c_void),
            GetAttribLocation: FnPtr::new(0 as *const c_void),
            GetBooleani_v: FnPtr::new(0 as *const c_void),
            GetBooleanv: FnPtr::new(0 as *const c_void),
            GetBufferParameteri64v: FnPtr::new(0 as *const c_void),
            GetBufferParameteriv: FnPtr::new(0 as *const c_void),
            GetBufferPointerv: FnPtr::new(0 as *const c_void),
            GetBufferSubData: FnPtr::new(0 as *const c_void),
            GetCompressedTexImage: FnPtr::new(0 as *const c_void),
            GetCompressedTextureImage: FnPtr::new(0 as *const c_void),
            GetCompressedTextureSubImage: FnPtr::new(0 as *const c_void),
            GetDebugMessageLog: FnPtr::new(0 as *const c_void),
            GetDoublei_v: FnPtr::new(0 as *const c_void),
            GetDoublev: FnPtr::new(0 as *const c_void),
            GetError: FnPtr::new(0 as *const c_void),
            GetFloati_v: FnPtr::new(0 as *const c_void),
            GetFloatv: FnPtr::new(0 as *const c_void),
            GetFragDataIndex: FnPtr::new(0 as *const c_void),
            GetFragDataLocation: FnPtr::new(0 as *const c_void),
            GetFramebufferAttachmentParameteriv: FnPtr::new(0 as *const c_void),
            GetFramebufferParameteriv: FnPtr::new(0 as *const c_void),
            GetGraphicsResetStatus: FnPtr::new(0 as *const c_void),
            GetInteger64i_v: FnPtr::new(0 as *const c_void),
            GetInteger64v: FnPtr::new(0 as *const c_void),
            GetIntegeri_v: FnPtr::new(0 as *const c_void),
            GetIntegerv: FnPtr::new(0 as *const c_void),
            GetInternalformati64v: FnPtr::new(0 as *const c_void),
            GetInternalformativ: FnPtr::new(0 as *const c_void),
            GetMultisamplefv: FnPtr::new(0 as *const c_void),
            GetNamedBufferParameteri64v: FnPtr::new(0 as *const c_void),
            GetNamedBufferParameteriv: FnPtr::new(0 as *const c_void),
            GetNamedBufferPointerv: FnPtr::new(0 as *const c_void),
            GetNamedBufferSubData: FnPtr::new(0 as *const c_void),
            GetNamedFramebufferAttachmentParameteriv: FnPtr::new(0 as *const c_void),
            GetNamedFramebufferParameteriv: FnPtr::new(0 as *const c_void),
            GetNamedRenderbufferParameteriv: FnPtr::new(0 as *const c_void),
            GetObjectLabel: FnPtr::new(0 as *const c_void),
            GetObjectPtrLabel: FnPtr::new(0 as *const c_void),
            GetPointerv: FnPtr::new(0 as *const c_void),
            GetProgramBinary: FnPtr::new(0 as *const c_void),
            GetProgramInfoLog: FnPtr::new(0 as *const c_void),
            GetProgramInterfaceiv: FnPtr::new(0 as *const c_void),
            GetProgramPipelineInfoLog: FnPtr::new(0 as *const c_void),
            GetProgramPipelineiv: FnPtr::new(0 as *const c_void),
            GetProgramResourceIndex: FnPtr::new(0 as *const c_void),
            GetProgramResourceLocation: FnPtr::new(0 as *const c_void),
            GetProgramResourceLocationIndex: FnPtr::new(0 as *const c_void),
            GetProgramResourceName: FnPtr::new(0 as *const c_void),
            GetProgramResourceiv: FnPtr::new(0 as *const c_void),
            GetProgramStageiv: FnPtr::new(0 as *const c_void),
            GetProgramiv: FnPtr::new(0 as *const c_void),
            GetQueryBufferObjecti64v: FnPtr::new(0 as *const c_void),
            GetQueryBufferObjectiv: FnPtr::new(0 as *const c_void),
            GetQueryBufferObjectui64v: FnPtr::new(0 as *const c_void),
            GetQueryBufferObjectuiv: FnPtr::new(0 as *const c_void),
            GetQueryIndexediv: FnPtr::new(0 as *const c_void),
            GetQueryObjecti64v: FnPtr::new(0 as *const c_void),
            GetQueryObjectiv: FnPtr::new(0 as *const c_void),
            GetQueryObjectui64v: FnPtr::new(0 as *const c_void),
            GetQueryObjectuiv: FnPtr::new(0 as *const c_void),
            GetQueryiv: FnPtr::new(0 as *const c_void),
            GetRenderbufferParameteriv: FnPtr::new(0 as *const c_void),
            GetSamplerParameterIiv: FnPtr::new(0 as *const c_void),
            GetSamplerParameterIuiv: FnPtr::new(0 as *const c_void),
            GetSamplerParameterfv: FnPtr::new(0 as *const c_void),
            GetSamplerParameteriv: FnPtr::new(0 as *const c_void),
            GetShaderInfoLog: FnPtr::new(0 as *const c_void),
            GetShaderPrecisionFormat: FnPtr::new(0 as *const c_void),
            GetShaderSource: FnPtr::new(0 as *const c_void),
            GetShaderiv: FnPtr::new(0 as *const c_void),
            GetString: FnPtr::new(0 as *const c_void),
            GetStringi: FnPtr::new(0 as *const c_void),
            GetSubroutineIndex: FnPtr::new(0 as *const c_void),
            GetSubroutineUniformLocation: FnPtr::new(0 as *const c_void),
            GetSynciv: FnPtr::new(0 as *const c_void),
            GetTexImage: FnPtr::new(0 as *const c_void),
            GetTexLevelParameterfv: FnPtr::new(0 as *const c_void),
            GetTexLevelParameteriv: FnPtr::new(0 as *const c_void),
            GetTexParameterIiv: FnPtr::new(0 as *const c_void),
            GetTexParameterIuiv: FnPtr::new(0 as *const c_void),
            GetTexParameterfv: FnPtr::new(0 as *const c_void),
            GetTexParameteriv: FnPtr::new(0 as *const c_void),
            GetTextureImage: FnPtr::new(0 as *const c_void),
            GetTextureLevelParameterfv: FnPtr::new(0 as *const c_void),
            GetTextureLevelParameteriv: FnPtr::new(0 as *const c_void),
            GetTextureParameterIiv: FnPtr::new(0 as *const c_void),
            GetTextureParameterIuiv: FnPtr::new(0 as *const c_void),
            GetTextureParameterfv: FnPtr::new(0 as *const c_void),
            GetTextureParameteriv: FnPtr::new(0 as *const c_void),
            GetTextureSubImage: FnPtr::new(0 as *const c_void),
            GetTransformFeedbackVarying: FnPtr::new(0 as *const c_void),
            GetTransformFeedbacki64_v: FnPtr::new(0 as *const c_void),
            GetTransformFeedbacki_v: FnPtr::new(0 as *const c_void),
            GetTransformFeedbackiv: FnPtr::new(0 as *const c_void),
            GetUniformBlockIndex: FnPtr::new(0 as *const c_void),
            GetUniformIndices: FnPtr::new(0 as *const c_void),
            GetUniformLocation: FnPtr::new(0 as *const c_void),
            GetUniformSubroutineuiv: FnPtr::new(0 as *const c_void),
            GetUniformdv: FnPtr::new(0 as *const c_void),
            GetUniformfv: FnPtr::new(0 as *const c_void),
            GetUniformiv: FnPtr::new(0 as *const c_void),
            GetUniformuiv: FnPtr::new(0 as *const c_void),
            GetVertexArrayIndexed64iv: FnPtr::new(0 as *const c_void),
            GetVertexArrayIndexediv: FnPtr::new(0 as *const c_void),
            GetVertexArrayiv: FnPtr::new(0 as *const c_void),
            GetVertexAttribIiv: FnPtr::new(0 as *const c_void),
            GetVertexAttribIuiv: FnPtr::new(0 as *const c_void),
            GetVertexAttribLdv: FnPtr::new(0 as *const c_void),
            GetVertexAttribPointerv: FnPtr::new(0 as *const c_void),
            GetVertexAttribdv: FnPtr::new(0 as *const c_void),
            GetVertexAttribfv: FnPtr::new(0 as *const c_void),
            GetVertexAttribiv: FnPtr::new(0 as *const c_void),
            GetnColorTable: FnPtr::new(0 as *const c_void),
            GetnCompressedTexImage: FnPtr::new(0 as *const c_void),
            GetnConvolutionFilter: FnPtr::new(0 as *const c_void),
            GetnHistogram: FnPtr::new(0 as *const c_void),
            GetnMapdv: FnPtr::new(0 as *const c_void),
            GetnMapfv: FnPtr::new(0 as *const c_void),
            GetnMapiv: FnPtr::new(0 as *const c_void),
            GetnMinmax: FnPtr::new(0 as *const c_void),
            GetnPixelMapfv: FnPtr::new(0 as *const c_void),
            GetnPixelMapuiv: FnPtr::new(0 as *const c_void),
            GetnPixelMapusv: FnPtr::new(0 as *const c_void),
            GetnPolygonStipple: FnPtr::new(0 as *const c_void),
            GetnSeparableFilter: FnPtr::new(0 as *const c_void),
            GetnTexImage: FnPtr::new(0 as *const c_void),
            GetnUniformdv: FnPtr::new(0 as *const c_void),
            GetnUniformfv: FnPtr::new(0 as *const c_void),
            GetnUniformiv: FnPtr::new(0 as *const c_void),
            GetnUniformuiv: FnPtr::new(0 as *const c_void),
            Hint: FnPtr::new(0 as *const c_void),
            InvalidateBufferData: FnPtr::new(0 as *const c_void),
            InvalidateBufferSubData: FnPtr::new(0 as *const c_void),
            InvalidateFramebuffer: FnPtr::new(0 as *const c_void),
            InvalidateNamedFramebufferData: FnPtr::new(0 as *const c_void),
            InvalidateNamedFramebufferSubData: FnPtr::new(0 as *const c_void),
            InvalidateSubFramebuffer: FnPtr::new(0 as *const c_void),
            InvalidateTexImage: FnPtr::new(0 as *const c_void),
            InvalidateTexSubImage: FnPtr::new(0 as *const c_void),
            IsBuffer: FnPtr::new(0 as *const c_void),
            IsEnabled: FnPtr::new(0 as *const c_void),
            IsEnabledi: FnPtr::new(0 as *const c_void),
            IsFramebuffer: FnPtr::new(0 as *const c_void),
            IsProgram: FnPtr::new(0 as *const c_void),
            IsProgramPipeline: FnPtr::new(0 as *const c_void),
            IsQuery: FnPtr::new(0 as *const c_void),
            IsRenderbuffer: FnPtr::new(0 as *const c_void),
            IsSampler: FnPtr::new(0 as *const c_void),
            IsShader: FnPtr::new(0 as *const c_void),
            IsSync: FnPtr::new(0 as *const c_void),
            IsTexture: FnPtr::new(0 as *const c_void),
            IsTransformFeedback: FnPtr::new(0 as *const c_void),
            IsVertexArray: FnPtr::new(0 as *const c_void),
            LineWidth: FnPtr::new(0 as *const c_void),
            LinkProgram: FnPtr::new(0 as *const c_void),
            LogicOp: FnPtr::new(0 as *const c_void),
            MapBuffer: FnPtr::new(0 as *const c_void),
            MapBufferRange: FnPtr::new(0 as *const c_void),
            MapNamedBuffer: FnPtr::new(0 as *const c_void),
            MapNamedBufferRange: FnPtr::new(0 as *const c_void),
            MemoryBarrier: FnPtr::new(0 as *const c_void),
            MemoryBarrierByRegion: FnPtr::new(0 as *const c_void),
            MinSampleShading: FnPtr::new(0 as *const c_void),
            MultiDrawArrays: FnPtr::new(0 as *const c_void),
            MultiDrawArraysIndirect: FnPtr::new(0 as *const c_void),
            MultiDrawElements: FnPtr::new(0 as *const c_void),
            MultiDrawElementsBaseVertex: FnPtr::new(0 as *const c_void),
            MultiDrawElementsIndirect: FnPtr::new(0 as *const c_void),
            MultiTexCoordP1ui: FnPtr::new(0 as *const c_void),
            MultiTexCoordP1uiv: FnPtr::new(0 as *const c_void),
            MultiTexCoordP2ui: FnPtr::new(0 as *const c_void),
            MultiTexCoordP2uiv: FnPtr::new(0 as *const c_void),
            MultiTexCoordP3ui: FnPtr::new(0 as *const c_void),
            MultiTexCoordP3uiv: FnPtr::new(0 as *const c_void),
            MultiTexCoordP4ui: FnPtr::new(0 as *const c_void),
            MultiTexCoordP4uiv: FnPtr::new(0 as *const c_void),
            NamedBufferData: FnPtr::new(0 as *const c_void),
            NamedBufferStorage: FnPtr::new(0 as *const c_void),
            NamedBufferSubData: FnPtr::new(0 as *const c_void),
            NamedFramebufferDrawBuffer: FnPtr::new(0 as *const c_void),
            NamedFramebufferDrawBuffers: FnPtr::new(0 as *const c_void),
            NamedFramebufferParameteri: FnPtr::new(0 as *const c_void),
            NamedFramebufferReadBuffer: FnPtr::new(0 as *const c_void),
            NamedFramebufferRenderbuffer: FnPtr::new(0 as *const c_void),
            NamedFramebufferTexture: FnPtr::new(0 as *const c_void),
            NamedFramebufferTextureLayer: FnPtr::new(0 as *const c_void),
            NamedRenderbufferStorage: FnPtr::new(0 as *const c_void),
            NamedRenderbufferStorageMultisample: FnPtr::new(0 as *const c_void),
            NormalP3ui: FnPtr::new(0 as *const c_void),
            NormalP3uiv: FnPtr::new(0 as *const c_void),
            ObjectLabel: FnPtr::new(0 as *const c_void),
            ObjectPtrLabel: FnPtr::new(0 as *const c_void),
            PatchParameterfv: FnPtr::new(0 as *const c_void),
            PatchParameteri: FnPtr::new(0 as *const c_void),
            PauseTransformFeedback: FnPtr::new(0 as *const c_void),
            PixelStoref: FnPtr::new(0 as *const c_void),
            PixelStorei: FnPtr::new(0 as *const c_void),
            PointParameterf: FnPtr::new(0 as *const c_void),
            PointParameterfv: FnPtr::new(0 as *const c_void),
            PointParameteri: FnPtr::new(0 as *const c_void),
            PointParameteriv: FnPtr::new(0 as *const c_void),
            PointSize: FnPtr::new(0 as *const c_void),
            PolygonMode: FnPtr::new(0 as *const c_void),
            PolygonOffset: FnPtr::new(0 as *const c_void),
            PopDebugGroup: FnPtr::new(0 as *const c_void),
            PrimitiveRestartIndex: FnPtr::new(0 as *const c_void),
            ProgramBinary: FnPtr::new(0 as *const c_void),
            ProgramParameteri: FnPtr::new(0 as *const c_void),
            ProgramUniform1d: FnPtr::new(0 as *const c_void),
            ProgramUniform1dv: FnPtr::new(0 as *const c_void),
            ProgramUniform1f: FnPtr::new(0 as *const c_void),
            ProgramUniform1fv: FnPtr::new(0 as *const c_void),
            ProgramUniform1i: FnPtr::new(0 as *const c_void),
            ProgramUniform1iv: FnPtr::new(0 as *const c_void),
            ProgramUniform1ui: FnPtr::new(0 as *const c_void),
            ProgramUniform1uiv: FnPtr::new(0 as *const c_void),
            ProgramUniform2d: FnPtr::new(0 as *const c_void),
            ProgramUniform2dv: FnPtr::new(0 as *const c_void),
            ProgramUniform2f: FnPtr::new(0 as *const c_void),
            ProgramUniform2fv: FnPtr::new(0 as *const c_void),
            ProgramUniform2i: FnPtr::new(0 as *const c_void),
            ProgramUniform2iv: FnPtr::new(0 as *const c_void),
            ProgramUniform2ui: FnPtr::new(0 as *const c_void),
            ProgramUniform2uiv: FnPtr::new(0 as *const c_void),
            ProgramUniform3d: FnPtr::new(0 as *const c_void),
            ProgramUniform3dv: FnPtr::new(0 as *const c_void),
            ProgramUniform3f: FnPtr::new(0 as *const c_void),
            ProgramUniform3fv: FnPtr::new(0 as *const c_void),
            ProgramUniform3i: FnPtr::new(0 as *const c_void),
            ProgramUniform3iv: FnPtr::new(0 as *const c_void),
            ProgramUniform3ui: FnPtr::new(0 as *const c_void),
            ProgramUniform3uiv: FnPtr::new(0 as *const c_void),
            ProgramUniform4d: FnPtr::new(0 as *const c_void),
            ProgramUniform4dv: FnPtr::new(0 as *const c_void),
            ProgramUniform4f: FnPtr::new(0 as *const c_void),
            ProgramUniform4fv: FnPtr::new(0 as *const c_void),
            ProgramUniform4i: FnPtr::new(0 as *const c_void),
            ProgramUniform4iv: FnPtr::new(0 as *const c_void),
            ProgramUniform4ui: FnPtr::new(0 as *const c_void),
            ProgramUniform4uiv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2x3dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2x3fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2x4dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix2x4fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3x2dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3x2fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3x4dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix3x4fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4x2dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4x2fv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4x3dv: FnPtr::new(0 as *const c_void),
            ProgramUniformMatrix4x3fv: FnPtr::new(0 as *const c_void),
            ProvokingVertex: FnPtr::new(0 as *const c_void),
            PushDebugGroup: FnPtr::new(0 as *const c_void),
            QueryCounter: FnPtr::new(0 as *const c_void),
            ReadBuffer: FnPtr::new(0 as *const c_void),
            ReadPixels: FnPtr::new(0 as *const c_void),
            ReadnPixels: FnPtr::new(0 as *const c_void),
            ReleaseShaderCompiler: FnPtr::new(0 as *const c_void),
            RenderbufferStorage: FnPtr::new(0 as *const c_void),
            RenderbufferStorageMultisample: FnPtr::new(0 as *const c_void),
            ResumeTransformFeedback: FnPtr::new(0 as *const c_void),
            SampleCoverage: FnPtr::new(0 as *const c_void),
            SampleMaski: FnPtr::new(0 as *const c_void),
            SamplerParameterIiv: FnPtr::new(0 as *const c_void),
            SamplerParameterIuiv: FnPtr::new(0 as *const c_void),
            SamplerParameterf: FnPtr::new(0 as *const c_void),
            SamplerParameterfv: FnPtr::new(0 as *const c_void),
            SamplerParameteri: FnPtr::new(0 as *const c_void),
            SamplerParameteriv: FnPtr::new(0 as *const c_void),
            Scissor: FnPtr::new(0 as *const c_void),
            ScissorArrayv: FnPtr::new(0 as *const c_void),
            ScissorIndexed: FnPtr::new(0 as *const c_void),
            ScissorIndexedv: FnPtr::new(0 as *const c_void),
            SecondaryColorP3ui: FnPtr::new(0 as *const c_void),
            SecondaryColorP3uiv: FnPtr::new(0 as *const c_void),
            ShaderBinary: FnPtr::new(0 as *const c_void),
            ShaderSource: FnPtr::new(0 as *const c_void),
            ShaderStorageBlockBinding: FnPtr::new(0 as *const c_void),
            StencilFunc: FnPtr::new(0 as *const c_void),
            StencilFuncSeparate: FnPtr::new(0 as *const c_void),
            StencilMask: FnPtr::new(0 as *const c_void),
            StencilMaskSeparate: FnPtr::new(0 as *const c_void),
            StencilOp: FnPtr::new(0 as *const c_void),
            StencilOpSeparate: FnPtr::new(0 as *const c_void),
            TexBuffer: FnPtr::new(0 as *const c_void),
            TexBufferRange: FnPtr::new(0 as *const c_void),
            TexCoordP1ui: FnPtr::new(0 as *const c_void),
            TexCoordP1uiv: FnPtr::new(0 as *const c_void),
            TexCoordP2ui: FnPtr::new(0 as *const c_void),
            TexCoordP2uiv: FnPtr::new(0 as *const c_void),
            TexCoordP3ui: FnPtr::new(0 as *const c_void),
            TexCoordP3uiv: FnPtr::new(0 as *const c_void),
            TexCoordP4ui: FnPtr::new(0 as *const c_void),
            TexCoordP4uiv: FnPtr::new(0 as *const c_void),
            TexImage1D: FnPtr::new(0 as *const c_void),
            TexImage2D: FnPtr::new(0 as *const c_void),
            TexImage2DMultisample: FnPtr::new(0 as *const c_void),
            TexImage3D: FnPtr::new(0 as *const c_void),
            TexImage3DMultisample: FnPtr::new(0 as *const c_void),
            TexParameterIiv: FnPtr::new(0 as *const c_void),
            TexParameterIuiv: FnPtr::new(0 as *const c_void),
            TexParameterf: FnPtr::new(0 as *const c_void),
            TexParameterfv: FnPtr::new(0 as *const c_void),
            TexParameteri: FnPtr::new(0 as *const c_void),
            TexParameteriv: FnPtr::new(0 as *const c_void),
            TexStorage1D: FnPtr::new(0 as *const c_void),
            TexStorage2D: FnPtr::new(0 as *const c_void),
            TexStorage2DMultisample: FnPtr::new(0 as *const c_void),
            TexStorage3D: FnPtr::new(0 as *const c_void),
            TexStorage3DMultisample: FnPtr::new(0 as *const c_void),
            TexSubImage1D: FnPtr::new(0 as *const c_void),
            TexSubImage2D: FnPtr::new(0 as *const c_void),
            TexSubImage3D: FnPtr::new(0 as *const c_void),
            TextureBarrier: FnPtr::new(0 as *const c_void),
            TextureBuffer: FnPtr::new(0 as *const c_void),
            TextureBufferRange: FnPtr::new(0 as *const c_void),
            TextureParameterIiv: FnPtr::new(0 as *const c_void),
            TextureParameterIuiv: FnPtr::new(0 as *const c_void),
            TextureParameterf: FnPtr::new(0 as *const c_void),
            TextureParameterfv: FnPtr::new(0 as *const c_void),
            TextureParameteri: FnPtr::new(0 as *const c_void),
            TextureParameteriv: FnPtr::new(0 as *const c_void),
            TextureStorage1D: FnPtr::new(0 as *const c_void),
            TextureStorage2D: FnPtr::new(0 as *const c_void),
            TextureStorage2DMultisample: FnPtr::new(0 as *const c_void),
            TextureStorage3D: FnPtr::new(0 as *const c_void),
            TextureStorage3DMultisample: FnPtr::new(0 as *const c_void),
            TextureSubImage1D: FnPtr::new(0 as *const c_void),
            TextureSubImage2D: FnPtr::new(0 as *const c_void),
            TextureSubImage3D: FnPtr::new(0 as *const c_void),
            TextureView: FnPtr::new(0 as *const c_void),
            TransformFeedbackBufferBase: FnPtr::new(0 as *const c_void),
            TransformFeedbackBufferRange: FnPtr::new(0 as *const c_void),
            TransformFeedbackVaryings: FnPtr::new(0 as *const c_void),
            Uniform1d: FnPtr::new(0 as *const c_void),
            Uniform1dv: FnPtr::new(0 as *const c_void),
            Uniform1f: FnPtr::new(0 as *const c_void),
            Uniform1fv: FnPtr::new(0 as *const c_void),
            Uniform1i: FnPtr::new(0 as *const c_void),
            Uniform1iv: FnPtr::new(0 as *const c_void),
            Uniform1ui: FnPtr::new(0 as *const c_void),
            Uniform1uiv: FnPtr::new(0 as *const c_void),
            Uniform2d: FnPtr::new(0 as *const c_void),
            Uniform2dv: FnPtr::new(0 as *const c_void),
            Uniform2f: FnPtr::new(0 as *const c_void),
            Uniform2fv: FnPtr::new(0 as *const c_void),
            Uniform2i: FnPtr::new(0 as *const c_void),
            Uniform2iv: FnPtr::new(0 as *const c_void),
            Uniform2ui: FnPtr::new(0 as *const c_void),
            Uniform2uiv: FnPtr::new(0 as *const c_void),
            Uniform3d: FnPtr::new(0 as *const c_void),
            Uniform3dv: FnPtr::new(0 as *const c_void),
            Uniform3f: FnPtr::new(0 as *const c_void),
            Uniform3fv: FnPtr::new(0 as *const c_void),
            Uniform3i: FnPtr::new(0 as *const c_void),
            Uniform3iv: FnPtr::new(0 as *const c_void),
            Uniform3ui: FnPtr::new(0 as *const c_void),
            Uniform3uiv: FnPtr::new(0 as *const c_void),
            Uniform4d: FnPtr::new(0 as *const c_void),
            Uniform4dv: FnPtr::new(0 as *const c_void),
            Uniform4f: FnPtr::new(0 as *const c_void),
            Uniform4fv: FnPtr::new(0 as *const c_void),
            Uniform4i: FnPtr::new(0 as *const c_void),
            Uniform4iv: FnPtr::new(0 as *const c_void),
            Uniform4ui: FnPtr::new(0 as *const c_void),
            Uniform4uiv: FnPtr::new(0 as *const c_void),
            UniformBlockBinding: FnPtr::new(0 as *const c_void),
            UniformMatrix2dv: FnPtr::new(0 as *const c_void),
            UniformMatrix2fv: FnPtr::new(0 as *const c_void),
            UniformMatrix2x3dv: FnPtr::new(0 as *const c_void),
            UniformMatrix2x3fv: FnPtr::new(0 as *const c_void),
            UniformMatrix2x4dv: FnPtr::new(0 as *const c_void),
            UniformMatrix2x4fv: FnPtr::new(0 as *const c_void),
            UniformMatrix3dv: FnPtr::new(0 as *const c_void),
            UniformMatrix3fv: FnPtr::new(0 as *const c_void),
            UniformMatrix3x2dv: FnPtr::new(0 as *const c_void),
            UniformMatrix3x2fv: FnPtr::new(0 as *const c_void),
            UniformMatrix3x4dv: FnPtr::new(0 as *const c_void),
            UniformMatrix3x4fv: FnPtr::new(0 as *const c_void),
            UniformMatrix4dv: FnPtr::new(0 as *const c_void),
            UniformMatrix4fv: FnPtr::new(0 as *const c_void),
            UniformMatrix4x2dv: FnPtr::new(0 as *const c_void),
            UniformMatrix4x2fv: FnPtr::new(0 as *const c_void),
            UniformMatrix4x3dv: FnPtr::new(0 as *const c_void),
            UniformMatrix4x3fv: FnPtr::new(0 as *const c_void),
            UniformSubroutinesuiv: FnPtr::new(0 as *const c_void),
            UnmapBuffer: FnPtr::new(0 as *const c_void),
            UnmapNamedBuffer: FnPtr::new(0 as *const c_void),
            UseProgram: FnPtr::new(0 as *const c_void),
            UseProgramStages: FnPtr::new(0 as *const c_void),
            ValidateProgram: FnPtr::new(0 as *const c_void),
            ValidateProgramPipeline: FnPtr::new(0 as *const c_void),
            VertexArrayAttribBinding: FnPtr::new(0 as *const c_void),
            VertexArrayAttribFormat: FnPtr::new(0 as *const c_void),
            VertexArrayAttribIFormat: FnPtr::new(0 as *const c_void),
            VertexArrayAttribLFormat: FnPtr::new(0 as *const c_void),
            VertexArrayBindingDivisor: FnPtr::new(0 as *const c_void),
            VertexArrayElementBuffer: FnPtr::new(0 as *const c_void),
            VertexArrayVertexBuffer: FnPtr::new(0 as *const c_void),
            VertexArrayVertexBuffers: FnPtr::new(0 as *const c_void),
            VertexAttrib1d: FnPtr::new(0 as *const c_void),
            VertexAttrib1dv: FnPtr::new(0 as *const c_void),
            VertexAttrib1f: FnPtr::new(0 as *const c_void),
            VertexAttrib1fv: FnPtr::new(0 as *const c_void),
            VertexAttrib1s: FnPtr::new(0 as *const c_void),
            VertexAttrib1sv: FnPtr::new(0 as *const c_void),
            VertexAttrib2d: FnPtr::new(0 as *const c_void),
            VertexAttrib2dv: FnPtr::new(0 as *const c_void),
            VertexAttrib2f: FnPtr::new(0 as *const c_void),
            VertexAttrib2fv: FnPtr::new(0 as *const c_void),
            VertexAttrib2s: FnPtr::new(0 as *const c_void),
            VertexAttrib2sv: FnPtr::new(0 as *const c_void),
            VertexAttrib3d: FnPtr::new(0 as *const c_void),
            VertexAttrib3dv: FnPtr::new(0 as *const c_void),
            VertexAttrib3f: FnPtr::new(0 as *const c_void),
            VertexAttrib3fv: FnPtr::new(0 as *const c_void),
            VertexAttrib3s: FnPtr::new(0 as *const c_void),
            VertexAttrib3sv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nbv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Niv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nsv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nub: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nubv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nuiv: FnPtr::new(0 as *const c_void),
            VertexAttrib4Nusv: FnPtr::new(0 as *const c_void),
            VertexAttrib4bv: FnPtr::new(0 as *const c_void),
            VertexAttrib4d: FnPtr::new(0 as *const c_void),
            VertexAttrib4dv: FnPtr::new(0 as *const c_void),
            VertexAttrib4f: FnPtr::new(0 as *const c_void),
            VertexAttrib4fv: FnPtr::new(0 as *const c_void),
            VertexAttrib4iv: FnPtr::new(0 as *const c_void),
            VertexAttrib4s: FnPtr::new(0 as *const c_void),
            VertexAttrib4sv: FnPtr::new(0 as *const c_void),
            VertexAttrib4ubv: FnPtr::new(0 as *const c_void),
            VertexAttrib4uiv: FnPtr::new(0 as *const c_void),
            VertexAttrib4usv: FnPtr::new(0 as *const c_void),
            VertexAttribBinding: FnPtr::new(0 as *const c_void),
            VertexAttribDivisor: FnPtr::new(0 as *const c_void),
            VertexAttribFormat: FnPtr::new(0 as *const c_void),
            VertexAttribI1i: FnPtr::new(0 as *const c_void),
            VertexAttribI1iv: FnPtr::new(0 as *const c_void),
            VertexAttribI1ui: FnPtr::new(0 as *const c_void),
            VertexAttribI1uiv: FnPtr::new(0 as *const c_void),
            VertexAttribI2i: FnPtr::new(0 as *const c_void),
            VertexAttribI2iv: FnPtr::new(0 as *const c_void),
            VertexAttribI2ui: FnPtr::new(0 as *const c_void),
            VertexAttribI2uiv: FnPtr::new(0 as *const c_void),
            VertexAttribI3i: FnPtr::new(0 as *const c_void),
            VertexAttribI3iv: FnPtr::new(0 as *const c_void),
            VertexAttribI3ui: FnPtr::new(0 as *const c_void),
            VertexAttribI3uiv: FnPtr::new(0 as *const c_void),
            VertexAttribI4bv: FnPtr::new(0 as *const c_void),
            VertexAttribI4i: FnPtr::new(0 as *const c_void),
            VertexAttribI4iv: FnPtr::new(0 as *const c_void),
            VertexAttribI4sv: FnPtr::new(0 as *const c_void),
            VertexAttribI4ubv: FnPtr::new(0 as *const c_void),
            VertexAttribI4ui: FnPtr::new(0 as *const c_void),
            VertexAttribI4uiv: FnPtr::new(0 as *const c_void),
            VertexAttribI4usv: FnPtr::new(0 as *const c_void),
            VertexAttribIFormat: FnPtr::new(0 as *const c_void),
            VertexAttribIPointer: FnPtr::new(0 as *const c_void),
            VertexAttribL1d: FnPtr::new(0 as *const c_void),
            VertexAttribL1dv: FnPtr::new(0 as *const c_void),
            VertexAttribL2d: FnPtr::new(0 as *const c_void),
            VertexAttribL2dv: FnPtr::new(0 as *const c_void),
            VertexAttribL3d: FnPtr::new(0 as *const c_void),
            VertexAttribL3dv: FnPtr::new(0 as *const c_void),
            VertexAttribL4d: FnPtr::new(0 as *const c_void),
            VertexAttribL4dv: FnPtr::new(0 as *const c_void),
            VertexAttribLFormat: FnPtr::new(0 as *const c_void),
            VertexAttribLPointer: FnPtr::new(0 as *const c_void),
            VertexAttribP1ui: FnPtr::new(0 as *const c_void),
            VertexAttribP1uiv: FnPtr::new(0 as *const c_void),
            VertexAttribP2ui: FnPtr::new(0 as *const c_void),
            VertexAttribP2uiv: FnPtr::new(0 as *const c_void),
            VertexAttribP3ui: FnPtr::new(0 as *const c_void),
            VertexAttribP3uiv: FnPtr::new(0 as *const c_void),
            VertexAttribP4ui: FnPtr::new(0 as *const c_void),
            VertexAttribP4uiv: FnPtr::new(0 as *const c_void),
            VertexAttribPointer: FnPtr::new(0 as *const c_void),
            VertexBindingDivisor: FnPtr::new(0 as *const c_void),
            VertexP2ui: FnPtr::new(0 as *const c_void),
            VertexP2uiv: FnPtr::new(0 as *const c_void),
            VertexP3ui: FnPtr::new(0 as *const c_void),
            VertexP3uiv: FnPtr::new(0 as *const c_void),
            VertexP4ui: FnPtr::new(0 as *const c_void),
            VertexP4uiv: FnPtr::new(0 as *const c_void),
            Viewport: FnPtr::new(0 as *const c_void),
            ViewportArrayv: FnPtr::new(0 as *const c_void),
            ViewportIndexedf: FnPtr::new(0 as *const c_void),
            ViewportIndexedfv: FnPtr::new(0 as *const c_void),
            WaitSync: FnPtr::new(0 as *const c_void) 
        }
    }
}

pub static mut STORAGE: *const PointerStorage = 0 as *const PointerStorage;

pub fn missing_fn_panic() -> ! {
    panic!("gl function was not loaded")
}

pub const EXTERNAL_POINTERS: bool = false;

// (?<=storage)(::.+)(\.f\))(\(.*\))
// $1$2 } else { gl$1$3 }


// (mem::transmute)
// if EXTERNAL_POINTERS { $1

// storage::
// (*STORAGE).
