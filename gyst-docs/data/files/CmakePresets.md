```json
{
	"version": 4,
	"cmakeMinimumRequired": {
		"major": 3,
		"minor": 23,
		"patch": 0
	},
	"configurePresets": [
		{
			"name": "confg",
			"hidden": true,
			"cacheVariables": {
				"CMAKE_RUNTIME_OUTPUT_DIRECTORY": "${CMAKE_CURRENT_SOURCE_DIR}/build/target/bin/${CMAKE_BUILD_TYPE}",
				"CMAKE_LIBRARY_OUTPUT_DIRECTORY": "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}",
				"CMAKE_ARCHIVE_OUTPUT_DIRECTORY": "${CMAKE_CURRENT_SOURCE_DIR}/build/target/lib/${CMAKE_BUILD_TYPE}",
				"CMAKE_C_STANDARD": 17,
				"CMAKE_CXX_STANDARD": 20,
				"CMAKE_C_EXTENSIONS": "FALSE",
				"CMAKE_CXX_EXTENSIONS": "FALSE",
				"CMAKE_C_STANDARD_REQUIRED": "TRUE",
				"CMAKE_CXX_STANDARD_REQUIRED": "TRUE" 
			}
		},
		{
			"name": "default",
			"generator": "<generator>",
			"inherits" "confg",
			"cacheVariables": {
				"CMAKE_BUILD_TYPE": "Debug"
			}
		},
		{
			"name": "release",
			"generator": "<generator>",
			"inherits" "confg",
			"cacheVariables": {
				"CMAKE_BUILD_TYPE": "Release"
			}
		}
	]
}
```
