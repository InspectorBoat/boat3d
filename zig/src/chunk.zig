const World = @import("world.zig").World;
const BlockFace = @import("block.zig").BlockFace;
const ByteBuffer = @import("bytebuffer.zig").ByteBuffer;
const BLOCKS = @import("main.zig").BLOCKS;
const print = @import("main.zig").print;
const std = @import("std");
const Lcg = @import("world.zig").Lcg;
const Normal = @import("block.zig").Normal;
const GlBuffer = @import("gl.zig").GlBuffer;
pub const Chunk = struct {
    blocks: [4096]u8,
    counts: [6]i32,
    offsets: [6]*void,
    pos: packed struct { x: i32, y: i32, z: i32 },
    buffer: GlBuffer,
    faces: i32,
    pub fn makeTerrain(self: *Chunk, random: *Lcg, counter: *usize) void {
        var i: usize = 0; while (i < 4096) : (i += 1) {
            self.blocks[i] = if (random.next() % 1024 == 0) 1 else 0;
            // self.blocks[i] = if (i == 0x000 or i == 0x002 or i == 0x100 or i == 0x102) 1 else 0;
            self.blocks[i] = if (i == 0x000) 1 else 0;
            counter.* += self.blocks[i];
        }
        // var x: usize = 0; while (x < 16) : (x += 1) {
        //     var y: usize = 0; while (y < 16) : (y += 1) {
        //         var z: usize = 0; while (z < 16) : (z += 1) {
        //             if (z == 0) {
        //                 self.blocks[(x << 8) | (y << 4) | (z << 0)] = 1;
        //             }
        //             else {
        //                 self.blocks[(x << 8) | (y << 4) | (z << 0)] = 0;
        //             }
        //             // if ((x - 8) * (x - 8) + (y - 8) * (y - 8) + (z - 8) * (z - 8) <= 16) {
        //             //     self.blocks[(x << 8) | (y << 4) | (z << 0)] = 1;
        //             // }
        //             // else {
        //             //     self.blocks[(x << 8) s| (y << 4) | (z << 0)] = 0;
        //             // }
        //         }
        //     }
        // }
    }
    pub fn getSouthFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        return &BLOCKS[self.blocks[index]].model[Normal.SOUT.toInt()];
    }
    pub fn getWestFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        return &BLOCKS[self.blocks[index]].model[Normal.WEST.toInt()];
    }
    pub fn getBelowFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        return &BLOCKS[self.blocks[index]].model[Normal.BELO.toInt()];
    }
    pub fn getNorthFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        if (index & 0x00f == 0) {
            // if (self.pos.z == 0) {
                return &BlockFace.NONE2;
            // }
            // else {
                // return &BLOCKS[world.chunks[@intCast(usize, (self.pos.x << 8) | (self.pos.y << 4) | ((self.pos.z - 1) << 0))].blocks[index | 0x00f]].model[Normal.NORT.toInt()];
            // }
        }
        return &BLOCKS[self.blocks[index - 0x001]].model[Normal.NORT.toInt()];
    }
    pub fn getEastFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        if (index & 0xf00 == 0) {
            // if (self.pos.x == 0) {
                return &BlockFace.NONE2;
            // }
            // else {
                // return &BLOCKS[world.chunks[@intCast(usize, ((self.pos.x - 1) << 8) | (self.pos.y << 4) | ((self.pos.z) << 0))].blocks[index | 0xf00]].model[Normal.EAST.toInt()];
            // }
        }
        return &BLOCKS[self.blocks[index - 0x100]].model[Normal.EAST.toInt()];
    }
    pub fn getAboveFace(self: *Chunk, index: usize, world: *World) *const BlockFace {
        _ = world;
        if (index & 0x0f0 == 0) {
            // if (self.pos.y == 0) {
                return &BlockFace.NONE2;
            // }
            // else {
                // return &BLOCKS[world.chunks[@intCast(usize, (self.pos.x << 8) | ((self.pos.y - 1) << 4) | ((self.pos.z) << 0))].blocks[index | 0x0f0]].model[Normal.ABOV.toInt()];
            // }
        }
        return &BLOCKS[self.blocks[index - 0x010]].model[Normal.ABOV.toInt()];
    }
    pub fn meshNorthSouth(self: *Chunk, buffer: *ByteBuffer, world: *World) void {
        var x: usize = 0; while (x < 16) : (x += 1) {
            var y: usize = 0; while (y < 16) : (y += 1) {
                var z: usize = 0; while (z < 16) : (z += 1) {
                    const index = (x << 8) | (y << 4) | (z << 0);
                    const face_s = self.getSouthFace(index, world);
                    const face_n = self.getNorthFace(index, world);
                    const compare = face_s.as_u32() - face_n.as_u32();
                    if (compare == 0x10101010) { continue; }
                    const offset = (x << 4) | (y << 12) | (z << 20);
                    
                    if (compare < 0x10101010) { buffer.put_u64(face_s.as_u64() - 0x10101010 + offset); }
                    if (compare > 0x10101010) { buffer.put_u64(face_n.as_u64()              + offset); }
                }
            }
        }
        print("{}\n", .{buffer.ind});
    }
    pub fn meshEastWest(self: *Chunk, buffer: *ByteBuffer, world: *World) void {
        var x: usize = 0; while (x < 16) : (x += 1) {
            var y: usize = 0; while (y < 16) : (y += 1) {
                var z: usize = 0; while (z < 16) : (z += 1) {
                    const index = (x << 8) | (y << 4) | (z << 0);
                    const face_w = self.getWestFace(index, world);
                    const face_e = self.getEastFace(index, world);
                    const compare = face_w.as_u32() - face_e.as_u32();
                    if (compare == 0x10101010) { continue; }
                    const offset = (x << 4) | (y << 12) | (z << 20);
                    
                    if (compare < 0x10101010) { buffer.put_u64(face_w.as_u64() - 0x10101010 + offset); }
                    if (compare > 0x10101010) { buffer.put_u64(face_e.as_u64()              + offset); }
                }
            }
        }
    }
    pub fn meshBelowAbove(self: *Chunk, buffer: *ByteBuffer, world: *World) void {
        var x: usize = 0; while (x < 16) : (x += 1) {
            var y: usize = 0; while (y < 16) : (y += 1) {
                var z: usize = 0; while (z < 16) : (z += 1) {
                    const index = (x << 8) | (y << 4) | (z << 0);
                    const face_b = self.getBelowFace(index, world);
                    const face_a = self.getAboveFace(index, world);
                    const compare = face_b.as_u32() - face_a.as_u32();
                    if (compare == 0x10101010) { continue; }
                    const offset = (x << 4) | (y << 12) | (z << 20);
                    
                    if (compare < 0x10101010) { buffer.put_u64(face_b.as_u64() - 0x10101010 + offset); }
                    if (compare > 0x10101010) { buffer.put_u64(face_a.as_u64()              + offset); }
                }
            }
        }
    }
};