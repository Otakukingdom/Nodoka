
project(lmdb)

file(GLOB SOURCES
        ".c"
        ".h"
        )

add_library(lmdb SHARED ${SOURCES})