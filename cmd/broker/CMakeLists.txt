cmake_minimum_required(VERSION 3.9) 
project(mosquitto_houseflow)

add_library(${PROJECT_NAME} SHARED ${PROJECT_NAME}.c)

set(MOSQUITTO_DIR "/home/gbaranski/Downloads/mosquitto-2.0.7")
set(LIBPQ_DIR "/usr/include/libpq")
set(LIBSODIUM_DIR "/usr/include/libsodium")

target_include_directories(${PROJECT_NAME} 
  PRIVATE
    ${MOSQUITTO_DIR}/include
    ${LIBPQ_DIR}
    ${LIBSODIUM_DIR}
)

add_compile_options(-Wall -Wextra -pedantic -Werror -fPIC -shared)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
