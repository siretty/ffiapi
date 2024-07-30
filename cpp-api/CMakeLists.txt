cmake_minimum_required(VERSION 3.28)
project(FFIAPI_CPP_API)

set(CMAKE_CXX_STANDARD 23)

include(CTest)


add_library(ffiapi-cpp-api INTERFACE)

target_include_directories(ffiapi-cpp-api
        INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}"
)

target_sources(ffiapi-cpp-api
        INTERFACE FILE_SET HEADERS FILES ffiapi/v1/obj.hpp
        INTERFACE FILE_SET HEADERS FILES ffiapi/v1/obj.ipp
        INTERFACE FILE_SET HEADERS FILES ffiapi/v1.hpp
)


add_library(ffiapi-cpp-api-compiled-source OBJECT ffiapi/v1.cpp)

target_link_libraries(ffiapi-cpp-api-compiled-source
        PUBLIC ffiapi-cpp-api
)

target_compile_definitions(ffiapi-cpp-api-compiled-source
        PUBLIC FFIAPI_V1_COMPILED_SOURCE
)


if (BUILD_TESTING)
    add_executable(ffiapi-api-cpp-unit-tests
            ffiapi/test-main.cpp
            ffiapi/v1/obj.test.cpp
    )

    target_link_libraries(ffiapi-api-cpp-unit-tests
            PRIVATE ffiapi-cpp-api-compiled-source
            PRIVATE ffiapi-testing
    )

    add_test(NAME ffiapi-api-cpp-unit-tests COMMAND ffiapi-api-cpp-unit-tests)
endif ()