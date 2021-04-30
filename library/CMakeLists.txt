cmake_minimum_required(VERSION 3.1)
project(bwapi_xi_test)
set(CMAKE_CXX_STANDARD 17)
include(CMakePrintHelpers)

set(OPENBW_ENABLE_UI OFF)
set(OPENBWAPI_BUILD_STATIC_LIBS ON)
set(OPENBW_DIR openbw/openbw-ui)

include_directories(openbw/include)

add_executable(bwapi_xi_test tests)
target_include_directories(bwapi_xi_test PRIVATE
        tests
        openbw/openbwdata/BW
        )

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -c")
set_target_properties(bwapi_xi_test PROPERTIES LINKER_LANGUAGE CXX)

add_subdirectory(openbw/bwapi)
add_subdirectory(openbw/bwapilib)
add_subdirectory(openbw/bwapicore)
add_subdirectory(openbw/openbwdata)

target_link_libraries(bwapi_xi_test BWAPI BWAPILIB BWData OpenBWData)

cmake_print_variables(OPENBW_DIR)
cmake_print_variables(OPENBW_ENABLE_UI)
cmake_print_variables(OPENBWAPI_BUILD_STATIC_LIBS)
cmake_print_variables(CMAKE_CURRENT_SOURCE_DIR)
cmake_print_variables(PROJECT_SOURCE_DIR)

#target_include_directories(${SRC_INCLUDES})

#add_subdirectory(openbw/bwapilib)


#file(GLOB_RECURSE SRC_FILES
#        ${PROJECT_SOURCE_DIR}/src *.cc
#        ${PROJECT_SOURCE_DIR}/src *.cpp
#)
#SET_TARGET_PROPERTIES(bwapi_xi_test PROPERTIES LINKER_LANGUAGE C++)
