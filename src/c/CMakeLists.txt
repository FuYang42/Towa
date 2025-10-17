cmake_minimum_required(VERSION 3.10)
project(Towa_C_Tools C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# 环境检测工具
add_executable(env_checker env_checker.c)

# 设置输出目录
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Windows 特定设置
if(WIN32)
    target_compile_definitions(env_checker PRIVATE _CRT_SECURE_NO_WARNINGS)
endif()

# 安装规则
install(TARGETS env_checker DESTINATION bin)
