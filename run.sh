#!/usr/bin/env bash

cargo build
DYLD_LIBRARY_PATH=library/bwapi/lib target/debug/launcher
