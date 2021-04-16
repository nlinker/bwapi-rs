cmake_minimum_required(VERSION 3.17)
project(bwapi_rsx_lib)
set(CMAKE_CXX_STANDARD 17)

message(FATAL_ERROR "To be done yet, wip")

set(OPENBW_DIR openbw/openbw-ui)

include_directories(${SRC_INCLUDES})
include_directories(openbw/bwapilib/include)
include_directories(openbw/bwapi/Source)
include_directories(openbw/bwapicore)
include_directories(openbw/openbwdata)
include_directories(openbw/util/Source)
include_directories(target/cxxbridge)

add_subdirectory(openbw/bwapi)
add_subdirectory(openbw/bwapicore)
add_subdirectory(openbw/bwapilib)
add_subdirectory(openbw/openbwdata)

add_library(bwapi_xi_lib SHARED ${SRC_FILES})
SET_TARGET_PROPERTIES(bwapi_xi_lib PROPERTIES LINKER_LANGUAGE C++)

file(GLOB_RECURSE SRC_FILES
        ${PROJECT_SOURCE_DIR}/src *.cc
        ${PROJECT_SOURCE_DIR}/src *.cpp
)
#target_sources(bwapi_xi_lib PUBLIC src)
