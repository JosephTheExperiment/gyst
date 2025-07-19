# Work on
1. Convert [[Common options]] to individual commands.
2. Add printing device.
	1. Create the templates needed for help.
	2. Create the functions needed for the low level templates.
	3. Create macros for the high and middle level templates.
3. Check on cmake files.
	1. Change the c and c++ standard.
	2. Change and check on the needed cache variables.
4. Implement the cli.
5. Github repo.
6. Testing projects.
## commands
1. Help as a command to display help massages. 
2. Verbose and quiet commands for more or less output.
# Learn about
- Testing in c/c++.
- Install in cmake.
# Keep an eye on
- C++20 modules in cmake 3.28 https://cmake.org/cmake/help/latest/manual/cmake-cxxmodules.7.html.
- Developing the documentation for each command.
- Cut unnecessary features for the first release ( 0.5.0 ).
# After release
- Add vcpkg for to gyst as the second package manager, but leave it's documentation for now.
- Package a library with: cpack, conan or vcpkg.
- Have more customizability options.
- Add more quality of life features.