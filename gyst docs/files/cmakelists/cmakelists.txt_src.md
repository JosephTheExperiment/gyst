# Apps
```text
target_sources(${PROJECT_NAME} 
	PRIVATE 
		main.{c, cpp}
	PRIVATE FILE_SET HEADERS
	FILES
)
```
<hr>
# Libraries {static, shared}
```text
target_sources(${PROJECT_NAME} 
	PRIVATE 
		${PROJECT_NAME}.{c, cpp}
	PRIVATE FILE_SET HEADERS
	FILES
)
```