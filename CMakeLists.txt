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
find_package(Qt5Sql)

if(APPLE)
    link_directories(${CMAKE_CURRENT_SOURCE_DIR}/libs/libvlc/mac)
elseif(WIN32)
    link_directories(${CMAKE_CURRENT_SOURCE_DIR}/libs/libvlc)
endif()

FILE(GLOB_RECURSE LibFiles "src/*.h")
add_custom_target(headers SOURCES ${LibFiles})

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

set(SOURCE_FILES src/main.cpp src/core/ConcretePlayer.cpp src/core/ConcretePlayer.h src/ui-element/MainWindow.cpp src/ui-element/MainWindow.h src/ui-element/SettingsForm.cpp src/ui-element/SettingsForm.h src/core/AudiobookScan.cpp src/model/Directory.cpp src/model/Directory.h src/core/DatabaseConnect.cpp src/core/DatabaseConnect.h src/core/NodokaApp.cpp src/core/NodokaApp.h src/event-handler/DirectoryHandler.cpp src/event-handler/DirectoryHandler.h)
add_executable(NodokaNative WIN32 ${SOURCE_FILES})

target_link_libraries(NodokaNative Qt5::Widgets Qt5::Sql vlc vlccore)
