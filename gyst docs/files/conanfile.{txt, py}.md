# conanfile.txt
```txt
[requires]
<library name>/<library version>

[tool_requires]
cmake/3.23.0

[generators]
CMakeDeps
CMakeToolchain
```
---
# conanfile.py
```python
from conan import ConanFile
	  
class projectName(ConanFile):
	settings = "os", "compiler", "build_type", "arch"
	generators = "CMakeToolchain", "CMakeDeps"
	  
	def requirements(self):
		self.requires("<library name>/<library version>")
	  
	def build_requirements(self):
		self.tool_requires("cmake/[>=3.23]")
```