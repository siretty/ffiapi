cmake_minimum_required(VERSION 3.28)
project(FFIAPI_TESTING_SIMPLE_IMP)

set(_simple_imp_library_file libsimple_imp.so)
set(_cargo_target_directory "${CMAKE_CURRENT_SOURCE_DIR}/../../target")
set(_simple_imp_debug_library "${_cargo_target_directory}/debug/${_simple_imp_library_file}")
set(_simple_imp_release_library "${_cargo_target_directory}/release/${_simple_imp_library_file}")

add_library(ffiapi-testing-simple-imp SHARED IMPORTED GLOBAL)

set_target_properties(ffiapi-testing-simple-imp PROPERTIES
        IMPORTED_LOCATION_DEBUG "${_simple_imp_debug_library}"
        IMPORTED_LOCATION_RELEASE "${_simple_imp_release_library}"
)

target_link_libraries(ffiapi-testing-simple-imp
        INTERFACE ffiapi-testing-simple-api
)

add_dependencies(ffiapi-testing-simple-imp
        ffiapi-testing-simple-imp-cargo
)

add_custom_target(ffiapi-testing-simple-imp-cargo
        COMMAND cargo build --quiet --package simple-imp --lib
        COMMAND cargo build --quiet --package simple-imp --lib --release
        BYPRODUCTS
        "${_simple_imp_debug_library}"
        "${_simple_imp_release_library}"
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/.."
        COMMENT "Building simple-imp with cargo"
        SOURCES
        Cargo.toml
        src/lib.rs
)