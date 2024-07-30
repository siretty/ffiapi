cmake_minimum_required(VERSION 3.28)
project(FFIAPI_TESTING_SIMPLE_API)

set(CMAKE_CXX_STANDARD 23)

add_library(ffiapi-testing-simple-api STATIC
        simple/thing.cpp
)

target_sources(ffiapi-testing-simple-api
        PUBLIC FILE_SET HEADERS FILES
        simple/ffi.hpp
        simple/thing.hpp
)

target_include_directories(ffiapi-testing-simple-api
        PUBLIC "."
)

target_link_libraries(ffiapi-testing-simple-api
        PUBLIC ffiapi-cpp-api
)