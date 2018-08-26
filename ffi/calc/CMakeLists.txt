cmake_minimum_required(VERSION 3.5)

project(calc)

add_library(calc STATIC calc.c)

target_include_directories(calc PUBLIC ${PROJECT_SOURCE_DIR})

