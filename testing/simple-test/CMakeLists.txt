cmake_minimum_required(VERSION 3.28)
project(FFIAPI_TESTING_SIMPLE_TEST)

include(CTest)

set(CMAKE_CXX_STANDARD 23)

add_executable(ffiapi-testing-simple-test
        simple-test.cpp
)

target_link_libraries(ffiapi-testing-simple-test
        PRIVATE ffiapi-testing-simple-imp
)

add_test(NAME ffiapi-testing-simple-test COMMAND ffiapi-testing-simple-test)