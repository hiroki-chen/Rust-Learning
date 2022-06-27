cmake_minimum_required(VERSION 3.20)
project(Rust_Learning)

set(CMAKE_CXX_STANDARD 14)

include_directories(src)

add_executable(Rust_Learning
        src/main.cc)

target_link_directories(Rust_Learning PRIVATE
        ${CMAKE_SOURCE_DIR}/target/debug)
target_link_libraries(Rust_Learning mylib)