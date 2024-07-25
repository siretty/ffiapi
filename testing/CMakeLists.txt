cmake_minimum_required(VERSION 3.28)
project(FFIAPI_TESTING)

set(CMAKE_CXX_STANDARD 23)

add_subdirectory(ffiapi-testing)
add_subdirectory(simple-api)
add_subdirectory(simple-imp)
add_subdirectory(simple-test)