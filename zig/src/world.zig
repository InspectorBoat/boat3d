const Chunk = @import("chunk.zig").Chunk;
const ByteBuffer = @import("bytebuffer.zig").ByteBuffer;
const BlockFace = @import("block.zig").BlockFace;
const print = @import("main.zig").print;
const std = @import("std");
const Camera = @import("camera.zig").Camera;
const GlBuffer = @import("gl.zig").GlBuffer;
const c = @cImport({
    @cInclude("glad/gl.h");
    @cInclude("GLFW/glfw3.h");
});

pub const World = struct {
    chunks: [32768]Chunk,
    camera: Camera,
    pub fn new() *World {
        var world = std.heap.page_allocator.create(World) catch unreachable;
        world.camera = Camera.new();
        var random = Lcg { .val = 1 };
        var counter: usize = 0;
        var x: usize = 0; while (x < 32) : (x += 1) {
            var y: usize = 0; while (y < 32) : (y += 1) {
                var z: usize = 0; while (z < 32) : (z += 1) {
                    var chunk = &world.chunks[(x << 10) | (y << 5) | (z << 0)];
                    chunk.makeTerrain(&random, &counter);
                    chunk.pos = .{ .x = @intCast(i32, x), .y = @intCast(i32, y), .z = @intCast(i32, z) };
                    chunk.buffer = GlBuffer.create();
                }
            }
        }
        return world;
    }
    pub fn meshAllChunks(self: *World) void {
        const startTime = std.time.nanoTimestamp();
        var byteBuffer: *ByteBuffer = ByteBuffer.new();
        var totalFaces: usize = 0;
        {
            var x: usize = 0; while (x < 32) : (x += 1) {
                var y: usize = 0; while (y < 32) : (y += 1) {
                    var z: usize = 0; while (z < 32) : (z += 1) {
                        var chunk = &self.chunks[(x << 10) | (y << 5) | (z << 0)];
                        if (@bitCast(i96, chunk.pos) == 0) {
                            chunk.meshNorthSouth(byteBuffer, self);
                            chunk.meshEastWest(byteBuffer, self);
                            chunk.meshBelowAbove(byteBuffer, self);
                        }

                        byteBuffer.formatQuads();

                        chunk.buffer.bufferStorage(@intCast(i64, byteBuffer.ind + 16), c.GL_DYNAMIC_STORAGE_BIT);
                        chunk.buffer.bufferSubData(0, 12, &chunk.pos);
                        chunk.buffer.bufferSubData(16, @intCast(i64, byteBuffer.ind), &byteBuffer.arr);

                        totalFaces += @intCast(usize, byteBuffer.ind / 8);
                        chunk.faces = @intCast(i32, byteBuffer.ind / 8);
                        byteBuffer.ind = 0;
                    }
                }
            }
        }
        print("faces: {} | time: {d}\n", .{totalFaces, @intToFloat(f64, std.time.nanoTimestamp() - startTime) / 1000000.0});
    }
};

pub const Lcg = struct {
    val: u32,
    pub fn next(self: *Lcg) u32 {
        self.val = @addWithOverflow(@mulWithOverflow(self.val, 1103515245).@"0", 12346).@"0" % (2147483648 - 1);
        return self.val;
    }
};