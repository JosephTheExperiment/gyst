For a command to be executed five modules must play a role:
1. The purser => For getting and validating inputs.  
2. Commands => To execute the given command with the validated inputs.
3. System interactions => Any interactions between any of the modules should be done thru here. 
4. Features => Additional parts which dose not contribute to any of the other modules.
5. Data => Only for storing data.  

# The purser
Inputs will be declared; pursed, validated, and fixed if needed.
So after the main purser starts the first stage of input command pursing by identifying the command used the first stage completes with: 
## Command name correction
1. The cli should make sure that the names of each command is written in the right way without fixing it automatically.
2. Point 1 can be achieved by checking the wrong command name against a list of command names searching for the similar name, then prompt the user if that was the right command they had in mind. 
## Required input prompting
If a required input was missing, prompt the user with it's name, type, and description, then prompt for the missing input. 
## Validating user input.
The user may confuse the type of the input, so a simple checking, and prompting the user if the input is wrong will solve the issue if present.  
## Validating project state.
1. For example if the user entered a path with the right syntax, but simply doesn't exist.
2. The cli can't work with a non-existing path, so it will prompt the user with the option to create the desired path or not, additionally have the option to accept the prompt automatically.
3. The purser will only contain the shared parts between commands of the validating project state.
# Commands 
The commands module contains the logic to execute the specified command, and that includes the specific parts for the command from the validating project state stage, and the logic needed to execute the command. 
# System interactions
No module can interact with the system of user without giving it to system interactions module. Any interaction with the system of the user should be done with caution and with the agreement of the user, specially when the action needed to be done has a big effect on the system of the user.
#  Features
Aside from the bare bones experience of the cli; the features module contains every feature that can't be included by definition into any other module, in this case it's implementation should be contained in the features module.
# Data
