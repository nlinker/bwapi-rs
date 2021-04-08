#!/usr/bin/env bash

# just to reproduce errors

"c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-arch" "x86_64" \
 "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-1f1f49df99b34bc4/out/cxxbridge/include" \
 "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-1f1f49df99b34bc4/out/cxxbridge/crate" \
 "-I" "/Users/nick/rust/scai/bwapi-xi/library/bwapilib/include" \
 "-I" "src" "-Wall" "-Wextra" "" "-std=c++17" \
 "-o" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-1f1f49df99b34bc4/out/cxxbridge/sources/library/src/lib.rs.o" \
 "-c" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-1f1f49df99b34bc4/out/cxxbridge/sources/library/src/lib.rs.cc"