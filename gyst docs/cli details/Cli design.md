# Command design (3 stages)
## 1. Input parsing (4 stages)
1. Command name correction.
2. Required input prompting.
3. Validating user input.
4. Validating project state.
## 2. Runtime (1 part) 
1. Run the command, showing it's logs, but reverse any changes made before runtime, if an error had occurred.
## 3. Compilation (2 parts)
2. What changed in the state of the project, **or** what error have occurred from other stages.  
3. Suggests for what to do next. 
---
# Help massage headings
## Commands
4. Short description with additional details.
5. Usage: Required arguments and options.
6. Examples: Different uses, or ranging form simple to advanced.
7. Arguments: Categorized according to their alphabetical order.
8. Read more: Related topics, and commands, and a URL for documentation.
## Help Command massage
9. Name and short description.
10. Tutorial guide: => gyst topic tutorial. 
11. Usage: Options and command.
12. Commands (categorized according to their function): Names and short descriptions.
13. Read more: Important topics and a URL for documentation.
---
# Notes for commands design
## Displaying information 
- Remember to NOT output too little to confuse, and NOT too much to overwhelm.
- Use color when possible.
- Alter the output according to the user's preferences.  
## Errors and conflicts
- Any conflict should be solved by prompting the user, if you can fix an error; prompt the user then implement it, but if the cli isn't ran in tty display an error with the conflict.
- When giving errors they should have descriptions, and suggestions to solve the error.
## Working with the file system
- Any created file, folder, or added data to a file should be checked if it already exists, and working with the consideration of the result.     
## Getting around options 
- Split discerptions, arguments/flags, and options by knowing which of the available options are enabled.