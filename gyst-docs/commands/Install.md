# Description
Installs libraries via one of the available package managers: vcpkg, conan, or hunter.
# Details
To search for the available libraries: 
For conan visit conan center at: https://conan.io/center. For vcpkg visit vcpkg packages at https://vcpkg.io/en/packages, or use the subcommand search in your package manager.
# Args:
## Vcpkg
- name/version:triplet.. => One or more libraries names, version (not required), and triplet.
## Conan
- name/version:options.. => One or more libraries names, version (not required). Write "." to install all the libraries added.
# Options:
- -stg, --settings: profile or triplet => If it wasn't specified with the library name and version this profile (for conan) or triplet (for vcpkg) will apply. 
- --conan: flags => Conan flags. These flags will have priority over the flags in the project_info.toml file.
- --vcpkg: flags => Vcpkg flags. These flags will have priority over the flags in the project_info.toml file.