# Work on
1. Convert [[Common options]] to individual commands.
2. Custom help massage, and macros for commands and help command.
	1. Help massage generator for commands. 
	2. Help massage generator for cli.
	3. Help command (just basic functionality).
3. Check on cmake files.
	1. Change the c and c++ standard.
	2. Change and check on the needed cache variables.
4. Implement the cli.
5. Github repo.
6. Testing projects.
## commands
1. Help as a command to display help massages and topics in addition to help flag. 
2. Verbose and quiet commands for more or less output.
3. Start command to initialize gyst.
# Learn about
- Environment variables.
- Testing in c/c++.
- Install in cmake.
# Keep an eye on
- C++20 modules in cmake 3.28 https://cmake.org/cmake/help/latest/manual/cmake-cxxmodules.7.html.
- Developing the documentation for each command.
- Cut unnecessary features for the first release.
# After release
- Add vcpkg for to gyst as the second package manager, but leave it's documentation for now.
- Package a library with: cpack, conan or vcpkg.