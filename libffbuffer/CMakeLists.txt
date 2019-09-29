cmake_minimum_required(VERSION 3.0)
project(libffbuffer CXX)

add_library(ffbuffer STATIC ff_ubuffer.cpp)

find_path(FF_INCLUDE NAMES ff/ubuffer.hpp HINTS $ENV{HOME}/fastflow)
message("FF_INCLUDE include dir = ${FF_INCLUDE}")

if(NOT FF_INCLUDE)
  message(FATAL_ERROR "FF library not found")
endif()

target_include_directories(ffbuffer PUBLIC ${FF_INCLUDE})

install(TARGETS ffbuffer DESTINATION .)
