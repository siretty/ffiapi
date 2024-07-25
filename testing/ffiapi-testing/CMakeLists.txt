cmake_minimum_required(VERSION 3.28)
project(FFIAPI_TESTING_FFIAPI_TESTING)

set(CMAKE_CXX_STANDARD 23)

add_library(ffiapi-testing INTERFACE)

target_sources(ffiapi-testing
        INTERFACE FILE_SET HEADERS BASE_DIRS ffiapi-testing FILES
        ffiapi-testing.hpp
)

target_include_directories(ffiapi-testing
        INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}"
)