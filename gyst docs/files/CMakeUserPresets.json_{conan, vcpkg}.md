# Conan
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
			"name": "default_conan",
			"inherits": "default",
			"toolchainFile": "conan/conan_toolchain.cmake"
		},
		{
			"name": "release_conan",
			"inherits": "release",
			"toolchainFile": "conan/conan_toolchain.cmake"
		}
	],
}
```
# Vcpkg
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
			"name": "default_vcpkg",
			"inherits": "default",
			"toolchainFile": "$env{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake"
		},
		{
			"name": "release_vcpkg",
			"inherits": "release",
			"toolchainFile": "$env{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake"
		}
	],
}
```
