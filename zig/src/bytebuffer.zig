const std = @import("std");
const print = @import("std").debug.print;

pub const ByteBuffer = struct {
    arr: [262144]u8,
    ind: usize,
    pub fn put_u8(self: *ByteBuffer, val: u8) void {
        self.arr[self.ind] = val;
        self.ind += 1;
    }
    pub fn put_u64(self: *ByteBuffer, val: u64) void {
        var ptr = @intToPtr(*u64, @ptrToInt(@ptrCast(*u64, @alignCast(8, &self.arr))) + self.ind);
        ptr.* = val;
        self.ind += 8;
    }
    pub fn new() *ByteBuffer {
        var byteBuffer: *ByteBuffer = std.heap.page_allocator.create(ByteBuffer) catch unreachable;
        byteBuffer.ind = 0;
        return byteBuffer;
    }
    //   0f0   00f
    //   ---------
    // 0 ure ~ lef
    // 1 ven ~ bot
    // 2 dep ~ rig
    // 3 wid ~ top
    // 4 hei ~ dep
    // 5 nor ~ nor
    // 6 tex ~ tex
    // 7 tex ~ tex
    // 
    // 
    // Converts a quad from left-bottom-right-top format to u-v-d-w-h format

    pub fn formatQuads(self: *ByteBuffer) void {
        var i: usize = 0; while (i < self.ind) : (i += 8) {
            var face = self.arr[i..i+8];
         // var ure = (arr[0]);
         // var ven = (arr[1]);
            var dep = (face[2] & 0xf0) | (0x0 + (face[4] & 0x0f));
            var nor = (face[5]);
            var wid = (face[3] & 0xf0) | (0xf - (face[2] & 0x0f) - (face[0] & 0x0f));
            var hei = (face[4] & 0xf0) | (0xf - (face[3] & 0x0f) - (face[1] & 0x0f));
         // var tex = (arr[6], arr[7]);
         // arr[0] = ure;
         // arr[1] = ven;
            face[2] = dep;
            face[3] = nor;
            face[4] = wid;
            face[5] = hei;
            // arr[6] = tex.0;
            // arr[7] = tex.1;
            print("{d}\n", .{ face });
        }
    }
};