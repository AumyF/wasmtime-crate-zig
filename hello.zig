const std = @import("std");

export fn hello() void {
    std.debug.print("Hello, zig from wasmtime embedded in rust!\n", .{});
}
