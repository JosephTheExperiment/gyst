# Apps
```text
cmake_minimum_required(VERSION 3.23) 
project(project_name VERSION 0.1 LANGUAGES {C, CXX})

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/bin/${CMAKE_BUILD_TYPE}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}") 
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}")
set(CMAKE_{C, CXX}_STANDARD {17, 20})
set(CMAKE_{C, CXX}_STANDARD_REQUIRED On) 
set(CMAKE_{C, CXX}_EXTENSIONS Off)

add_executable(${PROJECT_NAME})

add_subdirectory(src)
```

<hr>
# Libraries {static, shared}
```text
cmake_minimum_required(VERSION 3.23) 
project(project_name VERSION 0.1 LANGUAGES {C, CXX})

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/bin/${CMAKE_BUILD_TYPE}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}") 
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}")
set(CMAKE_{C, CXX}_STANDARD {17, 20})
set(CMAKE_{C, CXX}_STANDARD_REQUIRED On) 
set(CMAKE_{C, CXX}_EXTENSIONS Off)

add_library(${PROJECT_NAME} {STATIC, SHARED})
target_include_directories(${PROJECT_NAME} PUBLIC include)

add_subdirectory(src)
```

<hr>
# Include libraries
```text
cmake_minimum_required(VERSION 3.23) 
project(project_name VERSION 0.1 LANGUAGES {C, CXX})

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/bin/${CMAKE_BUILD_TYPE}")
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}") 
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}")
set(CMAKE_{C, CXX}_STANDARD {17, 20})
set(CMAKE_{C, CXX}_STANDARD_REQUIRED On) 
set(CMAKE_{C, CXX}_EXTENSIONS Off)

add_library(${PROJECT_NAME} INTERFACE)
target_include_directories(${PROJECT_NAME} INTERFACE include)
```