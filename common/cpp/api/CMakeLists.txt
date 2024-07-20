cmake_minimum_required(VERSION 3.28)
project(FFIAPI_COMMON_CPP_API)

set(CMAKE_CXX_STANDARD 23)

add_library(ffiapi-common-cpp-api INTERFACE)

target_include_directories(ffiapi-common-cpp-api
        INTERFACE include
)

add_subdirectory(testing)
