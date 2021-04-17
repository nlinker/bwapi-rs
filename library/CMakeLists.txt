cmake_minimum_required(VERSION 3.19)
project(bwapi_xi_lib)
set(CMAKE_CXX_STANDARD 17)

set(OPENBW_DIR openbw/openbw-ui)

include_directories(openbw/bwapilib/include)

include(CMakePrintHelpers)
cmake_print_variables(CMAKE_CURRENT_SOURCE_DIR)

#target_include_directories(${SRC_INCLUDES})
#target_include_directories(bwapi_xi_lib openbw/bwapi/Source)
#target_include_directories(bwapi_xi_lib openbw/bwapicore)
#target_include_directories(bwapi_xi_lib openbw/openbwdata)
#target_include_directories(bwapi_xi_lib openbw/util/Source)
#target_include_directories(bwapi_xi_lib target/cxxbridge)

add_subdirectory(openbw/bwapi)
add_subdirectory(openbw/bwapicore)
add_subdirectory(openbw/bwapilib)
add_subdirectory(openbw/openbwdata)

add_library(bwapi_xi_lib PUBLIC src)
target_sources(bwapi_xi_lib PUBLIC src)
SET_TARGET_PROPERTIES(bwapi_xi_lib PROPERTIES LINKER_LANGUAGE C++)

#file(GLOB_RECURSE SRC_FILES
#        ${PROJECT_SOURCE_DIR}/src *.cc
#        ${PROJECT_SOURCE_DIR}/src *.cpp
#)
