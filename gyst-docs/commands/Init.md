# Description
Initializes some optional features to an existing project.  
# Options:
- --test: name/version => Specifies a unit testing framework via name and version (conan required), adds tests, and enables testing.
- --vcs: {git} = git => Initializes a version control repository.
- --conan: {txt, py} = txt => Adds [[Conanfile]] to install libraries via conan.
- --vcpkg: baseline => Adds [[Vcpkg manifest]] to install libraries via vcpkg, and if set changes the baseline to one given.