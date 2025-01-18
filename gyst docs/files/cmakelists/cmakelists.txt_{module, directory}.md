# Modules
```text
add_library(module_name MODULE)
target_link_libraries(${PROJECT_NAME} PRIVATE module_name)

target_sources(module_name 
	PRIVATE 
		module_name.{c, cpp}
	PRIVATE FILE_SET HEADERS
	FILES
		module_name.{h, hpp}
)
```
<hr>
# Directories
```text
target_sources(${PROJECT_NAME} 
	PRIVATE 
	PRIVATE FILE_SET HEADERS
	FILES
)
```
