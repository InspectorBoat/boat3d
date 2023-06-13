const c = @cImport({
    @cInclude("glad/gl.h");
    @cInclude("GLFW/glfw3.h");
});
const print = @import("std").debug.print;

pub const GlBuffer = struct {
    id: u32,
    pub fn create() GlBuffer {
        var id: u32 = 0;
        c.glCreateBuffers(1, &id);
        return GlBuffer {
            .id = id
        };
    }
    pub fn bindBuffer(self: *GlBuffer, target: u32) void {
        c.glBindBuffer(target, self.id);
    }
    pub fn bindBufferBase(self: *GlBuffer, target: u32) void {
        c.glBindBufferBase(target, 0, self.id);
    }
    pub fn isBuffer(self: *GlBuffer) bool {
        return c.glIsBuffer(self.id) == c.glTrue;
    }
    pub fn deleteBuffer(self: *GlBuffer) void {
        c.glDeleteBuffers(1, &self.id);
    }
    pub fn bufferStorage(
        self: *GlBuffer,
        bytes: isize,
        flags: u32
    ) void {
        c.glNamedBufferStorage(
            self.id,
            bytes,
            null,
            flags
        );
    }
    pub fn bufferData(
        self: *GlBuffer,
        length: isize,
        data: ?*const anyopaque,
        usage: c_uint
    ) void {
        c.glNamedBufferData(
            self.id,
            length,
            data,
            usage
        );
    }
    pub fn bufferSubData(
        self: *GlBuffer,
        start: isize,
        length: isize,
        data: ?*const anyopaque,
    ) void {
        c.glNamedBufferSubData(
            self.id,
            start,
            length,
            data
        );
    }
    pub fn getSubData(
        self: *GlBuffer,
        offset: isize,
        length: isize,
        data: ?* anyopaque
    ) void {
        c.glGetNamedBufferSubData(
            self.id,
            offset,
            length,
            data
        );
    }
    pub fn unbind(target: u32) void {
        c.glBindBuffer(c.GL_NONE, target);
    }
};

pub const GlShader = struct {
    id: u32,
    pub fn create(shaderType: u32, source: *const u8) GlShader {
        var id = c.glCreateShader(shaderType);
        c.glShaderSource(id, 1, &source, null);
        c.glCompileShader(id);

        var status: i32 = 0;
        c.glGetShaderiv(id, c.GL_COMPILE_STATUS, &status);
        if (status != c.GL_TRUE) {
            print("Failed to compile {s} shader", .{ if (shaderType == c.GL_VERTEX_SHADER) "vertex" else "fragment" });
        }
        return GlShader {
            .id = id
        };
    }
};

pub const GlProgram = struct {
    id: u32,
    pub fn create(vertexShader: GlShader, fragmentShader: GlShader) GlProgram {
        var glProgram = GlProgram { .id = c.glCreateProgram() };
        c.glAttachShader(glProgram.id, vertexShader.id);
        c.glAttachShader(glProgram.id, fragmentShader.id);
        c.glLinkProgram(glProgram.id);

        var status: i32 = 0;
        c.glGetProgramiv(glProgram.id, c.GL_LINK_STATUS, &status);
        if (status != c.GL_TRUE) { print("FUCK", .{}); }
        return glProgram;
    }
    pub fn useProgram(self: *GlProgram) void {
        c.glUseProgram(self.id);
    }
};
