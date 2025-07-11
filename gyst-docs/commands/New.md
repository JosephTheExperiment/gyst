# Description
Creates a new project, or adds a target to an existing project.
# Args:
- name => Project/target name.
- {c, c++} => Project language. This argument isn't required if you are adding to an existing project.
# Options:
- -t, --type: {app, shared_lib, static_lib, include_lib} = app => Project/target type.
- --dir: path = working directory => Specifies a directory for the project/target.
- -G: build system => Build system generator for cmake. For help use: cmake --help. 
## Optional features:
- --test: name/version => Specifies a unit testing framework via name and version (conan required), adds tests, and enables testing.
- --vcs: {git} = git => Initializes a version control repository.
- --conan: {txt, py} = txt => Adds [[Conanfile]] to install libraries via conan.
- --vcpkg: baseline => Adds [[Vcpkg manifest]] to install libraries via vcpkg.
## Project information:
- --info => Add more information via: version, description, and homepage by prompting the user. 
- --version: version => Project version.
- --description: description => Project description. 
- --homepage: homepage => Project homepage.