# Modules
```text
add_library(<module name> MODULE)
target_link_libraries(${PROJECT_NAME} PRIVATE <module name>)

target_sources(<module name> 
	PRIVATE 
		<module name>.c/cpp
	PRIVATE FILE_SET HEADERS
	FILES
		<module name>.h/hpp
)
```
---
# Directories
```text
target_sources(${PROJECT_NAME} 
	PRIVATE 
	PRIVATE FILE_SET HEADERS
	FILES
)
```
