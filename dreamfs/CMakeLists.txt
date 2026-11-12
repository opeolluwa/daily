cmake_minimum_required(VERSION 4.3)
project(dreamfs LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(GNUInstallDirs)

add_library(dreamfs SHARED library.cpp)
target_include_directories(dreamfs
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(TARGETS dreamfs
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
install(FILES library.h DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})

option(DREAMFS_BUILD_TESTS "Build the C++ smoke test" ON)
if(DREAMFS_BUILD_TESTS)
    add_executable(dreamfs_cpp_test tests/cpp/main.cc)
    target_link_libraries(dreamfs_cpp_test PRIVATE dreamfs)
endif()
