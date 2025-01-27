# Apps
```text
cmake_minimum_required(VERSION 3.23) 
project(<project name> VERSION 0.1 LANGUAGES C/CXX)

add_executable(${PROJECT_NAME})

add_subdirectory(src)
```
---
# Libraries
```text
cmake_minimum_required(VERSION 3.23) 
project(<project name> VERSION 0.1 LANGUAGES C/CXX)

add_library(${PROJECT_NAME} STATIC/SHARED)
target_include_directories(${PROJECT_NAME} PUBLIC include)

add_subdirectory(src)
```
---
# Include libraries
```text
cmake_minimum_required(VERSION 3.23) 
project(<project name> VERSION 0.1 LANGUAGES C/CXX)

add_library(${PROJECT_NAME} INTERFACE)
target_include_directories(${PROJECT_NAME} INTERFACE include)
```