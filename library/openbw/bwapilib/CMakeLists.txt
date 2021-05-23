#cmake_minimum_required(VERSION 3.14)
#project(BWAPILIB)

include_directories(../include)

file(GLOB_RECURSE SRC_FILES ${PROJECT_SOURCE_DIR}/source *.cpp)

#add_library(BWAPILIB ${SRC_FILES})
add_library(BWAPILIB STATIC ${SRC_FILES})

target_include_directories(BWAPILIB SYSTEM PUBLIC ${PROJECT_SOURCE_DIR}/include)
target_compile_options(BWAPILIB PRIVATE -w)

# Here is not-so-standard element.
# The target installation command has to exist
# and has to install all linkable targets
# into `.` (current) directory for cargo to find it.
install(TARGETS BWAPILIB DESTINATION .)
