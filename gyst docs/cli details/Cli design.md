# Subcommands design (3 stages)
## 1. Input parsing (4 stages)
1. Subcommand name correction.
2. Required input prompting.
3. Validating user input.
4. Validating project state.
## 2. Runtime (1 part) 
1. Run the subcommand, showing it's inner workings, but reverse any changes made before if an error had happened.
## 3. Compilation (2 parts)
2. What changed in the state of the project, **or** what error have occurred from other stages.  
3. Suggesting what to do next. 
---
# Help massage
## Subcommands
- ### Content
	1. Short and detailed description.
	2. Read more => Related topics and a URL for documentation.
	3. Usage.
	4. Three examples => Different uses, or ranging form simple to advanced.
	5. Arguments and flags => categorized according to their function.
	6. Related topics and subcommands with explanations.
- ### Pattern
```txt
<short description>

<detailed description>

Read more: 
	<documentation URL>
	<topics>

Usage: gyst <subcommand> <required arguments and flags> [OPTIONS]

Examples:
	gyst <subcommand> <example arguments and flags>

Arguments:
argument <argument description>

Options:
-f, --flag <value> <flag description> (default: <default value>)
```
## Help subcommand massage
- ### Content
	1. Name and short description.
	2. Usage.
	3. Tutorial guide topic command: "gyst topic tutorial". 
	4. Subcommands categorized according to their function.
		1. Name and short description. 
- ### Pattern
```txt

```
---
# Notes for subcommands design
## Displaying information 
- Remember to NOT output too little to confuse, and NOT too much to overwhelm.
- Use color when possible.
## Errors and conflicts
- Any conflict should be solved by prompting the user, if you can fix an error; prompt the user then implement it, but if the cli isn't ran in tty display an error with the conflict.
- When giving errors they should have descriptions, and suggestions to solve the error.
## Working with the file system
- Any created file, folder, or added data to a file should be checked if it already exists, and working with the consideration of the result.     
## Getting around options 
- Split discerptions, args/flags, and options by knowing which package manager is used and for any other option later