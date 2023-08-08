use gl::types::*;
use std::mem;
use std::ffi::c_void;

pub unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::ActiveShaderProgram.f)(pipeline, program) }
pub unsafe fn ActiveTexture(texture: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ActiveTexture.f)(texture) }
pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::AttachShader.f)(program, shader) }
pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::BeginConditionalRender.f)(id, mode) }
pub unsafe fn BeginQuery(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BeginQuery.f)(target, id) }
pub unsafe fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::BeginQueryIndexed.f)(target, index, id) }
pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BeginTransformFeedback.f)(primitiveMode) }
pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindAttribLocation.f)(program, index, name) }
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindBuffer.f)(target, buffer) }
pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::BindBufferBase.f)(target, index, buffer) }
pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::BindBufferRange.f)(target, index, buffer, offset, size) }
pub unsafe fn BindBuffersBase(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint) -> ()>(storage::BindBuffersBase.f)(target, first, count, buffers) }
pub unsafe fn BindBuffersRange(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizeiptr) -> ()>(storage::BindBuffersRange.f)(target, first, count, buffers, offsets, sizes) }
pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(storage::BindFragDataLocation.f)(program, color, name) }
pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, *const GLchar) -> ()>(storage::BindFragDataLocationIndexed.f)(program, colorNumber, index, name) }
pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindFramebuffer.f)(target, framebuffer) }
pub unsafe fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum) -> ()>(storage::BindImageTexture.f)(unit, texture, level, layered, layer, access, format) }
pub unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindImageTextures.f)(first, count, textures) }
pub unsafe fn BindProgramPipeline(pipeline: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindProgramPipeline.f)(pipeline) }
pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindRenderbuffer.f)(target, renderbuffer) }
pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::BindSampler.f)(unit, sampler) }
pub unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindSamplers.f)(first, count, samplers) }
pub unsafe fn BindTexture(target: GLenum, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTexture.f)(target, texture) }
pub unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::BindTextureUnit.f)(unit, texture) }
pub unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(storage::BindTextures.f)(first, count, textures) }
pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::BindTransformFeedback.f)(target, id) }
pub unsafe fn BindVertexArray(array: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::BindVertexArray.f)(array) }
pub unsafe fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>(storage::BindVertexBuffer.f)(bindingindex, buffer, offset, stride) }
pub unsafe fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>(storage::BindVertexBuffers.f)(first, count, buffers, offsets, strides) }
pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::BlendColor.f)(red, green, blue, alpha) }
pub unsafe fn BlendEquation(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::BlendEquation.f)(mode) }
pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendEquationSeparate.f)(modeRGB, modeAlpha) }
pub unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(storage::BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) }
pub unsafe fn BlendEquationi(buf: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::BlendEquationi.f)(buf, mode) }
pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.f)(sfactor, dfactor) }
pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
pub unsafe fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> ()>(storage::BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
pub unsafe fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(storage::BlendFunci.f)(buf, src, dst) }
pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(storage::BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
pub unsafe fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) -> ()>(storage::BlitNamedFramebuffer.f)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) -> ()>(storage::BufferData.f)(target, size, data, usage) }
pub unsafe fn BufferStorage(target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield) -> ()>(storage::BufferStorage.f)(target, size, data, flags) }
pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void) -> ()>(storage::BufferSubData.f)(target, offset, size, data) }
pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(storage::CheckFramebufferStatus.f)(target) }
pub unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> GLenum>(storage::CheckNamedFramebufferStatus.f)(framebuffer, target) }
pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ClampColor.f)(target, clamp) }
pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.f)(mask) }
pub unsafe fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void) -> ()>(storage::ClearBufferData.f)(target, internalformat, format, type_, data) }
pub unsafe fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) -> ()>(storage::ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) }
pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(storage::ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) }
pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(storage::ClearBufferfv.f)(buffer, drawbuffer, value) }
pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(storage::ClearBufferiv.f)(buffer, drawbuffer, value) }
pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(storage::ClearBufferuiv.f)(buffer, drawbuffer, value) }
pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.f)(red, green, blue, alpha) }
pub unsafe fn ClearDepth(depth: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::ClearDepth.f)(depth) }
pub unsafe fn ClearDepthf(d: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearDepthf.f)(d) }
pub unsafe fn ClearNamedBufferData(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void) -> ()>(storage::ClearNamedBufferData.f)(buffer, internalformat, format, type_, data) }
pub unsafe fn ClearNamedBufferSubData(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) -> ()>(storage::ClearNamedBufferSubData.f)(buffer, internalformat, offset, size, format, type_, data) }
pub unsafe fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint) -> ()>(storage::ClearNamedFramebufferfi.f)(framebuffer, buffer, drawbuffer, depth, stencil) }
pub unsafe fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLfloat) -> ()>(storage::ClearNamedFramebufferfv.f)(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLint) -> ()>(storage::ClearNamedFramebufferiv.f)(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLuint) -> ()>(storage::ClearNamedFramebufferuiv.f)(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.f)(s) }
pub unsafe fn ClearTexImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void) -> ()>(storage::ClearTexImage.f)(texture, level, format, type_, data) }
pub unsafe fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::ClearTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) }
pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>(storage::ClientWaitSync.f)(sync, flags, timeout) }
pub unsafe fn ClipControl(origin: GLenum, depth: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ClipControl.f)(origin, depth) }
pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.f)(red, green, blue, alpha) }
pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMaski.f)(index, r, g, b, a) }
pub unsafe fn ColorP3ui(type_: GLenum, color: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::ColorP3ui.f)(type_, color) }
pub unsafe fn ColorP3uiv(type_: GLenum, color: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::ColorP3uiv.f)(type_, color) }
pub unsafe fn ColorP4ui(type_: GLenum, color: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::ColorP4ui.f)(type_, color) }
pub unsafe fn ColorP4uiv(type_: GLenum, color: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::ColorP4uiv.f)(type_, color) }
pub unsafe fn CompileShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CompileShader.f)(shader) }
pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void) -> ()>(storage::CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void) -> ()>(storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void) -> ()>(storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage1D.f)(texture, level, xoffset, width, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>(storage::CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) }
pub unsafe fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>(storage::CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
pub unsafe fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr) -> ()>(storage::CopyNamedBufferSubData.f)(readBuffer, writeBuffer, readOffset, writeOffset, size) }
pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> ()>(storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> ()>(storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
pub unsafe fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei) -> ()>(storage::CopyTextureSubImage1D.f)(texture, level, xoffset, x, y, width) }
pub unsafe fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTextureSubImage2D.f)(texture, level, xoffset, yoffset, x, y, width, height) }
pub unsafe fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::CopyTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, x, y, width, height) }
pub unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateBuffers.f)(n, buffers) }
pub unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateFramebuffers.f)(n, framebuffers) }
pub unsafe fn CreateProgram() -> GLuint { mem::transmute::<_, extern "system" fn() -> GLuint>(storage::CreateProgram.f)() }
pub unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateProgramPipelines.f)(n, pipelines) }
pub unsafe fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(storage::CreateQueries.f)(target, n, ids) }
pub unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateRenderbuffers.f)(n, renderbuffers) }
pub unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateSamplers.f)(n, samplers) }
pub unsafe fn CreateShader(type_: GLenum) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(storage::CreateShader.f)(type_) }
pub unsafe fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint>(storage::CreateShaderProgramv.f)(type_, count, strings) }
pub unsafe fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(storage::CreateTextures.f)(target, n, textures) }
pub unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateTransformFeedbacks.f)(n, ids) }
pub unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::CreateVertexArrays.f)(n, arrays) }
pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.f)(mode) }
pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const c_void) -> ()>(storage::DebugMessageCallback.f)(callback, userParam) }
pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> ()>(storage::DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> ()>(storage::DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteBuffers.f)(n, buffers) }
pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteFramebuffers.f)(n, framebuffers) }
pub unsafe fn DeleteProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteProgram.f)(program) }
pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteProgramPipelines.f)(n, pipelines) }
pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteQueries.f)(n, ids) }
pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteRenderbuffers.f)(n, renderbuffers) }
pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteSamplers.f)(count, samplers) }
pub unsafe fn DeleteShader(shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DeleteShader.f)(shader) }
pub unsafe fn DeleteSync(sync: GLsync) -> () { mem::transmute::<_, extern "system" fn(GLsync) -> ()>(storage::DeleteSync.f)(sync) }
pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTextures.f)(n, textures) }
pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteTransformFeedbacks.f)(n, ids) }
pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(storage::DeleteVertexArrays.f)(n, arrays) }
pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.f)(func) }
pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.f)(flag) }
pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::DepthRange.f)(n, f) }
pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLdouble) -> ()>(storage::DepthRangeArrayv.f)(first, count, v) }
pub unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::DepthRangeIndexed.f)(index, n, f) }
pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::DepthRangef.f)(n, f) }
pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DetachShader.f)(program, shader) }
pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.f)(cap) }
pub unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::DisableVertexArrayAttrib.f)(vaobj, index) }
pub unsafe fn DisableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::DisableVertexAttribArray.f)(index) }
pub unsafe fn Disablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Disablei.f)(target, index) }
pub unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) }
pub unsafe fn DispatchComputeIndirect(indirect: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(storage::DispatchComputeIndirect.f)(indirect) }
pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(storage::DrawArrays.f)(mode, first, count) }
pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const c_void) -> ()>(storage::DrawArraysIndirect.f)(mode, indirect) }
pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(storage::DrawArraysInstanced.f)(mode, first, count, instancecount) }
pub unsafe fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint) -> ()>(storage::DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) }
pub unsafe fn DrawBuffer(buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DrawBuffer.f)(buf) }
pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(storage::DrawBuffers.f)(n, bufs) }
pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void) -> ()>(storage::DrawElements.f)(mode, count, type_, indices) }
pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint) -> ()>(storage::DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void) -> ()>(storage::DrawElementsIndirect.f)(mode, type_, indirect) }
pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei) -> ()>(storage::DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
pub unsafe fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint) -> ()>(storage::DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) }
pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint) -> ()>(storage::DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint) -> ()>(storage::DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void) -> ()>(storage::DrawRangeElements.f)(mode, start, end, count, type_, indices) }
pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint) -> ()>(storage::DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
pub unsafe fn DrawTransformFeedback(mode: GLenum, id: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::DrawTransformFeedback.f)(mode, id) }
pub unsafe fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei) -> ()>(storage::DrawTransformFeedbackInstanced.f)(mode, id, instancecount) }
pub unsafe fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(storage::DrawTransformFeedbackStream.f)(mode, id, stream) }
pub unsafe fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei) -> ()>(storage::DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) }
pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.f)(cap) }
pub unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::EnableVertexArrayAttrib.f)(vaobj, index) }
pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::EnableVertexAttribArray.f)(index) }
pub unsafe fn Enablei(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::Enablei.f)(target, index) }
pub unsafe fn EndConditionalRender() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndConditionalRender.f)() }
pub unsafe fn EndQuery(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::EndQuery.f)(target) }
pub unsafe fn EndQueryIndexed(target: GLenum, index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::EndQueryIndexed.f)(target, index) }
pub unsafe fn EndTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.f)() }
pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(storage::FenceSync.f)(condition, flags) }
pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.f)() }
pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.f)() }
pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(storage::FlushMappedBufferRange.f)(target, offset, length) }
pub unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(storage::FlushMappedNamedBufferRange.f)(buffer, offset, length) }
pub unsafe fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::FramebufferParameteri.f)(target, pname, param) }
pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(storage::FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture.f)(target, attachment, texture, level) }
pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture1D.f)(target, attachment, textarget, texture, level) }
pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(storage::FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) }
pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>(storage::FramebufferTextureLayer.f)(target, attachment, texture, level, layer) }
pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.f)(mode) }
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenBuffers.f)(n, buffers) }
pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenFramebuffers.f)(n, framebuffers) }
pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenProgramPipelines.f)(n, pipelines) }
pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenQueries.f)(n, ids) }
pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenRenderbuffers.f)(n, renderbuffers) }
pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenSamplers.f)(count, samplers) }
pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTextures.f)(n, textures) }
pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenTransformFeedbacks.f)(n, ids) }
pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::GenVertexArrays.f)(n, arrays) }
pub unsafe fn GenerateMipmap(target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::GenerateMipmap.f)(target) }
pub unsafe fn GenerateTextureMipmap(texture: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::GenerateTextureMipmap.f)(texture) }
pub unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) }
pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) }
pub unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) }
pub unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) }
pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) -> ()>(storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> ()>(storage::GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(storage::GetAttachedShaders.f)(program, maxCount, count, shaders) }
pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetAttribLocation.f)(program, name) }
pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(storage::GetBooleani_v.f)(target, index, data) }
pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.f)(pname, data) }
pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(storage::GetBufferParameteri64v.f)(target, pname, params) }
pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetBufferParameteriv.f)(target, pname, params) }
pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const *mut c_void) -> ()>(storage::GetBufferPointerv.f)(target, pname, params) }
pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void) -> ()>(storage::GetBufferSubData.f)(target, offset, size, data) }
pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut c_void) -> ()>(storage::GetCompressedTexImage.f)(target, level, img) }
pub unsafe fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut c_void) -> ()>(storage::GetCompressedTextureImage.f)(texture, level, bufSize, pixels) }
pub unsafe fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLsizei, *mut c_void) -> ()>(storage::GetCompressedTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) }
pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint>(storage::GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLdouble) -> ()>(storage::GetDoublei_v.f)(target, index, data) }
pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetDoublev.f)(pname, data) }
pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.f)() }
pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(storage::GetFloati_v.f)(target, index, data) }
pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.f)(pname, data) }
pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetFragDataIndex.f)(program, name) }
pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetFragDataLocation.f)(program, name) }
pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetFramebufferParameteriv.f)(target, pname, params) }
pub unsafe fn GetGraphicsResetStatus() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetGraphicsResetStatus.f)() }
pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(storage::GetInteger64i_v.f)(target, index, data) }
pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(storage::GetInteger64v.f)(pname, data) }
pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(storage::GetIntegeri_v.f)(target, index, data) }
pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.f)(pname, data) }
pub unsafe fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64) -> ()>(storage::GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) }
pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> ()>(storage::GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(storage::GetMultisamplefv.f)(pname, index, val) }
pub unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(storage::GetNamedBufferParameteri64v.f)(buffer, pname, params) }
pub unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedBufferParameteriv.f)(buffer, pname, params) }
pub unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *const *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>(storage::GetNamedBufferPointerv.f)(buffer, pname, params) }
pub unsafe fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void) -> ()>(storage::GetNamedBufferSubData.f)(buffer, offset, size, data) }
pub unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetNamedFramebufferAttachmentParameteriv.f)(framebuffer, attachment, pname, params) }
pub unsafe fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedFramebufferParameteriv.f)(framebuffer, pname, param) }
pub unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetNamedRenderbufferParameteriv.f)(renderbuffer, pname, params) }
pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetObjectLabel.f)(identifier, name, bufSize, length, label) }
pub unsafe fn GetObjectPtrLabel(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
pub unsafe fn GetPointerv(pname: GLenum, params: *const *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const *mut c_void) -> ()>(storage::GetPointerv.f)(pname, params) }
pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void) -> ()>(storage::GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
pub unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
pub unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramPipelineiv.f)(pipeline, pname, params) }
pub unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(storage::GetProgramResourceIndex.f)(program, programInterface, name) }
pub unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetProgramResourceLocation.f)(program, programInterface, name) }
pub unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetProgramResourceLocationIndex.f)(program, programInterface, name) }
pub unsafe fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
pub unsafe fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *const GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>(storage::GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
pub unsafe fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(storage::GetProgramStageiv.f)(program, shadertype, pname, values) }
pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetProgramiv.f)(program, pname, params) }
pub unsafe fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjecti64v.f)(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectiv.f)(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectui64v.f)(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(storage::GetQueryBufferObjectuiv.f)(id, buffer, pname, offset) }
pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryIndexediv.f)(target, index, pname, params) }
pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(storage::GetQueryObjecti64v.f)(id, pname, params) }
pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetQueryObjectiv.f)(id, pname, params) }
pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint64) -> ()>(storage::GetQueryObjectui64v.f)(id, pname, params) }
pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetQueryObjectuiv.f)(id, pname, params) }
pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetQueryiv.f)(target, pname, params) }
pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetRenderbufferParameteriv.f)(target, pname, params) }
pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetSamplerParameterIiv.f)(sampler, pname, params) }
pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetSamplerParameterIuiv.f)(sampler, pname, params) }
pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetSamplerParameterfv.f)(sampler, pname, params) }
pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetSamplerParameteriv.f)(sampler, pname, params) }
pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>(storage::GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(storage::GetShaderSource.f)(shader, bufSize, length, source) }
pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetShaderiv.f)(shader, pname, params) }
pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.f)(name) }
pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>(storage::GetStringi.f)(name, index) }
pub unsafe fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(storage::GetSubroutineIndex.f)(program, shadertype, name) }
pub unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(storage::GetSubroutineUniformLocation.f)(program, shadertype, name) }
pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> ()>(storage::GetSynciv.f)(sync, pname, bufSize, length, values) }
pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void) -> ()>(storage::GetTexImage.f)(target, level, format, type_, pixels) }
pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTexLevelParameterfv.f)(target, level, pname, params) }
pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(storage::GetTexLevelParameteriv.f)(target, level, pname, params) }
pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameterIiv.f)(target, pname, params) }
pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(storage::GetTexParameterIuiv.f)(target, pname, params) }
pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.f)(target, pname, params) }
pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.f)(target, pname, params) }
pub unsafe fn GetTextureImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetTextureImage.f)(texture, level, format, type_, bufSize, pixels) }
pub unsafe fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTextureLevelParameterfv.f)(texture, level, pname, params) }
pub unsafe fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLint) -> ()>(storage::GetTextureLevelParameteriv.f)(texture, level, pname, params) }
pub unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTextureParameterIiv.f)(texture, pname, params) }
pub unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetTextureParameterIuiv.f)(texture, pname, params) }
pub unsafe fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetTextureParameterfv.f)(texture, pname, params) }
pub unsafe fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTextureParameteriv.f)(texture, pname, params) }
pub unsafe fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) }
pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar) -> ()>(storage::GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetTransformFeedbacki64_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64) -> ()>(storage::GetTransformFeedbacki64_v.f)(xfb, pname, index, param) }
pub unsafe fn GetTransformFeedbacki_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint) -> ()>(storage::GetTransformFeedbacki_v.f)(xfb, pname, index, param) }
pub unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetTransformFeedbackiv.f)(xfb, pname, param) }
pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(storage::GetUniformBlockIndex.f)(program, uniformBlockName) }
pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> ()>(storage::GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(storage::GetUniformLocation.f)(program, name) }
pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut GLuint) -> ()>(storage::GetUniformSubroutineuiv.f)(shadertype, location, params) }
pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLdouble) -> ()>(storage::GetUniformdv.f)(program, location, params) }
pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(storage::GetUniformfv.f)(program, location, params) }
pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(storage::GetUniformiv.f)(program, location, params) }
pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(storage::GetUniformuiv.f)(program, location, params) }
pub unsafe fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64) -> ()>(storage::GetVertexArrayIndexed64iv.f)(vaobj, index, pname, param) }
pub unsafe fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexArrayIndexediv.f)(vaobj, index, pname, param) }
pub unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexArrayiv.f)(vaobj, pname, param) }
pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribIiv.f)(index, pname, params) }
pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(storage::GetVertexAttribIuiv.f)(index, pname, params) }
pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribLdv.f)(index, pname, params) }
pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>(storage::GetVertexAttribPointerv.f)(index, pname, pointer) }
pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(storage::GetVertexAttribdv.f)(index, pname, params) }
pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(storage::GetVertexAttribfv.f)(index, pname, params) }
pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(storage::GetVertexAttribiv.f)(index, pname, params) }
pub unsafe fn GetnColorTable(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetnColorTable.f)(target, format, type_, bufSize, table) }
pub unsafe fn GetnCompressedTexImage(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, *mut c_void) -> ()>(storage::GetnCompressedTexImage.f)(target, lod, bufSize, pixels) }
pub unsafe fn GetnConvolutionFilter(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetnConvolutionFilter.f)(target, format, type_, bufSize, image) }
pub unsafe fn GetnHistogram(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetnHistogram.f)(target, reset, format, type_, bufSize, values) }
pub unsafe fn GetnMapdv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLdouble) -> ()>(storage::GetnMapdv.f)(target, query, bufSize, v) }
pub unsafe fn GetnMapfv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLfloat) -> ()>(storage::GetnMapfv.f)(target, query, bufSize, v) }
pub unsafe fn GetnMapiv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLint) -> ()>(storage::GetnMapiv.f)(target, query, bufSize, v) }
pub unsafe fn GetnMinmax(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetnMinmax.f)(target, reset, format, type_, bufSize, values) }
pub unsafe fn GetnPixelMapfv(map: GLenum, bufSize: GLsizei, values: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLfloat) -> ()>(storage::GetnPixelMapfv.f)(map, bufSize, values) }
pub unsafe fn GetnPixelMapuiv(map: GLenum, bufSize: GLsizei, values: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(storage::GetnPixelMapuiv.f)(map, bufSize, values) }
pub unsafe fn GetnPixelMapusv(map: GLenum, bufSize: GLsizei, values: *mut GLushort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLushort) -> ()>(storage::GetnPixelMapusv.f)(map, bufSize, values) }
pub unsafe fn GetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLubyte) -> ()>(storage::GetnPolygonStipple.f)(bufSize, pattern) }
pub unsafe fn GetnSeparableFilter(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void, GLsizei, *mut c_void, *mut c_void) -> ()>(storage::GetnSeparableFilter.f)(target, format, type_, rowBufSize, row, columnBufSize, column, span) }
pub unsafe fn GetnTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::GetnTexImage.f)(target, level, format, type_, bufSize, pixels) }
pub unsafe fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble) -> ()>(storage::GetnUniformdv.f)(program, location, bufSize, params) }
pub unsafe fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> ()>(storage::GetnUniformfv.f)(program, location, bufSize, params) }
pub unsafe fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> ()>(storage::GetnUniformiv.f)(program, location, bufSize, params) }
pub unsafe fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> ()>(storage::GetnUniformuiv.f)(program, location, bufSize, params) }
pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.f)(target, mode) }
pub unsafe fn InvalidateBufferData(buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::InvalidateBufferData.f)(buffer) }
pub unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(storage::InvalidateBufferSubData.f)(buffer, offset, length) }
pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>(storage::InvalidateFramebuffer.f)(target, numAttachments, attachments) }
pub unsafe fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(storage::InvalidateNamedFramebufferData.f)(framebuffer, numAttachments, attachments) }
pub unsafe fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::InvalidateNamedFramebufferSubData.f)(framebuffer, numAttachments, attachments, x, y, width, height) }
pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
pub unsafe fn InvalidateTexImage(texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(storage::InvalidateTexImage.f)(texture, level) }
pub unsafe fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> ()>(storage::InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsBuffer.f)(buffer) }
pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.f)(cap) }
pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> GLboolean>(storage::IsEnabledi.f)(target, index) }
pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsFramebuffer.f)(framebuffer) }
pub unsafe fn IsProgram(program: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgram.f)(program) }
pub unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsProgramPipeline.f)(pipeline) }
pub unsafe fn IsQuery(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsQuery.f)(id) }
pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsRenderbuffer.f)(renderbuffer) }
pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsSampler.f)(sampler) }
pub unsafe fn IsShader(shader: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsShader.f)(shader) }
pub unsafe fn IsSync(sync: GLsync) -> GLboolean { mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(storage::IsSync.f)(sync) }
pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTexture.f)(texture) }
pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsTransformFeedback.f)(id) }
pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsVertexArray.f)(array) }
pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.f)(width) }
pub unsafe fn LinkProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LinkProgram.f)(program) }
pub unsafe fn LogicOp(opcode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::LogicOp.f)(opcode) }
pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut c_void>(storage::MapBuffer.f)(target, access) }
pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void>(storage::MapBufferRange.f)(target, offset, length, access) }
pub unsafe fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut c_void { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> *mut c_void>(storage::MapNamedBuffer.f)(buffer, access) }
pub unsafe fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void>(storage::MapNamedBufferRange.f)(buffer, offset, length, access) }
pub unsafe fn MemoryBarrier(barriers: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::MemoryBarrier.f)(barriers) }
pub unsafe fn MemoryBarrierByRegion(barriers: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::MemoryBarrierByRegion.f)(barriers) }
pub unsafe fn MinSampleShading(value: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::MinSampleShading.f)(value) }
pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>(storage::MultiDrawArrays.f)(mode, first, count, drawcount) }
pub unsafe fn MultiDrawArraysIndirect(mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei) -> ()>(storage::MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) }
pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei) -> ()>(storage::MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint) -> ()>(storage::MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
pub unsafe fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei) -> ()>(storage::MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) }
pub unsafe fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::MultiTexCoordP1ui.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::MultiTexCoordP1uiv.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::MultiTexCoordP2ui.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::MultiTexCoordP2uiv.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::MultiTexCoordP3ui.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::MultiTexCoordP3uiv.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::MultiTexCoordP4ui.f)(texture, type_, coords) }
pub unsafe fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::MultiTexCoordP4uiv.f)(texture, type_, coords) }
pub unsafe fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum) -> ()>(storage::NamedBufferData.f)(buffer, size, data, usage) }
pub unsafe fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield) -> ()>(storage::NamedBufferStorage.f)(buffer, size, data, flags) }
pub unsafe fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void) -> ()>(storage::NamedBufferSubData.f)(buffer, offset, size, data) }
pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NamedFramebufferDrawBuffer.f)(framebuffer, buf) }
pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(storage::NamedFramebufferDrawBuffers.f)(framebuffer, n, bufs) }
pub unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::NamedFramebufferParameteri.f)(framebuffer, pname, param) }
pub unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NamedFramebufferReadBuffer.f)(framebuffer, src) }
pub unsafe fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLuint) -> ()>(storage::NamedFramebufferRenderbuffer.f)(framebuffer, attachment, renderbuffertarget, renderbuffer) }
pub unsafe fn NamedFramebufferTexture(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint) -> ()>(storage::NamedFramebufferTexture.f)(framebuffer, attachment, texture, level) }
pub unsafe fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint) -> ()>(storage::NamedFramebufferTextureLayer.f)(framebuffer, attachment, texture, level, layer) }
pub unsafe fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLsizei, GLsizei) -> ()>(storage::NamedRenderbufferStorage.f)(renderbuffer, internalformat, width, height) }
pub unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::NamedRenderbufferStorageMultisample.f)(renderbuffer, samples, internalformat, width, height) }
pub unsafe fn NormalP3ui(type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::NormalP3ui.f)(type_, coords) }
pub unsafe fn NormalP3uiv(type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::NormalP3uiv.f)(type_, coords) }
pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(storage::ObjectLabel.f)(identifier, name, length, label) }
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(*const c_void, GLsizei, *const GLchar) -> ()>(storage::ObjectPtrLabel.f)(ptr, length, label) }
pub unsafe fn PatchParameterfv(pname: GLenum, values: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PatchParameterfv.f)(pname, values) }
pub unsafe fn PatchParameteri(pname: GLenum, value: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PatchParameteri.f)(pname, value) }
pub unsafe fn PauseTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PauseTransformFeedback.f)() }
pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelStoref.f)(pname, param) }
pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.f)(pname, param) }
pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PointParameterf.f)(pname, param) }
pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::PointParameterfv.f)(pname, params) }
pub unsafe fn PointParameteri(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PointParameteri.f)(pname, param) }
pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::PointParameteriv.f)(pname, params) }
pub unsafe fn PointSize(size: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PointSize.f)(size) }
pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::PolygonMode.f)(face, mode) }
pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PolygonOffset.f)(factor, units) }
pub unsafe fn PopDebugGroup() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroup.f)() }
pub unsafe fn PrimitiveRestartIndex(index: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::PrimitiveRestartIndex.f)(index) }
pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const c_void, GLsizei) -> ()>(storage::ProgramBinary.f)(program, binaryFormat, binary, length) }
pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::ProgramParameteri.f)(program, pname, value) }
pub unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble) -> ()>(storage::ProgramUniform1d.f)(program, location, v0) }
pub unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform1dv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(storage::ProgramUniform1f.f)(program, location, v0) }
pub unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform1fv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(storage::ProgramUniform1i.f)(program, location, v0) }
pub unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform1iv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(storage::ProgramUniform1ui.f)(program, location, v0) }
pub unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform1uiv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble) -> ()>(storage::ProgramUniform2d.f)(program, location, v0, v1) }
pub unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform2dv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>(storage::ProgramUniform2f.f)(program, location, v0, v1) }
pub unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform2fv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform2i.f)(program, location, v0, v1) }
pub unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform2iv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(storage::ProgramUniform2ui.f)(program, location, v0, v1) }
pub unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform2uiv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble) -> ()>(storage::ProgramUniform3d.f)(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform3dv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::ProgramUniform3f.f)(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform3fv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform3i.f)(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform3iv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> ()>(storage::ProgramUniform3ui.f)(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform3uiv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::ProgramUniform4d.f)(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(storage::ProgramUniform4dv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ProgramUniform4f.f)(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(storage::ProgramUniform4fv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> ()>(storage::ProgramUniform4i.f)(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(storage::ProgramUniform4iv.f)(program, location, count, value) }
pub unsafe fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(storage::ProgramUniform4uiv.f)(program, location, count, value) }
pub unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
pub unsafe fn ProvokingVertex(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ProvokingVertex.f)(mode) }
pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(storage::PushDebugGroup.f)(source, id, length, message) }
pub unsafe fn QueryCounter(id: GLuint, target: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::QueryCounter.f)(id, target) }
pub unsafe fn ReadBuffer(src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.f)(src) }
pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void) -> ()>(storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
pub unsafe fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(storage::ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data) }
pub unsafe fn ReleaseShaderCompiler() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.f)() }
pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorage.f)(target, internalformat, width, height) }
pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) }
pub unsafe fn ResumeTransformFeedback() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::ResumeTransformFeedback.f)() }
pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(storage::SampleCoverage.f)(value, invert) }
pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(storage::SampleMaski.f)(maskNumber, mask) }
pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::SamplerParameterIiv.f)(sampler, pname, param) }
pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(storage::SamplerParameterIuiv.f)(sampler, pname, param) }
pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(storage::SamplerParameterf.f)(sampler, pname, param) }
pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(storage::SamplerParameterfv.f)(sampler, pname, param) }
pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::SamplerParameteri.f)(sampler, pname, param) }
pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::SamplerParameteriv.f)(sampler, pname, param) }
pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.f)(x, y, width, height) }
pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLint) -> ()>(storage::ScissorArrayv.f)(first, count, v) }
pub unsafe fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei) -> ()>(storage::ScissorIndexed.f)(index, left, bottom, width, height) }
pub unsafe fn ScissorIndexedv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::ScissorIndexedv.f)(index, v) }
pub unsafe fn SecondaryColorP3ui(type_: GLenum, color: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::SecondaryColorP3ui.f)(type_, color) }
pub unsafe fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::SecondaryColorP3uiv.f)(type_, color) }
pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const c_void, length: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei) -> ()>(storage::ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(storage::ShaderSource.f)(shader, count, string, length) }
pub unsafe fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) }
pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.f)(func, ref_, mask) }
pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(storage::StencilFuncSeparate.f)(face, func, ref_, mask) }
pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.f)(mask) }
pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::StencilMaskSeparate.f)(face, mask) }
pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.f)(fail, zfail, zpass) }
pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(storage::TexBuffer.f)(target, internalformat, buffer) }
pub unsafe fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TexBufferRange.f)(target, internalformat, buffer, offset, size) }
pub unsafe fn TexCoordP1ui(type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::TexCoordP1ui.f)(type_, coords) }
pub unsafe fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::TexCoordP1uiv.f)(type_, coords) }
pub unsafe fn TexCoordP2ui(type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::TexCoordP2ui.f)(type_, coords) }
pub unsafe fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::TexCoordP2uiv.f)(type_, coords) }
pub unsafe fn TexCoordP3ui(type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::TexCoordP3ui.f)(type_, coords) }
pub unsafe fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::TexCoordP3uiv.f)(type_, coords) }
pub unsafe fn TexCoordP4ui(type_: GLenum, coords: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::TexCoordP4ui.f)(type_, coords) }
pub unsafe fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::TexCoordP4uiv.f)(type_, coords) }
pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>(storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>(storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) -> ()>(storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameterIiv.f)(target, pname, params) }
pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(storage::TexParameterIuiv.f)(target, pname, params) }
pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.f)(target, pname, param) }
pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.f)(target, pname, params) }
pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.f)(target, pname, param) }
pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.f)(target, pname, params) }
pub unsafe fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei) -> ()>(storage::TexStorage1D.f)(target, levels, internalformat, width) }
pub unsafe fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::TexStorage2D.f)(target, levels, internalformat, width, height) }
pub unsafe fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>(storage::TexStorage3D.f)(target, levels, internalformat, width, height, depth) }
pub unsafe fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
pub unsafe fn TextureBarrier() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::TextureBarrier.f)() }
pub unsafe fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint) -> ()>(storage::TextureBuffer.f)(texture, internalformat, buffer) }
pub unsafe fn TextureBufferRange(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TextureBufferRange.f)(texture, internalformat, buffer, offset, size) }
pub unsafe fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::TextureParameterIiv.f)(texture, pname, params) }
pub unsafe fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(storage::TextureParameterIuiv.f)(texture, pname, params) }
pub unsafe fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(storage::TextureParameterf.f)(texture, pname, param) }
pub unsafe fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(storage::TextureParameterfv.f)(texture, pname, param) }
pub unsafe fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(storage::TextureParameteri.f)(texture, pname, param) }
pub unsafe fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(storage::TextureParameteriv.f)(texture, pname, param) }
pub unsafe fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei) -> ()>(storage::TextureStorage1D.f)(texture, levels, internalformat, width) }
pub unsafe fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(storage::TextureStorage2D.f)(texture, levels, internalformat, width, height) }
pub unsafe fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> ()>(storage::TextureStorage2DMultisample.f)(texture, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> ()>(storage::TextureStorage3D.f)(texture, levels, internalformat, width, height, depth) }
pub unsafe fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> ()>(storage::TextureStorage3DMultisample.f)(texture, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TextureSubImage1D.f)(texture, level, xoffset, width, format, type_, pixels) }
pub unsafe fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, type_, pixels) }
pub unsafe fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) -> ()>(storage::TextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
pub unsafe fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
pub unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::TransformFeedbackBufferBase.f)(xfb, index, buffer) }
pub unsafe fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(storage::TransformFeedbackBufferRange.f)(xfb, index, buffer, offset, size) }
pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>(storage::TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
pub unsafe fn Uniform1d(location: GLint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble) -> ()>(storage::Uniform1d.f)(location, x) }
pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform1dv.f)(location, count, value) }
pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(storage::Uniform1f.f)(location, v0) }
pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform1fv.f)(location, count, value) }
pub unsafe fn Uniform1i(location: GLint, v0: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Uniform1i.f)(location, v0) }
pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform1iv.f)(location, count, value) }
pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(storage::Uniform1ui.f)(location, v0) }
pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform1uiv.f)(location, count, value) }
pub unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>(storage::Uniform2d.f)(location, x, y) }
pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform2dv.f)(location, count, value) }
pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::Uniform2f.f)(location, v0, v1) }
pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform2fv.f)(location, count, value) }
pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Uniform2i.f)(location, v0, v1) }
pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform2iv.f)(location, count, value) }
pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(storage::Uniform2ui.f)(location, v0, v1) }
pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform2uiv.f)(location, count, value) }
pub unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble) -> ()>(storage::Uniform3d.f)(location, x, y, z) }
pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform3dv.f)(location, count, value) }
pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform3f.f)(location, v0, v1, v2) }
pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform3fv.f)(location, count, value) }
pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Uniform3i.f)(location, v0, v1, v2) }
pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform3iv.f)(location, count, value) }
pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform3ui.f)(location, v0, v1, v2) }
pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform3uiv.f)(location, count, value) }
pub unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Uniform4d.f)(location, x, y, z, w) }
pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(storage::Uniform4dv.f)(location, count, value) }
pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Uniform4f.f)(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(storage::Uniform4fv.f)(location, count, value) }
pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(storage::Uniform4i.f)(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(storage::Uniform4iv.f)(location, count, value) }
pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::Uniform4ui.f)(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(storage::Uniform4uiv.f)(location, count, value) }
pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) }
pub unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2x3dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x3fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix2x4dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix2x4fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3x2dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x2fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix3x4dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix3x4fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4x2dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x2fv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(storage::UniformMatrix4x3dv.f)(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(storage::UniformMatrix4x3fv.f)(location, count, transpose, value) }
pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>(storage::UniformSubroutinesuiv.f)(shadertype, count, indices) }
pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::UnmapBuffer.f)(target) }
pub unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::UnmapNamedBuffer.f)(buffer) }
pub unsafe fn UseProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::UseProgram.f)(program) }
pub unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(storage::UseProgramStages.f)(pipeline, stages, program) }
pub unsafe fn ValidateProgram(program: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgram.f)(program) }
pub unsafe fn ValidateProgramPipeline(pipeline: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ValidateProgramPipeline.f)(pipeline) }
pub unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexArrayAttribBinding.f)(vaobj, attribindex, bindingindex) }
pub unsafe fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexArrayAttribFormat.f)(vaobj, attribindex, size, type_, normalized, relativeoffset) }
pub unsafe fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexArrayAttribIFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexArrayAttribLFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexArrayBindingDivisor.f)(vaobj, bindingindex, divisor) }
pub unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexArrayElementBuffer.f)(vaobj, buffer) }
pub unsafe fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei) -> ()>(storage::VertexArrayVertexBuffer.f)(vaobj, bindingindex, buffer, offset, stride) }
pub unsafe fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) -> ()>(storage::VertexArrayVertexBuffers.f)(vaobj, first, count, buffers, offsets, strides) }
pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttrib1d.f)(index, x) }
pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib1dv.f)(index, v) }
pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(storage::VertexAttrib1f.f)(index, x) }
pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib1fv.f)(index, v) }
pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>(storage::VertexAttrib1s.f)(index, x) }
pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib1sv.f)(index, v) }
pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttrib2d.f)(index, x, y) }
pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib2dv.f)(index, v) }
pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(storage::VertexAttrib2f.f)(index, x, y) }
pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib2fv.f)(index, v) }
pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>(storage::VertexAttrib2s.f)(index, x, y) }
pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib2sv.f)(index, v) }
pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib3d.f)(index, x, y, z) }
pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib3dv.f)(index, v) }
pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib3f.f)(index, x, y, z) }
pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib3fv.f)(index, v) }
pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib3s.f)(index, x, y, z) }
pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib3sv.f)(index, v) }
pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4Nbv.f)(index, v) }
pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4Niv.f)(index, v) }
pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4Nsv.f)(index, v) }
pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::VertexAttrib4Nub.f)(index, x, y, z, w) }
pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4Nubv.f)(index, v) }
pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4Nuiv.f)(index, v) }
pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4Nusv.f)(index, v) }
pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttrib4bv.f)(index, v) }
pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttrib4d.f)(index, x, y, z, w) }
pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttrib4dv.f)(index, v) }
pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::VertexAttrib4f.f)(index, x, y, z, w) }
pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::VertexAttrib4fv.f)(index, v) }
pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttrib4iv.f)(index, v) }
pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>(storage::VertexAttrib4s.f)(index, x, y, z, w) }
pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttrib4sv.f)(index, v) }
pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttrib4ubv.f)(index, v) }
pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttrib4uiv.f)(index, v) }
pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttrib4usv.f)(index, v) }
pub unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribBinding.f)(attribindex, bindingindex) }
pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribDivisor.f)(index, divisor) }
pub unsafe fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) }
pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(storage::VertexAttribI1i.f)(index, x) }
pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI1iv.f)(index, v) }
pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexAttribI1ui.f)(index, x) }
pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI1uiv.f)(index, v) }
pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(storage::VertexAttribI2i.f)(index, x, y) }
pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI2iv.f)(index, v) }
pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI2ui.f)(index, x, y) }
pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI2uiv.f)(index, v) }
pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI3i.f)(index, x, y, z) }
pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI3iv.f)(index, v) }
pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI3ui.f)(index, x, y, z) }
pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI3uiv.f)(index, v) }
pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(storage::VertexAttribI4bv.f)(index, v) }
pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(storage::VertexAttribI4i.f)(index, x, y, z, w) }
pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(storage::VertexAttribI4iv.f)(index, v) }
pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(storage::VertexAttribI4sv.f)(index, v) }
pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(storage::VertexAttribI4ubv.f)(index, v) }
pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>(storage::VertexAttribI4ui.f)(index, x, y, z, w) }
pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(storage::VertexAttribI4uiv.f)(index, v) }
pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(storage::VertexAttribI4usv.f)(index, v) }
pub unsafe fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>(storage::VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
pub unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(storage::VertexAttribL1d.f)(index, x) }
pub unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL1dv.f)(index, v) }
pub unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(storage::VertexAttribL2d.f)(index, x, y) }
pub unsafe fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL2dv.f)(index, v) }
pub unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttribL3d.f)(index, x, y, z) }
pub unsafe fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL3dv.f)(index, v) }
pub unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::VertexAttribL4d.f)(index, x, y, z, w) }
pub unsafe fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(storage::VertexAttribL4dv.f)(index, v) }
pub unsafe fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(storage::VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>(storage::VertexAttribLPointer.f)(index, size, type_, stride, pointer) }
pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP1ui.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP1uiv.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP2ui.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP2uiv.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP3ui.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP3uiv.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(storage::VertexAttribP4ui.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(storage::VertexAttribP4uiv.f)(index, type_, normalized, value) }
pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) -> ()>(storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
pub unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(storage::VertexBindingDivisor.f)(bindingindex, divisor) }
pub unsafe fn VertexP2ui(type_: GLenum, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::VertexP2ui.f)(type_, value) }
pub unsafe fn VertexP2uiv(type_: GLenum, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::VertexP2uiv.f)(type_, value) }
pub unsafe fn VertexP3ui(type_: GLenum, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::VertexP3ui.f)(type_, value) }
pub unsafe fn VertexP3uiv(type_: GLenum, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::VertexP3uiv.f)(type_, value) }
pub unsafe fn VertexP4ui(type_: GLenum, value: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(storage::VertexP4ui.f)(type_, value) }
pub unsafe fn VertexP4uiv(type_: GLenum, value: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(storage::VertexP4uiv.f)(type_, value) }
pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.f)(x, y, width, height) }
pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLfloat) -> ()>(storage::ViewportArrayv.f)(first, count, v) }
pub unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ViewportIndexedf.f)(index, x, y, w, h) }
pub unsafe fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(storage::ViewportIndexedfv.f)(index, v) }
pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> () { mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(storage::WaitSync.f)(sync, flags, timeout) }

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

mod storage {
            use super::FnPtr;
            use std::ffi::c_void;
pub static mut ActiveShaderProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ActiveTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut AttachShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BeginConditionalRender: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BeginQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BeginQueryIndexed: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BeginTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindAttribLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindBufferBase: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindBuffersBase: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindBuffersRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindFragDataLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindFragDataLocationIndexed: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindImageTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindImageTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindSampler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindTextureUnit: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindVertexArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindVertexBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BindVertexBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendEquation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendEquationSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendEquationSeparatei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendEquationi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendFuncSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendFuncSeparatei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlendFunci: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlitFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BlitNamedFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BufferStorage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut BufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CheckFramebufferStatus: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CheckNamedFramebufferStatus: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClampColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Clear: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferfi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearBufferuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearDepth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearDepthf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedBufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedFramebufferfi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedFramebufferfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedFramebufferiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearNamedFramebufferuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearStencil: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClearTexSubImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClientWaitSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ClipControl: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorMaski: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorP4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ColorP4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompileShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTextureSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTextureSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CompressedTextureSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyImageSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyNamedBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTexImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTextureSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTextureSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CopyTextureSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateFramebuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateProgramPipelines: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateQueries: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateRenderbuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateShaderProgramv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateTransformFeedbacks: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CreateVertexArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut CullFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DebugMessageCallback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DebugMessageControl: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DebugMessageInsert: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteFramebuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteProgramPipelines: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteQueries: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteRenderbuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteTransformFeedbacks: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DeleteVertexArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthRangeArrayv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthRangeIndexed: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DepthRangef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DetachShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Disable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DisableVertexArrayAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DisableVertexAttribArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Disablei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DispatchCompute: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DispatchComputeIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawArraysIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawArraysInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawArraysInstancedBaseInstance: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstancedBaseInstance: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstancedBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawElementsInstancedBaseVertexBaseInstance: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawRangeElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawRangeElementsBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawTransformFeedbackInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawTransformFeedbackStream: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut DrawTransformFeedbackStreamInstanced: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Enable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EnableVertexArrayAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EnableVertexAttribArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Enablei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EndConditionalRender: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EndQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EndQueryIndexed: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut EndTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FenceSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Finish: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Flush: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FlushMappedBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FlushMappedNamedBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FramebufferTextureLayer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut FrontFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenFramebuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenProgramPipelines: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenQueries: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenRenderbuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenSamplers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenTransformFeedbacks: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenVertexArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenerateMipmap: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GenerateTextureMipmap: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveAtomicCounterBufferiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveAttrib: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveSubroutineName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveSubroutineUniformName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveSubroutineUniformiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveUniform: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformBlockName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetActiveUniformsiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetAttachedShaders: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetAttribLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBooleani_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBooleanv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBufferParameteri64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBufferPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetCompressedTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetCompressedTextureImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetCompressedTextureSubImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetDebugMessageLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetDoublei_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetDoublev: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetError: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFloati_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFloatv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFragDataIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFragDataLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetFramebufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetGraphicsResetStatus: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetInteger64i_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetInteger64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetIntegeri_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetIntegerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetInternalformati64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetInternalformativ: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetMultisamplefv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedBufferParameteri64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedBufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedBufferPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedFramebufferAttachmentParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedFramebufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetNamedRenderbufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetObjectLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetObjectPtrLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramInterfaceiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramPipelineInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramPipelineiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceLocationIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceName: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramResourceiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramStageiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetProgramiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryBufferObjecti64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryBufferObjectiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryBufferObjectui64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryBufferObjectuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryIndexediv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryObjecti64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryObjectiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryObjectui64v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryObjectuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetQueryiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSamplerParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetShaderInfoLog: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetShaderSource: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetShaderiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetString: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetStringi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSubroutineIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSubroutineUniformLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetSynciv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexLevelParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureLevelParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureLevelParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTextureSubImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTransformFeedbacki64_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTransformFeedbacki_v: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetTransformFeedbackiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformBlockIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformIndices: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformLocation: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformSubroutineuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetUniformuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexArrayIndexed64iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexArrayIndexediv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexArrayiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribLdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetVertexAttribiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnColorTable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnCompressedTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnConvolutionFilter: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnHistogram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnMapdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnMapfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnMapiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnMinmax: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnPixelMapfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnPixelMapuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnPixelMapusv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnPolygonStipple: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnSeparableFilter: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnUniformdv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnUniformfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnUniformiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut GetnUniformuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Hint: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateBufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateNamedFramebufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateNamedFramebufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateSubFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateTexImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut InvalidateTexSubImage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsEnabled: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsEnabledi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsFramebuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsQuery: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsSampler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsShader: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut IsVertexArray: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut LineWidth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut LinkProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut LogicOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MapBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MapBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MapNamedBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MapNamedBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MemoryBarrier: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MemoryBarrierByRegion: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MinSampleShading: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiDrawArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiDrawArraysIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiDrawElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiDrawElementsBaseVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiDrawElementsIndirect: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut MultiTexCoordP4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedBufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedBufferStorage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedBufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferDrawBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferDrawBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferReadBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferRenderbuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedFramebufferTextureLayer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedRenderbufferStorage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NamedRenderbufferStorageMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NormalP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut NormalP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ObjectLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ObjectPtrLabel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PatchParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PatchParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PauseTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PixelStoref: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PixelStorei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PointParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PointParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PointParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PointParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PointSize: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PolygonMode: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PolygonOffset: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PopDebugGroup: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PrimitiveRestartIndex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniform4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix2x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix3x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProgramUniformMatrix4x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ProvokingVertex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut PushDebugGroup: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut QueryCounter: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ReadBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ReadPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ReadnPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ReleaseShaderCompiler: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut RenderbufferStorage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ResumeTransformFeedback: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SampleCoverage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SampleMaski: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SamplerParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Scissor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ScissorArrayv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ScissorIndexed: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ScissorIndexedv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SecondaryColorP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut SecondaryColorP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ShaderBinary: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ShaderSource: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ShaderStorageBlockBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilFuncSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilMaskSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut StencilOpSeparate: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexCoordP4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexImage2DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexImage3DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexStorage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexStorage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexStorage2DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexStorage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexStorage3DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TexSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureBarrier: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameterIiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameterIuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureStorage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureStorage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureStorage2DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureStorage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureStorage3DMultisample: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureSubImage1D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureSubImage3D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TextureView: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TransformFeedbackBufferBase: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TransformFeedbackBufferRange: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut TransformFeedbackVaryings: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Uniform4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformBlockBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix2x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix3x4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformMatrix4x3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UniformSubroutinesuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UnmapBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UnmapNamedBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UseProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut UseProgramStages: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ValidateProgram: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ValidateProgramPipeline: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayAttribBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayAttribFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayAttribIFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayAttribLFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayBindingDivisor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayElementBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayVertexBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexArrayVertexBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib1sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib2sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib3sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nbv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Niv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nsv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nub: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nuiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4Nusv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4bv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4fv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4s: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4ubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttrib4usv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribBinding: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribDivisor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI1i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI1iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI2i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI2iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI3i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI3iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4bv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4i: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4iv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4sv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4ubv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribI4usv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribIFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribIPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL1d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL1dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL2d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL2dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL3d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL3dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL4d: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribL4dv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribLFormat: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribLPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP1ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP1uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribP4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexAttribPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexBindingDivisor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP2ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP2uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP3ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP3uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP4ui: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut VertexP4uiv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut Viewport: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ViewportArrayv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ViewportIndexedf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut ViewportIndexedfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
pub static mut WaitSync: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const c_void,
                is_loaded: false
            };
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