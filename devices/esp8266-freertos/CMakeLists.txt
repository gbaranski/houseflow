# The following five lines of boilerplate have to be in your project's
# CMakeLists in this exact order for cmake to work correctly
cmake_minimum_required(VERSION 3.5)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(houseflow)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

target_add_binary_data(mqtt_ssl.elf "components/hf/letsencrypt.pem" TEXT)