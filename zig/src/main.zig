const std = @import("std");
const Block = @import("block.zig").Block;
const BlockState = @import("block.zig").BlockState;
const BlockFace = @import("block.zig").BlockFace;
const Chunk = @import("chunk.zig").Chunk;
const World = @import("world.zig").World;
const ByteBuffer = @import("bytebuffer.zig").ByteBuffer;
const GlShader = @import("gl.zig").GlShader;
const GlBuffer = @import("gl.zig").GlBuffer;
const GlProgram = @import("gl.zig").GlProgram;
const Normal = @import("block.zig").Normal;
const Camera = @import("camera.zig").Camera;
const c = @cImport({
    @cInclude("glad/gl.h");
    @cInclude("GLFW/glfw3.h");
});

pub const print = std.debug.print;

pub const BLOCKS = [_]BlockState {
    BlockState {
        .block = Block { .id = 0 },
        .model = [6]BlockFace {
            BlockFace.NONE, BlockFace.NONE, BlockFace.NONE, BlockFace.NONE2, BlockFace.NONE2, BlockFace.NONE2
        }
    },
    BlockState {
        .block = Block { .id = 1 },
        .model = [6]BlockFace {
            BlockFace {
                .lef = 0x10, .rig = 0x10,
                .top = 0x10, .bot = 0x10,
                .dep = 0x00, .nor = Normal.SOUT,
                .tex = 0x0000
            },
            BlockFace {
                .lef = 0x10, .rig = 0x10,
                .top = 0x10, .bot = 0x10,
                .dep = 0x00, .nor = Normal.WEST,
                .tex = 0x0000
            },
            BlockFace {
                .lef = 0x10, .rig = 0x10,
                .top = 0x10, .bot = 0x10,
                .dep = 0x00, .nor = Normal.BELO,
                .tex = 0x0000
            },
            BlockFace {
                .lef = 0x00, .rig = 0x00,
                .top = 0x00, .bot = 0x00,
                .dep = 0x00, .nor = Normal.NORT,
                .tex = 0x0000
            },
            BlockFace {
                .lef = 0x00, .rig = 0x00,
                .top = 0x00, .bot = 0x00,
                .dep = 0x00, .nor = Normal.EAST,
                .tex = 0x0000
            },
            BlockFace {
                .lef = 0x00, .rig = 0x00,
                .top = 0x00, .bot = 0x00,
                .dep = 0x00, .nor = Normal.ABOV,
                .tex = 0x0000
            },
        }
    }
};


var windowStatus: struct { debugLines: bool, maximized: bool, width: i32, height: i32, prevMouseX: f64, prevMouseY: f64, keys: std.AutoHashMap(c_int, bool) } = .{
    .debugLines = false,
    .maximized = false,
    .width = 800.0,
    .height = 800.0,
    .prevMouseX = 0,
    .prevMouseY = 0,
    .keys = std.AutoHashMap(c_int, bool).init(std.heap.page_allocator)
};

pub fn keyCallback(window: ?*c.struct_GLFWwindow, key: c_int, scancode: c_int, action: c_int, mods: c_int) callconv(.C) void {
    _ = mods;
    _ = scancode;
    windowStatus.keys.put(key, action == c.GLFW_PRESS or action == c.GLFW_REPEAT) catch print("fuck", .{});
    
    if (key == c.GLFW_KEY_X and action == c.GLFW_PRESS) {
        c.glPolygonMode(c.GL_FRONT_AND_BACK, if (windowStatus.debugLines) c.GL_FILL else c.GL_LINE);
        windowStatus.debugLines = !windowStatus.debugLines;
    }
    if (key == c.GLFW_KEY_TAB and action == c.GLFW_PRESS) {
        if (windowStatus.maximized) { c.glfwRestoreWindow(window); } else { c.glfwMaximizeWindow(window); }
        windowStatus.maximized = !windowStatus.maximized;
    }
}

pub fn mouseButtonCallback(window: ?*c.struct_GLFWwindow, button: c_int, action: c_int, mods: c_int) callconv(.C) void {
    _ = mods;
    _ = action;
    _ = window;
    if (button == c.GLFW_MOUSE_BUTTON_LEFT) {

    }
}

pub fn windowSizeCallback(window: ?*c.struct_GLFWwindow, width: c_int, height: c_int) callconv(.C) void {
    _ = window;
    windowStatus.width = width;
    windowStatus.height = height;
    c.glViewport(0, 0, width, height);
}

pub fn framebufferSizeCallback(window: ?*c.struct_GLFWwindow, width: c_int, height: c_int) callconv(.C) void {
    _ = window;
    c.glViewport(0, 0, width, height);
}

pub fn main() void {
    print("\n", .{});
    // Initialize GLFW
    if (c.glfwInit() == 0) { print("Failed to initialized GLFW\n", .{}); }
    // Create window
    var window = c.glfwCreateWindow(windowStatus.width, windowStatus.height, "", null, null);
    c.glfwMakeContextCurrent(window);
    
    // Initialize OpenGL
    if (c.gladLoadGL(c.glfwGetProcAddress) == 0) { print("Failed to initialize OpenGL\n", .{}); }

    // Set up and bind shader program
    // Set up ands bind index array
    setupOpenGl();

    // Create a new world and generate terrain
    var world = World.new();

    // Generate optimized mesh for the world
    world.meshAllChunks();

    // Set up callback for keypresses and releases
    _ = c.glfwSetKeyCallback(window, keyCallback);
    _ = c.glfwSetMouseButtonCallback(window, mouseButtonCallback);
    _ = c.glfwSetWindowSizeCallback(window, windowSizeCallback);
    _ = c.glfwSetFramebufferSizeCallback(window, framebufferSizeCallback);

    c.glfwSetInputMode(window, c.GLFW_CURSOR, c.GLFW_CURSOR_DISABLED);
    
    // Main render loop

    while (c.glfwWindowShouldClose(window) == 0) {
        c.glfwPollEvents();
        
        var mouseX: f64 = undefined;
        var mouseY: f64 = undefined;
        c.glfwGetCursorPos(window, &mouseX, &mouseY);
        world.camera.ratio = @intToFloat(f32, windowStatus.width) / @intToFloat(f32, windowStatus.height);
        world.camera.move(windowStatus.keys, .{ .x = windowStatus.prevMouseX - mouseX, .y = windowStatus.prevMouseY - mouseY });
        windowStatus.prevMouseX = mouseX;
        windowStatus.prevMouseY = mouseY;

        c.glClear(c.GL_COLOR_BUFFER_BIT | c.GL_DEPTH_BUFFER_BIT);

        var transform = world.camera.getMatrix();
        c.glUniformMatrix4fv(0, 1, c.GL_FALSE, @ptrCast([*c]const f32, &transform));

        world.chunks[0].buffer.bindBufferBase(c.GL_SHADER_STORAGE_BUFFER);
        c.glDrawElements(c.GL_TRIANGLE_STRIP, world.chunks[0].faces * 5, c.GL_UNSIGNED_INT, null);

        c.glfwSwapBuffers(window);
        printGlError();
    }
}



pub fn printGlError() void {
    var glError: c_uint = c.glGetError();
    if (glError != 0) print("{}", .{c.glGetError()});
}


pub fn setupOpenGl() void {
    var vertexShader = GlShader.create(c.GL_VERTEX_SHADER, @ptrCast(*const u8, @embedFile("shader/shader.glsl.vert")));
    var fragmentShader = GlShader.create(c.GL_FRAGMENT_SHADER, @ptrCast(*const u8, @embedFile("shader/shader.glsl.frag")));
    var shaderProg = GlProgram.create(vertexShader, fragmentShader);
    shaderProg.useProgram();
    c.glClearColor(1.0, 1.0, 1.0, 1.0);

    c.glEnable(c.GL_DEPTH_TEST);
    c.glEnable(c.GL_PRIMITIVE_RESTART);
    c.glPrimitiveRestartIndex(0xffffffff);
    var indexArray: [1024 * 1024 / 4]u32 = undefined;

    var j: u32 = 0;
    var i: u32 = 0;
    while (i < 1024 * 1024 / 4) : (i += 1) {
        if (i % 5 == 4) {
            indexArray[i] = 0xffffffff;
        }
        else {
            indexArray[i] = j;
            j += 1;
        }
    }

    var indexBuffer = GlBuffer.create();

    indexBuffer.bindBuffer(c.GL_ELEMENT_ARRAY_BUFFER);
    indexBuffer.bufferStorage(1024 * 1024 / 4, c.GL_DYNAMIC_STORAGE_BIT);
    indexBuffer.bufferSubData(0, 1024 * 1024 / 4, &indexArray);
    printGlError();
}