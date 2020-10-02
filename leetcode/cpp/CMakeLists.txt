cmake_minimum_required(VERSION 3.13.4)

set(CMAKE_CXX_FLAGS "-pedantic -Wall -Wextra -O2")

set(name leetcode-cpp)

set(CMAKE_CXX_STANDARD 11)
enable_testing()
add_subdirectory(googletest)

project(${name})

add_executable(${name} detect-pattern-of-length-m-repeated-k-or-more-times.cpp)
target_link_libraries(${name} PUBLIC
    gtest_main
)

add_test(
    NAME ${name}
    COMMAND ${name}
)
