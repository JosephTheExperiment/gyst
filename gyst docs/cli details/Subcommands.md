# Common options
- Options:
	- --color: {auto, always, never} => Control color usage in the cli.
---
# new
- Description => Creates a new project, or adds a target to an existing project.
- Flags:
	- -n, --name: name => Project name.
	- --lang: {c, c++} => Project language.
	=> This flag isn't required if you are adding to an existing project.
- Options:
	- -t, --type: {app, shared_lib, static_lib, include_lib} = app => Project type.
	- --to: path = "./" => Specifies a directory for the project.
	- -G: build system => Build system generator for cmake; for help use: cmake --help. 
		
	- Optional features:
		- --test: name/version(conan required) => Specifies a unit testing framework, adds tests, and enables testing.
		- --git: repo url => Initializes git via: git init, or clones a repo if it's url is given.
		- --conan: {txt, py} = txt => Adds [[conanfile.txt]], or [[conanfile.py]] to install libraries via conan.
		- --vcpkg: baseline => Adds [[Vcpkg manifest]] to install libraries via vcpkg.
		
	- Additional information:
		- --info => Add more information via: version, description, and homepage. 
		- --version: version => Project version.
		- --description: description => Project description. 
		- --homepage: homepage => Project homepage.
---
# install
- Description => Installs libraries via conan  
- Long description => Installs libraries via conan using the the name, and version(conan only) of the library, and to search for libraries in conan visit conan center at : https://conan.io/center, and for vcpkg visit vcpkg packages at https://vcpkg.io/en/packages, or use the command: conan/vcpkg search.
- Args:
	- name/version(conan required).. => Libraries names, and versions. 
- Options:
	- --conan: flags => Conan flags.
	=> These flags will have priority over the flags in the project_info.toml file.
	- --vcpkg: flags => Vcpkg flags.
    => These flags will have priority over the flags in the project_info.toml file.
---
# uninstall
- Description => Uninstalls libraries.
- Args: 
	- name.. => Libraries names.
---
# update
- Description => Updates libraries.
- Args: 
	- name/version(conan required).. => Libraries names, and versions.
---
# add
- Description => Adds a source/header file, a folder, a module, or a library.
- Args:
	- name, names/versions(conan required).. => A name(s) (, and maybe libraries versions) for the type specified.
- Flags:
	- -t, --type: {s, h, dir, mod, lib} => Specifies a type to add.
- Options:
	- --to: path = "./" => Specifies a directory to add to. 
	=> If the directory specified dosen't exist thin it will create it.
	- -f, --force => Add, or replace without asking for confirmation.
---
# delete
- Description => Deletes a source/header file, a folder, a module, or a library.
- Args:	
	- name, name/version(conan required).. => A name for the type specified.
- Flags:
	- -t, --type: {s, h, dir, mod, lib} => Specifies a type to delete.
- Options:
	- --to: path = "./" => Specifies a directory to delete from. 
	=> If the directory specified dosen't exist thin it will create it.
	- -f, --force => Delete without asking for confirmation.
---
# set
- Description => Sets a variable to some value.
- Flags:
	- -n, --name: {cmake flags, vcpkg flags, conan flags} => Variable name. 
	cmake flags: {cmake_config, cmake_build, cmake_test}
	conan flags: {conan_install, conan_common_options}
	vcpkg flags: {vcpkg_install, vcpkg_common_options}
	- --to: flags => New value.
---
# init
- Description => Initializes some optional features to an existing project.  
- Options:
	- --test: name/version(conan required) => Specifies a unit testing framework, adds tests, and enables testing.
	- --git: repo url => Initializes git via: git init, or clones a repo if it's url is given.
	- --conan: {txt, py} = txt => Add [[conanfile.txt]], or [[conanfile.py]] to install libraries via conan.
	- --vcpkg: baseline => Add [[Vcpkg manifest]] to install libraries via vcpkg.
---
# build 
- Description => Builds a target.
- Long description => Builds a target, but if no input was given by default it will build the first target in the project_info.toml file.
- Optional args:
	- -n, --name: target => App, or library name. 
	=> Write '.' to build all targets.
- Options:
	- --release => Builds in release mode. 
	- --debug => Builds in debug mode.

<hr>
# run 
- Description => Builds the main app target, and then runs it.
- Long description => Builds the main app target, and runs it, but if no input was given by default it will build, and run the first app target in the project_info.toml file.
- Optional args:
	- -n, --name: target => App name. 
- Options:
	- --release => Runs in release mode.
	- --debug => Runs in debug mode.
	- -i, --input: args => Input for the app. 