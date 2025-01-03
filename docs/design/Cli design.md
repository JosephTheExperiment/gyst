# Subcommands design (3 stages)
## 1. Input parsing (4 stages)
1. Subcommand name correction
2. Required input prompting
3. Validating user input
4. Validating project state
## 2. Runtime (2 parts) 
1. Run the subcommand and Showing the inner workings
NOTE: If an error have taken place in this stage reverse any changes made before the error
## 3. Compilation (2 sections)
1. What changed in the state of the project **OR** What error have occurred from other stages 
2. Suggesting what to do next 
<hr>
# Notes for good design
## Displaying information 
- Remember to NOT output too little to confuse and NOT too much to overwhelm
- Use color when possible
## Errors and conflicts
- Any conflict should be solved by prompting the user
NOTE: If you can fix an error prompt the user then implement it 
NOTE: If the cli isn't ran in tty display an error with the conflict
- When giving errors thay should have descriptions and suggestions to solve the error
