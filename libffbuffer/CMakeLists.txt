cmake_minimum_required(VERSION 3.0)
project(libffbuffer)

set(CMAKE_BUILD_TYPE "Release")
message("Build type=${CMAKE_BUILD_TYPE}")
message("Build cxx flags=${CMAKE_CXX_FLAGS}")

add_library(ffbuffer STATIC ff_ubuffer.cpp)

find_path(FF_INCLUDE NAMES ff/ubuffer.hpp HINTS $ENV{HOME}/fastflow)
message("FF_INCLUDE=${FF_INCLUDE}")

if(NOT FF_INCLUDE)
  message(FATAL_ERROR "FF library not found")
endif()

target_include_directories(ffbuffer PUBLIC ${FF_INCLUDE})

# Benches
# add_executable(producer_consumer benches/producer_consumer.cpp)
# target_include_directories(producer_consumer PUBLIC .)
# target_link_libraries(producer_consumer ffbuffer pthread)

install(TARGETS ffbuffer DESTINATION .)
