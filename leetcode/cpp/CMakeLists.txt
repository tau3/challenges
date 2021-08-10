cmake_minimum_required(VERSION 3.18.4)

set(CMAKE_CXX_FLAGS "-g -pedantic -Wall -Wextra")
set(CMAKE_CXX_STANDARD 11)

project (leetcode)
file(GLOB leetcode_src
    # "*.h"
    # "*.cpp"
    "distance_between_bus_stops.cpp"
)

add_executable(${PROJECT_NAME} ${leetcode_src})

# unit tests
find_package(Catch2 CONFIG REQUIRED)
add_executable(unit_tests "distance_between_bus_stops.cpp") 
target_link_libraries(unit_tests PRIVATE Catch2::Catch2)
include(Catch)
catch_discover_tests(unit_tests)
