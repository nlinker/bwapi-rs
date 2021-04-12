#!/usr/bin/env bash

# just to reproduce errors

 "c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-arch" "x86_64" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-b18cd913387b3bcd/out/cxxbridge/include" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-b18cd913387b3bcd/out/cxxbridge/crate" "-I" "/Users/nick/rust/scai/bwapi-xi/library/bwapilib/include" "-I" "src" "-Wall" "-Wextra" "-std=c++17" "-Wno-unused-parameter" "-o" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-b18cd913387b3bcd/out/cxxbridge/sources/library/src/lib.rs.o" "-c" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-b18cd913387b3bcd/out/cxxbridge/sources/library/src/lib.rs.cc"
