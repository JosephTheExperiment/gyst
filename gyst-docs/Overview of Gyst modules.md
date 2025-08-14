# Overview
For a command to be executed five modules must play a role:
1. The parser => For getting and processing the inputs automatically.
2. Commands => To execute the given command with the (partially) validated inputs.
3. Printing => If it isn't clear the printing module does printing.
4. System interactions => Any interactions between any of the modules and the system of the user should be done through here. 
5. Features => Additional parts that is't necessary to any of the other modules.
6. Data => Only for storing data like files and text.
# The parser
Inputs will be declared; parsed, validated, and fixed if needed.
So after the main parser starts the first stage of input command parsing by identifying the command used the first stage completes with: 
## Command name correction 
1. The cli should make sure that the names of each command is written in the right way without fixing it automatically.
2. Point 1 can be achieved by checking the wrong command name against a list of commands names searching for the similar name, then prompt the user with the right name, and if that was the right command they had in mind. 
## Required input prompting
If a required input was missing, prompt the user with it's name, type, and description, then ask for the missing input.
## Validating user input
The user may confuse the type of the input, so a simple checking, and prompting the user if the input is wrong will solve the issue if present.  
## Validating project state
1. For example if the user entered a path with the right syntax, but simply doesn't exist.
2. The cli can't work with a non-existing path, so it will prompt the user with the option to create the desired path or not, additionally have the option to accept the prompt automatically.
3. The purser will only contain the shared parts between commands.
# Commands 
## Command runner 
For a command to run some repeated parts must be ran before the command it self:
1. The parser (with all it's stages)
2. The validating project state stage (for the command). 
3. Some additional parts from the features module.
## Command
Every command in the commands module contains the logic to execute the specified command, and that includes the specific parts for the command from the validating project state stage, and the logic needed to execute the command. 
# Printing
Does most (if not all) of the printing including partially the help (sub-module); obviously while taking into account considerations like environment variables, and the system in which the cli is running in.
# System interactions
No module can interact with the system of user without giving it to system interactions module. Any interaction with the system of the user should be done with caution and with the agreement of the user, specially when the action needed to be done has a big effect on the system of the user.
# Features
Aside from the bare bones experience of the cli; the features module contains every feature that can't be included by it's definition into any other module, in that case it's implementation should be contained in the features module.
# Data
Contains data like files, text, or any other thing that lacks logic.