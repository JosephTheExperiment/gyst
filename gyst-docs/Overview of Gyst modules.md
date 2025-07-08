# Overview
For a command to be executed five modules must play a role:
1. The parser => For getting and validating inputs.  
2. Commands => To execute the given command with the (partially) validated inputs.
3. Printing => Isn't it clear the printing module does printing.
4. System interactions => Any interactions between any of the modules should be done through here. 
5. Features => Additional parts which dose not contribute to any of the other modules.
6. Data => Only for storing data like files and text.  
# The parser
Inputs will be declared; parsed, validated, and fixed if needed.
So after the main parser starts the first stage of input command parsing by identifying the command used the first stage completes with: 
## Command name correction [[The parser]] 
## Required input prompting [[The parser]]
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
Does most (if not all) of the printing including partially the help output (sub-module); obviously while taking into account any considerations like environment variables, and the system in which the cli is running in.
# System interactions
No module can interact with the system of user without giving it to system interactions module. Any interaction with the system of the user should be done with caution and with the agreement of the user, specially when the action needed to be done has a big effect on the system of the user.
# Features
Aside from the bare bones experience of the cli; the features module contains every feature that can't be included by definition into any other module, in this case it's implementation should be contained in the features module.
# Data
Contains data like files, text or any other data that lacks logic of any kind to be considered as code.