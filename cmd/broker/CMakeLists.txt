cmake_minimum_required(VERSION 3.9) 
project(mosquitto_houseflow)

add_library(${PROJECT_NAME} SHARED ${PROJECT_NAME}.c)

set(MOSQUITTO_DIR "/Users/gbaranski/mosquitto-2.0.6")
set(LIBPQ_DIR "/usr/local/opt/libpq")

target_include_directories(${PROJECT_NAME} 
  PRIVATE
    ${MOSQUITTO_DIR}
    ${MOSQUITTO_DIR}/include
    ${LIBPQ_DIR}/include
)

add_compile_options(-Wall -Wextra -pedantic -Werror -fPIC -shared)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
