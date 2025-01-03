```toml
[project_info]
name = "" 
language = "" # {c, c++}
generator = "" # Cmake build system generator 
package_manager = "" # {conan, vcpkg}

[targets]
target_name = [
	{
		name = "",
		type = "", # {app, shared_lib, static_lib, include_lib}
		path = "" # Src dir for the target  
	}
]

[dependencies]
library_name = "library_version"

[flags]
cmake_config = "-B build --preset default"  
cmake_build = "" 
cmake_test = ""
conan_install = "--output-folder=build/conan --build=missing"
```