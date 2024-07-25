cmake_minimum_required(VERSION 3.28)
project(FFIAPI)

include(CTest)
include(GNUInstallDirs)

add_subdirectory(cpp-api)
add_subdirectory(testing EXCLUDE_FROM_ALL)

install(TARGETS ffiapi-cpp-api
        FILE_SET HEADERS DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
)