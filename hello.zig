const std = @import("std");
const cbor = @import("cbor");

export fn hello() void {
    std.debug.print("Hello, zig from wasmtime embedded in rust!\n", .{});
}

export fn test_cbor() u64 {
    const allocator = std.heap.wasm_allocator;
    var str = std.ArrayList(u8).init(allocator);
    // defer str.deinit();

    cbor.stringify(.{ .name = "John Doe" }, .{}, str.writer()) catch {
        unreachable;
    };

    return string_to_ptr(str.items);
}

fn string_to_ptr(s: []const u8) u64 {
    const ptr: u64 = @intFromPtr(s.ptr);
    return ptr << 32 | s.len;
}
