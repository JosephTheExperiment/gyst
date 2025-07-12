# Description
Installs libraries via one of the available package managers: vcpkg, or conan.
# Details
To search for the available libraries: 
For conan visit: https://conan.io/center. For vcpkg visit: https://vcpkg.io/en/packages, or use the search subcommand in your package manager.
# Args:
## Conan
- name/version:{static, dynamic/shared} => One or more libraries names, version (not required). Write "." to install all the libraries added.
# Options:
- -stg, --settings: profile or triplet => If it wasn't specified with the library name and version this profile (for conan) or triplet (for vcpkg) will apply. 
- --conan: flags => Conan flags. These flags will have priority over the flags in the project_info.toml file.
- --vcpkg: flags => Vcpkg flags. These flags will have priority over the flags in the project_info.toml file.