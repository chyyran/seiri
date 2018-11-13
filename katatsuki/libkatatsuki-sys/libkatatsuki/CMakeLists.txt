cmake_minimum_required(VERSION 3.12 FATAL_ERROR)
project(katatsuki)

set(CMAKE_CXX_STANDARD 17)
set(BUILD_SHARED_LIBS OFF)
set(CMAKE_OBJECT_PATH_MAX 128)

add_definitions("-DTAGLIB_C_EXPORT -DTAGLIB_STATIC -DENABLE_STATIC_RUNTIME=ON")

#
# a relatively stable snapshot of taglib2 branch for our purposes.
#
set(REVISION "6048cdef3ef2143c2118d75979995d0bd75c1c74")

#
# import ExternalProject module
#
include(ExternalProject)

#
# compiler options
#
set(CMAKE_VERBOSE_MAKEFILE OFF)
set(SUPPORT_LARGE_FILES ON)
if(UNIX AND NOT APPLE)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC -std=c++11")
elseif(MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W3 /MT")
else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -stdlib=libc++ -mmacosx-version-min=10.10")
endif()

set(CMAKE_INSTALL_PREFIX ${CMAKE_BINARY_DIR}/taglib)
#
# download and make TagLib
#
ExternalProject_Add(
        taglib
        PREFIX "${CMAKE_BINARY_DIR}/taglib"
        GIT_REPOSITORY https://github.com/taglib/taglib
        GIT_TAG ${REVISION}
        INSTALL_DIR "${CMAKE_BINARY_DIR}/taglib"
        CMAKE_ARGS "-DCMAKE_INSTALL_PREFIX=${CMAKE_BINARY_DIR}/taglib" "-DCMAKE_BUILD_TYPE=RELEASE" "-DTAGLIB_C_EXPORT=" "-DTAGLIB_STATIC=" "-DBUILD_SHARED_LIBS=OFF" "-DENABLE_STATIC_RUNTIME=ON"
        CMAKE_CACHE_ARGS "-DCMAKE_POSITION_INDEPENDENT_CODE:BOOL=true" "-DBUILD_SHARED_LIBS=OFF"
        )

#
# force rebuild of TagLib source (useful when switching between node/electron)
#
ExternalProject_Add_Step(
        taglib
        forcebuild
        COMMAND ${CMAKE_COMMAND} -E echo_append ""
        COMMENT "Forcing build step for taglib"
        DEPENDEES configure
        DEPENDERS build
        ALWAYS 1
)

include_directories(. ${CMAKE_BINARY_DIR}/taglib/include/taglib)
link_directories(${CMAKE_BINARY_DIR}/taglib/lib)

add_library(katatsuki
        STATIC
        track_file_type.h
        TrackData.cpp
        TrackData.h StringUtils.h track_data.h track_data.cpp)

add_dependencies(katatsuki taglib)
target_link_libraries(katatsuki tag -static)