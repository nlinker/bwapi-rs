#!/usr/bin/env bash

# just to reproduce errors

#cargo build
#DYLD_LIBRARY_PATH=. cargo run

"cc" "-m64" "-arch" "x86_64" \
 "-L" "/Users/nick/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.10qqehs6lkia4xxl.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.11qbbsy36yt2ce9b.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.1l13uln2t7nuavq7.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.1nkvbvlwqiif62qd.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2nui47il806cgqsm.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2opmun5b1u2brbxh.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2p4sgecd63ewoejg.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2ty2dr8pdfji117.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2x5zxpqqw2ovwjhe.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.2yepovoznzdgh25d.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.31s0gypxjyglkxhd.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.325ga00ja1zc0j8n.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.34u48ywrn2ymbhge.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.395sx5jr9cbk49os.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.39y5wgin0gh6b17i.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.3ilwee0o1bpeg088.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.3jubuf03o19zf4d1.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.3vwwsnx64xf9vuyl.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.3xd1zync2rf22s9b.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.45iqcixqdnhk4qnc.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.4cq4xstiovo0x0rp.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.4cwwgse6852u1i8c.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.4metwhh0dbhzb8hg.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.4ms7qzj7aa9cro8g.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.52r0je69b7mtltnv.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.7wzmhzb4rc1rlxg.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.qhvv7g2l9bpgv6l.rcgu.o" \
 "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.rvdkujxfrhydk92.rcgu.o" \
 "-o" "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/liblibrary.dylib" \
 "-Wl,-exported_symbols_list,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/list"\
  "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.1wwdu5ua7qspoebe.rcgu.o" "/Users/nick/rust/scai/bwapi-xi/target/debug/deps/library.sxfbj4jqf9lngzk.rcgu.o" "-Wl,-dead_strip" "-dynamiclib" "-Wl,-dylib" "-nodefaultlibs" \
 "-L" "/Users/nick/rust/scai/bwapi-xi/target/debug/deps" \
 "-L" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-067e4ee2bee2c46d/out" \
 "-L" "/Users/nick/rust/scai/bwapi-xi/library/bwapilib/lib" \
 "-L" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/cxx-6afad30de4fd53d7/out" \
 "-L" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/link-cplusplus-005933d4173aacdd/out" \
 "-L" "/Users/nick/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-force_load" "-Wl,/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-067e4ee2bee2c46d/out/libbwapi_xi.a" "-lBWAPILIB" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libonce_cell-cb1e29d2f00c42bd.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libcxx-a05be82f344405e7.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/liblink_cplusplus-2867f35976fb4081.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libstd-33ab10cda0fffab0.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libpanic_unwind-81e8ce9846fe7b7d.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libobject-311cb8079e582cd6.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libaddr2line-85ea3ba7748c531f.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libgimli-7e96e48e793e07d6.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/librustc_demangle-482ecc6db18f3059.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libhashbrown-1095ddd9cbc41058.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/librustc_std_workspace_alloc-1877bb840e902e86.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libunwind-6df8cad01b702d59.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libcfg_if-204ee8a0055f58cc.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/liblibc-f4a2f2839ad5b23f.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/liballoc-96803f149c0979a0.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/librustc_std_workspace_core-465a16f2657ba500.rlib" "-Wl,-force_load" "-Wl,/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libcore-72787c1a9e4c6b7c.rlib" "/var/folders/7f/1bqg69sj2_x9nwbhpwswb9rm0000gn/T/rustcCmU4lV/libcompiler_builtins-929d677a8b7ad6f5.rlib" "-lc++" "-lSystem" "-lresolv" "-lc" "-lm" "-liconv"


