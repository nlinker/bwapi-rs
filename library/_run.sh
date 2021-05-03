#!/usr/bin/env bash

# just to reproduce errors

#cargo build
#DYLD_LIBRARY_PATH=. cargo run

"c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-arch" "x86_64" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/include" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/crate" "-I" "/Users/nick/rust/scai/bwapi-xi/library/openbw/bwapilib/include" "-I" "src" "-Wall" "-Wextra" "-std=c++17" "-Wno-unused-parameter" "-o" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/sources/library/src/lib.rs.o" "-c" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/sources/library/src/lib.rs.cc"
#"c++" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m64" "-arch" "x86_64" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/include" "-I" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/cxxbridge/crate" "-I" "/Users/nick/rust/scai/bwapi-xi/library/openbw/bwapilib/include" "-I" "src" "-Wall" "-Wextra" "-std=c++17" "-Wno-unused-parameter" "-o" "/Users/nick/rust/scai/bwapi-xi/target/debug/build/library-10a440775d1794e8/out/src/lib.o" "-c" "src/lib.cc"