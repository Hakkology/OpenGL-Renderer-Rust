cmake_minimum_required(VERSION 3.10)
project(opengl_renderer)

# Rust projesini CMake üzerinden derlemek için temel yapı.
# Genelde Rust projeleri Cargo ile derlenir ancak IDE desteği
# veya C++ entegrasyonu için bu dosya gerekebilir.

if (NOT WIN32)
    set(CMAKE_C_STANDARD 11)
    set(CMAKE_CXX_STANDARD 17)
endif()

# Dummy target
add_custom_target(cargo_build ALL
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Rust project with Cargo"
)
