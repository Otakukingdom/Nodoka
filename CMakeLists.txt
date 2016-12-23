cmake_minimum_required(VERSION 3.6)
project(NodokaNative)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed.
set(CMAKE_AUTOMOC ON)

set(CMAKE_CXX_STANDARD 14)

find_package(Qt5Widgets)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
link_directories(${CMAKE_CURRENT_SOURCE_DIR}/libs/libvlc)

set(SOURCE_FILES src/main.cpp)
add_executable(NodokaNative WIN32 ${SOURCE_FILES})

target_link_libraries(NodokaNative Qt5::Widgets vlc vlccore)