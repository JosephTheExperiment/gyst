# Subcommands design (3 stages)
## 1. Input parsing (4 stages)
1. Subcommand name correction.
2. Required input prompting.
3. Validating user input.
4. Validating project state.
## 2. Runtime (1 part) 
1. Run the subcommand, and Showing the inner working, but reverse any changes made before an error if it happened.
## 3. Compilation (2 parts)
1. What changed in the state of the project, **or** what error have occurred from other stages.  
2. Suggesting what to do next. 
---
# Notes for good design
## Displaying information 
- Remember to NOT output too little to confuse, and NOT too much to overwhelm.
- Use color when possible.
## Errors and conflicts
- Any conflict should be solved by prompting the user, if you can fix an error; prompt the user then implement it, but if the cli isn't ran in tty display an error with the conflict.
- When giving errors they should have descriptions, and suggestions to solve the error.
## Working with the file system
- Any created file, folder, or added data to a file should be checked if it does already exists, and working with the consideration of the result.     
