pub const Block = struct {
    id: u8,
};

pub const BlockFace = packed struct {
    lef: u8,
    bot: u8,
    rig: u8,
    top: u8,
    dep: u8,
    nor: Normal,
    tex: u16,
    pub fn as_u64(self: *const BlockFace) u64 {
        return @ptrCast(*const u64, self).*;
    }
    pub fn as_u32(self: *const BlockFace) u32 {
        return @ptrCast(*const u32, self).*;
    }
    pub const NONE: BlockFace = BlockFace {
        .lef = 0x1f, .bot = 0x1f,
        .rig = 0x1f, .top = 0x1f,
        .dep = 0xff, .nor = Normal.NONE,
        .tex = 0xffff
    };
    pub const NONE2: BlockFace = BlockFace {
        .lef = 0x0f, .bot = 0x0f,
        .rig = 0x0f, .top = 0x0f,
        .dep = 0xff, .nor = Normal.NONE,
        .tex = 0xffff
    };
};

pub const BlockModel = [6] BlockFace;

pub const BlockState = struct {
    model: BlockModel,
    block: Block
};

pub const Normal = enum(u8) {
    SOUT = 0,
    WEST = 1,
    BELO = 2,
    NORT = 3,
    EAST = 4,
    ABOV = 5,
    NONE = 0xff,
    pub fn toInt(self: Normal) usize {
        return @enumToInt(self);
    }
};