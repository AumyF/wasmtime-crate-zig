const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const zbor_dep = b.dependency("zbor", .{
        .target = target,
        .optimize = optimize,
    });
    const zbor_module = zbor_dep.module("zbor");

    const wasm = b.addSharedLibrary(.{
        .name = "hello",
        .root_source_file = .{ .path = "hello.zig" },
        .target = .{ .cpu_arch = .wasm32, .os_tag = .wasi, .abi = .musl },
        .optimize = optimize,
    });
    wasm.rdynamic = true;
    wasm.addModule("cbor", zbor_module);

    b.installArtifact(wasm);
}
