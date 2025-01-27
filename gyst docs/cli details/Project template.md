# New project template
## Main
- .git (if git is enabled) 
- README.md
- [[CmakePresets.json]]
- [[CMakeUserPresets.json_{conan, vcpkg}]]
- [[project_info.toml]]
- .gitignore (if git is enabled) => {build directory, CMakeUserPresets.json}
- [[cmakelists.txt]]
- [[conanfile.{txt, py}]] (if conan is enabled)
- tests (if testing is enabled)
	- [[cmakelists.txt_testing]]
	- test.cpp
- build
	- conan (if conan is enabled)
- triplets (if vcpkg is enabled and triplets have been chosen)
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
---
# Project organizers
## Modules layout
### Apps and static/shared libraries
- module name	 
	- module name.h/hpp
	- module name.c/cpp
	- [[cmakelists.txt_{module, directory}]]
### Include libararies
- module name	 
	- module name.h/hpp
	- [[cmakelists.txt_{module, directory}]]
## Directory layout
### All
- folder name
	- [[cmakelists.txt_{module, directory}]]