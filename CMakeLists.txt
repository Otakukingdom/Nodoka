cmake_minimum_required(VERSION 3.6)
project(NodokaNative)

set(CMAKE_BINARY_DIR ${CMAKE_CURRENT_SOURCE_DIR}/build)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed.
set(CMAKE_AUTOMOC ON)

# auto generate uic
set(CMAKE_AUTOUIC ON)

set(CMAKE_CXX_STANDARD 14)

find_package(Qt5Widgets)

if(APPLE)
    link_directories(${CMAKE_CURRENT_SOURCE_DIR}/libs/libvlc/mac)
endif()

FILE(GLOB_RECURSE LibFiles "src/*.h")
add_custom_target(headers SOURCES ${LibFiles})

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

set(SOURCE_FILES src/main.cpp src/model/ConcretePlayer.cpp src/model/ConcretePlayer.h src/ui-element/MainWindow.cpp src/ui-element/MainWindow.h)
add_executable(NodokaNative WIN32 ${SOURCE_FILES})

target_link_libraries(NodokaNative Qt5::Widgets vlc vlccore)
