# The following five lines of boilerplate have to be in your project's
# CMakeLists in this exact order for cmake to work correctly
cmake_minimum_required(VERSION 3.5)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(houseflow)

target_add_binary_data(mqtt_ssl.elf "main/letsencrypt.pem" TEXT)