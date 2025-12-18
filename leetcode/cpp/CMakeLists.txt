cmake_minimum_required(VERSION 3.18.4)
project(challenges)

set(CMAKE_CXX_FLAGS "-g -pedantic -Wall -Wextra")
set(CMAKE_CXX_STANDARD 11)

find_package(Catch2 3 REQUIRED)
add_executable(unit_tests "almost_sorted.cpp") 
target_link_libraries(unit_tests PRIVATE Catch2::Catch2WithMain)

target_compile_features(unit_tests PRIVATE cxx_std_17)
