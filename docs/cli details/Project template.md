# New project template
## Main
- .git **(if git is enabeld)** 
- README.md
- [[CmakePresets.json]]
- [[CMakeUserPresets.json_{conan, vcpkg}]]
- [[project_info.toml]]
- .gitignore **(if git is enabeld)** => {build directory, CMakeUserPresets.json}
- [[cmakelists.txt]]
- tests **(if testing is enabled)**
	- [[cmakelists.txt_testing]]
	- test.cpp
- build
	- conan **(if conan is enabled)**
- [[conanfile.py]] **(if conan is enabled)**
## Include libraries
- include
	- project name
		- project name.h/hpp
## Apps
- src
	- main.c/cpp
	- [[cmakelists.txt_src]]
## Libraries {static, shared}
- include
	- project name
		- project name.h/hpp
- src
	- project name.c/cpp
	- [[cmakelists.txt_src]]
<hr>
# Project organizers
## Modules layout
### Apps and static/shared libraries
- module name	 
	- module name.h/hpp
	- module name.c/cpp
	- [[cmakelists.txt_{module, section}]]
### Include libararies
- module name	 
	- module name.h/hpp
	- [[cmakelists.txt_{module, section}]]
## Sections layout
### All
- section name
	- [[cmakelists.txt_{module, section}]]