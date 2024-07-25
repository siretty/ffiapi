cmake_minimum_required(VERSION 3.28)
project(FFIAPI_CPP_API)

set(CMAKE_CXX_STANDARD 23)

include(CTest)

add_library(ffiapi-cpp-api INTERFACE)

target_include_directories(ffiapi-cpp-api
        INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}"
)

target_sources(ffiapi-cpp-api
        INTERFACE FILE_SET HEADERS FILES ffiapi/v1/ptr.hpp
)

if (BUILD_TESTING)
    add_executable(ffiapi-api-cpp-unit-tests
            ffiapi/v1/ptr.test.cpp
    )

    target_link_libraries(ffiapi-api-cpp-unit-tests
            PRIVATE ffiapi-cpp-api
            PRIVATE ffiapi-testing
    )

    add_test(NAME ffiapi-api-cpp-unit-tests COMMAND ffiapi-api-cpp-unit-tests)
endif ()