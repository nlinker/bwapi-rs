cmake_minimum_required(VERSION 3.17)
project(bwapi_rsx_lib)
set(CMAKE_CXX_STANDARD 17)

include_directories(openbw/bwapilib/include)
include_directories(target/cxxbridge)
include_directories(src)

file(GLOB_RECURSE SRC_FILES ${PROJECT_SOURCE_DIR}/src *.cpp)

add_library(bwapi_rsx_lib SHARED ${SRC_FILES})
target_include_directories(bwapi_rsx_lib PUBLIC ${SRC_INCLUDES})
