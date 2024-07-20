cmake_minimum_required(VERSION 3.28)
project(FFIAPI_COMMON_CPP_API_TESTING)

include(CTest)

add_executable(ffiapi-common-cpp-api-testing main.cpp)

target_link_libraries(ffiapi-common-cpp-api-testing
        PRIVATE ffiapi-common-cpp-api
)

add_test(NAME ffiapi-common-cpp-api COMMAND ffiapi-common-cpp-api-testing)
