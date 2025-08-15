# Work on
1. Add [[Command help example]]
2. Convert [[Common options]] to individual commands.
3. Replace cmake with bazel.
4. Add printing device.
	1. Create the templates needed for help.
	2. Create the functions needed for the low level templates.
	3. Create macros for the high and middle level templates.
5. Implement the cli.
6. Github repo.
7. Testing projects.
## commands
1. Help as a command to display help massages. 
2. Verbose and quiet commands for more or less output.
# Learn about
- Testing in c/c++.
- Man command
# Keep an eye on
- C++20 modules in cmake 3.28 https://cmake.org/cmake/help/latest/manual/cmake-cxxmodules.7.html.
- Developing the documentation for each command.
- Cut unnecessary features for the first release ( 0.5.0 ).
#  Release 1.0
- Add vcpkg for to gyst as the second package manager, but leave it's documentation for now.
- Package a library with: cpack, conan or vcpkg.
- Add more quality of life features.
# Release 1.5
- Machine friendly features.
- Have more customizability options.