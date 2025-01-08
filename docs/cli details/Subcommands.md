# Main
- Description => gyst description.
- Options
	- --no-color => Disables color.
---
# new
- Description => Creates new project, or adds a target to an existing project.
- Flags
	- -n, --name: name => Project name.
	- --lang: {c, c++} => Project language.
	=> This flag isn't required if you are adding to an existing project.
- Options
	- -t, --type: {app, shared_lib, static_lib, include_lib} = app => Project type.
	- --to: path = "./" => Specifies a directory for the project.
	- -G: build system => Build system generator for cmake; for help use: cmake --help. 
		
	- Optional features
		- --test: name/version => Specifies a unit testing framework, adds tests, and enables testing.
		- --git => Initializes git via: git init.
		- --conan: {txt, py} = txt => Add [[conanfile.txt]], or [[conanfile.py]] to install libraries via conan.
		- --vcpkg: baseline => Add [[Vcpkg manifest]] to install libraries via vcpkg.
		
	- Additional information
		- --info => Add more information via: version, description, and homepage. 
		- --version: version => Project version.
		- --description: description => Project description. 
		- --homepage: homepage => Project homepage.
---
# install
- Description => Installs libraries via conan  
- Long description => Installs libraries via conan using the the name, and version(conan only) of the library, and to search for libraries in conan visit conan center at : https://conan.io/center, and for vcpkg visit vcpkg packages at https://vcpkg.io/en/packages, or use the command: conan/vcpkg search.
- Args
	- name/version.. => Libraries names, and versions(conan only). 
- Options
	- --conan: flags => Conan flags.
	=> These flags will have priority over the flags in the project_info.toml file.
	- --vcpkg: flags => Vcpkg flags.
    => These flags will have priority over the flags in the project_info.toml file.
---
# uninstall
- Description => Uninstalls libraries.
- Args 
	- name.. => Libraries names.
---
# update
- Description => Updates libraries.
- Args 
	- name/version.. => Libraries names, and versions(conan only).
---
# add
- Description => Adds file, section, or module to some directory.
- Flags
	- -n, --name: name => file, section, or module name.  
	- -t, --type: {f, sec, mod} => Specifies a type to add.
- Options
	- --to: path = "./" => Specifies a directory to add to. 
	=> If the dir is dosen't exist ask for confirmation to create one.
	- -f, --force => Add, or replace without asking for confirmation.
---
# delete 
- Description => Deletes file, section, or module from some directory.
- Flags
	- -n, --name: name => file, section, or module name.
	- -t, --type: {f, sec, mod} => Type to delete.
- Options
	- --from: path = "./" => Specifies a directory to delete from. 
	=> If some section, or a module has files ask for confirmation to delete it.
	- -f, --force => Delete without asking for confirmation.
---
# set
- Description => Sets a variable to some value.
- Flags
	- -n, --name: {cmake flags, vcpkg flags, conan flags} => Variable. 
	cmake flags: {cmake_config, cmake_build, cmake_test}
	conan flags: {conan_install, conan_common_options}
	vcpkg flags: {vcpkg_install, vcpkg_common_options}
	- --to: flags => New value.
---
# init
- Description => Initializes some optional features to an existing project.  
- Options
	- --test: name/version => Specifies a unit testing framework, adds tests, and enables testing.
	- --git => Initializes git via: git init.
	- --conan: {txt, py} = txt => Add [[conanfile.txt]], or [[conanfile.py]] to install libraries via conan.
	- --vcpkg: baseline => Add [[Vcpkg manifest]] to install libraries via vcpkg.
---
# build 
- Description => Builds targets.
- Long description => Builds targets, but if no input was given by default it will build the first target in the project_info.toml file.
- Options
	- -n, --name: target name => App, or library name. 
	=> Write '.' to build all targets.
	- --release => Builds in release mode. 
	- --debug => Builds in debug mode.

<hr>
# run 
- Description => Builds the main app target, and runs it.
- Long description => Builds the main app target, and runs it, but if no input was given by default it will build, and run first app target in the project_info.toml file.
- Options
	- -n, --name: name => App name.  
	- --release => Run in release mode.
	- --debug => Run in debug mode.
	- -i, --input: args => Input for the app. 