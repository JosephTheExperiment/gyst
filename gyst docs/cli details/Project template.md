# New project template
## Main
- [[CmakePresets]]
- [[CMakeUserPresets]]
- [[Project_info]]
- [[Cmakelists_main]]
- build
## Options
- Git:
	- README.md
	- .git  
	- .gitignore => Inside it: {./build, CMakeUserPresets.json}
- Testing:
	- tests 
		- [[Cmakelists_testing]]
		- test.cpp
- Triplets:
	- triplets
- Conan:
	- [[Conanfile]]
	- build
		- conan 
## Project types
- Include libraries:
	- include
		- \<project name>
			- \<project name>.h/hpp
- Apps:
	- src
		- main.c/cpp
		- [[Cmakelists_source]]
- Libraries:
	- include
		- \<project name>
			- \<project name>.h/hpp
	- src
		- \<project name>.c/cpp
		- [[Cmakelists_source]]
---
# Project organizers
## Modules layout
 - Apps and libraries:
	- \<module name>	 
		- \<module name>.h/hpp
		- \<module name>.c/cpp
		- [[Cmakelists_modules and directories]]
- Include libraries:
	- \<module name>	 
		- \<module name>.h/hpp
		- [[Cmakelists_modules and directories]]
## Directory layout
- \<folder name>
	- [[Cmakelists_modules and directories]]