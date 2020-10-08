cmake_minimum_required(VERSION 3.13.4)

set(CMAKE_CXX_FLAGS "-pedantic -Wall -Wextra")

set(name leetcode-cpp)

set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_STANDARD 11)
enable_testing()
add_subdirectory(googletest)

project(${name})

add_executable(${name} maximum-swap.cpp)
target_link_libraries(${name} PUBLIC
    gtest_main
)

add_test(
    NAME ${name}
    COMMAND ${name}
)
