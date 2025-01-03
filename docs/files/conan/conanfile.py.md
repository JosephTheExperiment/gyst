```python
from conan import ConanFile
	  
class projectName(ConanFile):
	settings = "os", "compiler", "build_type", "arch"
	generators = "CMakeToolchain", "CMakeDeps"
	  
	def requirements(self):
		self.requires("library_name/library_version")
	  
	def build_requirements(self):
		self.tool_requires("cmake/[>=3.23]")
```
