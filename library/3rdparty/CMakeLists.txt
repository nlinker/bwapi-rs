cmake_minimum_required(VERSION 3.1)
project(external)
set(CMAKE_CXX_STANDARD 17)
include(CMakePrintHelpers)

file(REAL_PATH openbw OPENBW_DIR)
set(OPENBW_ENABLE_UI 1 CACHE BOOL "enable ui")

cmake_print_variables(OPENBW_DIR)
cmake_print_variables(OPENBW_ENABLE_UI)

add_subdirectory(bwapi)
