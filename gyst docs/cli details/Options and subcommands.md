# Common options
- Options:
	- --color: {auto, always, never} => Controls color usage in the cli.
---
# Subcommands
## new
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
		- --test: name/version => Specifies a unit testing framework via name and version (conan required), adds tests, and enables testing.
		- --git: repo URL => Initializes git via: git init, or clones a repo if it's URL is given.
		- --conan: {txt, py} = txt => Adds [[Conanfile]] to install libraries via conan.
		- --vcpkg: baseline => Adds [[Vcpkg manifest]] to install libraries via vcpkg.
		
	- Additional information:
		- --info => Add more information via: version, description, and homepage. 
		- --version: version => Project version.
		- --description: description => Project description. 
		- --homepage: homepage => Project homepage.
## set
- Description => Sets a variable to some value.
- Flags:
	- -n, --name: {cmake flags, vcpkg flags, conan flags} => Variable name. 
	=> cmake flags: {cmake_config, cmake_build, cmake_test}
	conan flags: {conan_install, conan_common_options}
	vcpkg flags: {vcpkg_install, vcpkg_common_options}
	- --to: flags => New value.
## init
- Description => Initializes some optional features to an existing project.  
- Options:
	- --test: name/version => Specifies a unit testing framework via name and version (conan required), adds tests, and enables testing.
	- --git: repo URL => Initializes git via: git init, or clones a repo if it's URL is given.
	- --conan: {txt, py} = txt => Add [[Conanfile]] to install libraries via conan.
	- --vcpkg: baseline => Add [[Vcpkg manifest]] to install libraries via vcpkg.
## install
- Description => Installs libraries via the package manager specified.  
- Long description => Installs libraries via the package manager specified, and to search for libraries on conan visit conan center at: https://conan.io/center, and for vcpkg visit vcpkg packages at https://vcpkg.io/en/packages, or use the subcommand search in your package manager.
- Args:
	- Vcpkg
		- name\[port features]/version:triplet.. => One or more libraries names, port features, versions (not required), and triplet.
	- Conan
		- name/version:options.. => One or more libraries names, versions (required).
	=> Write "." to install all the libraries added.
- Options:
	- -stg, --settings: profile or triplet => If it wasn't specified with the library name and version this profile (for conan) or triplet (for vcpkg) will apply. 
	- --conan: flags => Conan flags.
	=> These flags will have priority over the flags in the project_info.toml file.
	- --vcpkg: flags => Vcpkg flags.
    => These flags will have priority over the flags in the project_info.toml file.
## uninstall
- Description => Uninstalls one or more libraries.
- Args: 
	- name.. => One or more libraries names.
## update
- Description => Updates one or more libraries.
- Args: 
	- name/version.. => One or more libraries names and versions (required for conan).
## add
- Description => Adds one or more source/header files, directories, modules, or libraries.
- Args:
	- -n, --name: name.. => One or more names for the type specified.
	- Libraries:
		- Vcpkg:
			- name\[port features]/version.. => One or more libraries names, port features, and versions (not required).
		- Conan:
			- name/version:options.. => One or more libraries names, versions (required).
- Flags:
	- -t, --type: {s, h, dir, mod, lib} => Specifies a type to add.
- Options:
	- -stg, --settings: profile/triplet => If it wasn't specified with the library name and version this profile or triplet will apply. (For libraries only)
	- --to: path = "./" => Specifies a directory to add to. (Doesn't effect libraries)
	=> If the directory specified doesn't exist then ask for confirmation to create it.
	- -f, --force => Add, or replace without asking for confirmation.
## delete
- Description => Deletes one or more source/header files, directories, modules, or libraries.
- Flags:	
	- -n, --name: name.. => Names for the type specified.
	- -t, --type: {s, h, dir, mod, lib} => Specifies a type to delete.
- Options:
	- --from: path = "./" => Specifies a directory to delete from. 
	=> If the directory specified have anything inside it thin ask for confirmation to delete it.
	- -f, --force => Delete without asking for confirmation.
## build
- Description => Builds a target.
- Long description => Builds a target, but if no input was given by default it will build the first target in the project_info.toml file.
- Flags:
	- -n, --name: target.. => One or more app or library name. 
	=> Write '.' to build all targets.
- Options:
	- --release => Builds in release mode. 
	- --debug => Builds in debug mode.
## run
- Description => Builds the main app target, and then runs it.
- Long description => Builds the main app target, and runs it, but if no input was given by default it will build, and run the first app target in the project_info.toml file.
- Flags:
	- -n, --name: target => App name. 
- Options:
	- --release => Runs in release mode.
	- --debug => Runs in debug mode.
	- -i, --input: args => Input for the app.
## topic
-  Description => Explains common topics and ideas used in gyst.
- Args:
	- topic => Topic name.
		- [[Linking and building options]]
- Options:
	- -l, --list => Lists all the topics available in an alphabetical order.