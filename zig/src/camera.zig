const glm = @import("glm.zig");
const std = @import("std");
const c = @cImport({
    @cInclude("glad/gl.h");
    @cInclude("GLFW/glfw3.h");
});
pub const Camera = struct {
    pos: packed struct { x: f32, y: f32, z: f32 },
    rot: packed struct { pitch: f32, yaw: f32},
    ratio: f32,

    pub fn new() Camera {
        return Camera {
            .pos = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
            .rot = .{ .pitch = 0.0, .yaw = 0.0 },
            .ratio = 1.0,
        };
    }
    pub fn move(self: *Camera, keyMap: std.AutoHashMap(c_int, bool), mouseMovement: struct { x: f64, y: f64 }) void {
        self.rot.yaw += @floatCast(f32, mouseMovement.x / 200.0);
        self.rot.pitch -= @floatCast(f32, mouseMovement.y / 200.0);

        if (keyMap.get(c.GLFW_KEY_Q) orelse false) {
            self.rot.yaw = 0.0;
            self.rot.pitch = 0.0;
        }

        var it = keyMap.iterator();
        while (it.next()) |entry| {
            if (entry.value_ptr.* == false) { continue; }
            switch (entry.key_ptr.*) {
                c.GLFW_KEY_W => {
                    self.step(0.0, -1.0);
                },
                c.GLFW_KEY_S => {
                    self.step(0.0, 1.0);
                },
                c.GLFW_KEY_A => {
                    self.step(1.0, 0.0);
                },
                c.GLFW_KEY_D => {
                    self.step(-1.0, 0.0);
                },
                c.GLFW_KEY_LEFT_SHIFT => {
                    self.pos.y += 1;
                },
                c.GLFW_KEY_SPACE => {
                    self.pos.y -= 1;
                },
                else => {}
            }
        }
    }
    
    pub fn step(self: *Camera, x: f64, z: f64) void {
        self.pos.x += @floatCast(f32, (std.math.cos(self.rot.yaw)) * x - (std.math.sin(self.rot.yaw)) * z);
        self.pos.z -= @floatCast(f32, (std.math.sin(self.rot.yaw)) * x + (std.math.cos(self.rot.yaw)) * z);
    }


    pub fn getMatrix(self: *Camera) glm.Mat4 {
        var mat = glm.Mat4.identity();

        mat = mat.matmul(glm.perspective(3.14159 / 2.0, self.ratio, 0.1, 1000.0));
        mat = mat.matmul(glm.rotation(self.rot.pitch, glm.vec3(1, 0, 0)));
        mat = mat.matmul(glm.rotation(-self.rot.yaw, glm.vec3(0, 1, 0)));
        mat = mat.matmul(glm.translation(@bitCast(glm.Vec3, self.pos)));

        return mat;
    }
};