```toml
[project_info]
name = "" # Name
language = "" # {c, c++}
generator = "" # Cmake build system generator 
package_manager = "" # {conan, vcpkg, hunter}

[targets]
<target name> = [
	{
		name = "", # Name
		type = "", # {app, shared_lib, static_lib, include_lib}
		path = "" # Src dir for the target  
	}
]

[flags]
cmake_config = "-B build --preset default --fresh"  
cmake_build = "" 
cmake_test = ""
conan_install = "--output-folder=build/conan --build=missing"
conan_common_options = ""
vcpkg_install = ""
vcpkg_common_options = ""
```